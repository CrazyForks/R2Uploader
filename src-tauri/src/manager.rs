use aws_config::ConfigLoader;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::types::CompletedPart;
use aws_sdk_s3::Client;
use base64::{engine::general_purpose, Engine};
use dashmap::DashMap;
use hyper::client::HttpConnector;
use hyper_proxy::{Proxy, ProxyConnector};
use mime_guess::from_path;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Semaphore;
use uuid::Uuid;

struct R2Client {
    client: Client,
    bucket_name: String,
    account_id: String,
}

impl R2Client {
    async fn new(
        bucket_name: &str,
        account_id: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self, String> {
        let credentials = Credentials::new(access_key, secret_key, None, None, "r2-uploader");

        let proxy_connector = create_proxy_connector()?;
        let http_client = aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder::new()
            .build(proxy_connector);

        let config = ConfigLoader::default()
            .region(Region::new("auto"))
            .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
            .credentials_provider(credentials)
            .http_client(http_client)
            .load()
            .await;

        let client = Client::new(&config);

        Ok(Self {
            client,
            bucket_name: bucket_name.to_string(),
            account_id: account_id.to_string(),
        })
    }

    async fn upload_content(&self, content: &str, remote_filename: &str) -> Result<(), String> {
        let content_type = from_path(remote_filename).first_or_octet_stream();
        let content_type = content_type.as_ref();

        let body = aws_sdk_s3::primitives::ByteStream::from(content.as_bytes().to_vec());

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .content_type(content_type)
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn create_multipart_upload(&self, remote_filename: &str) -> Result<String, String> {
        let content_type = from_path(remote_filename).first_or_octet_stream();
        let content_type = content_type.as_ref();

        let result = self
            .client
            .create_multipart_upload()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        result
            .upload_id()
            .ok_or_else(|| "Failed to get upload ID".to_string())
            .map(|id| id.to_string())
    }

    async fn abort_multipart_upload(
        &self,
        remote_filename: &str,
        upload_id: &str,
    ) -> Result<(), String> {
        self.client
            .abort_multipart_upload()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .upload_id(upload_id)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn complete_multipart_upload(
        &self,
        remote_filename: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<(), String> {
        let completed_multipart_upload = aws_sdk_s3::types::CompletedMultipartUpload::builder()
            .set_parts(Some(parts))
            .build();

        self.client
            .complete_multipart_upload()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .upload_id(upload_id)
            .multipart_upload(completed_multipart_upload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn upload_part(
        &self,
        remote_filename: &str,
        upload_id: &str,
        part_number: i32,
        body: Vec<u8>,
    ) -> Result<CompletedPart, String> {
        let result = self
            .client
            .upload_part()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .upload_id(upload_id)
            .part_number(part_number)
            .body(aws_sdk_s3::primitives::ByteStream::from(body))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let e_tag = result
            .e_tag()
            .ok_or_else(|| "Failed to get ETag".to_string())?
            .to_string();

        Ok(CompletedPart::builder()
            .e_tag(e_tag)
            .part_number(part_number)
            .build())
    }

    async fn stream_upload_file(
        &self,
        path: &str,
        remote_filename: &str,
        task_id: &str,
    ) -> Result<(), String> {
        const CHUNK_SIZE: usize = 5 * 1024 * 1024; // 5MB chunks

        let upload_id = self.create_multipart_upload(remote_filename).await?;

        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|e| e.to_string())?;

        let mut part_number = 1;
        let mut completed_parts = Vec::new();
        let mut buffer = Vec::with_capacity(CHUNK_SIZE);

        loop {
            if UPLOAD_STATUS
                .get(task_id)
                .map_or(false, |status| status.as_str() == "cancelled")
            {
                // 如果上传被取消，中止多部分上传
                self.abort_multipart_upload(remote_filename, &upload_id)
                    .await?;
                return Err("Upload cancelled".to_string());
            }

            use tokio::io::AsyncReadExt;
            let n = file
                .read_buf(&mut buffer)
                .await
                .map_err(|e| e.to_string())?;

            if n == 0 {
                break;
            }

            if buffer.len() >= CHUNK_SIZE || n == 0 {
                let chunk = buffer.split_off(0);
                let completed_part = self
                    .upload_part(remote_filename, &upload_id, part_number, chunk)
                    .await?;

                completed_parts.push(completed_part);
                part_number += 1;
            }
        }

        if !completed_parts.is_empty() {
            self.complete_multipart_upload(remote_filename, &upload_id, completed_parts)
                .await?;
        }

        Ok(())
    }
}

fn create_proxy_connector() -> Result<ProxyConnector<HttpConnector>, String> {
    let proxy = sysproxy::Sysproxy::get_system_proxy()
        .map_err(|e| format!("Failed to get system proxy: {}", e))?;

    let proxy_uri = format!("http://{}:{}", proxy.host, proxy.port)
        .parse()
        .map_err(|e| format!("Failed to parse proxy URI: {}", e))?;

    let proxy = Proxy::new(hyper_proxy::Intercept::All, proxy_uri);
    let connector = HttpConnector::new();

    ProxyConnector::from_proxy(connector, proxy)
        .map_err(|e| format!("Failed to create proxy connector: {}", e))
}

static UPLOAD_STATUS: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UploadSource {
    FilePath(String),
    FileContent(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    pub source: UploadSource,
    pub remote_filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDetail {
    pub id: String,
    pub path: String,
    pub relative_path: String,
    pub is_dir: bool,
}

async fn get_file_details_internal(
    path: String,
    base_path: &str,
) -> Result<Vec<FileDetail>, String> {
    let metadata = tokio::fs::metadata(&path)
        .await
        .map_err(|e| format!("无法获取文件元数据：{}", e))?;

    let mut result = Vec::new();
    if metadata.is_dir() {
        let mut entries = tokio::fs::read_dir(&path)
            .await
            .map_err(|e| format!("无法读取目录：{}", e))?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let child_path = entry.path().to_string_lossy().to_string();
            let child_details = Box::pin(get_file_details_internal(child_path, base_path)).await?;
            result.extend(child_details);
        }
    } else {
        let relative_path = path
            .strip_prefix(base_path)
            .map(|p| p.to_string())
            .unwrap_or_else(|| path.clone());

        result.push(FileDetail {
            id: Uuid::new_v4().to_string(),
            path: path.clone(),
            relative_path,
            is_dir: false,
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_file_details(path: String) -> Result<Vec<FileDetail>, String> {
    Box::pin(async move {
        let base_path = std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "".to_string());

        get_file_details_internal(path, &base_path).await
    })
    .await
}

#[tauri::command]
pub async fn preview_file(path: String) -> Result<String, String> {
    let metadata = tokio::fs::metadata(&path)
        .await
        .map_err(|e| format!("无法获取文件元数据：{}", e))?;

    if metadata.len() > 10 * 1024 * 1024 {
        return Err("文件大小超过 10MB 限制".to_string());
    }

    if path.ends_with(".txt") {
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| format!("无法读取文件：{}", e))?;
        let lines: Vec<&str> = content.lines().take(100).collect();
        return Ok(lines.join("\n"));
    }

    if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") {
        let data = tokio::fs::read(&path)
            .await
            .map_err(|e| format!("无法读取图片文件：{}", e))?;
        let base64 = general_purpose::STANDARD.encode(data);
        return Ok(format!(
            "data:image/{};base64,{}",
            path.split('.').last().unwrap_or("png"),
            base64
        ));
    }

    Err("不支持的文件类型".to_string())
}

#[tauri::command]
pub async fn ping_bucket(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
) -> Result<(), String> {
    let credentials = Credentials::new(access_key, secret_key, None, None, "r2-uploader");

    let config = ConfigLoader::default()
        .region(Region::new("auto"))
        .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
        .credentials_provider(credentials)
        .load()
        .await;

    let client = Client::new(&config);

    let result = client
        .list_objects_v2()
        .bucket(bucket_name)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    println!("Bucket contents: {:?}", result.contents);

    Ok(())
}

#[tauri::command]
pub async fn cancel_upload(task_id: String) -> Result<(), String> {
    if let Some(mut status) = UPLOAD_STATUS.get_mut(&task_id) {
        *status = "cancelled".to_string();
        Ok(())
    } else {
        Err("Upload task not found".to_string())
    }
}

#[tauri::command]
pub async fn r2_upload(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    files: Vec<File>,
) -> Result<(), String> {
    let client = R2Client::new(bucket_name, account_id, access_key, secret_key).await?;

    for file in files {
        // 使用文件的id作为上传任务的id
        UPLOAD_STATUS.insert(file.id.clone(), "uploading".to_string());

        let result = match &file.source {
            UploadSource::FilePath(path) => {
                println!(
                    "Uploading file to bucket: {}, account_id: {}, file_path: {}",
                    client.bucket_name, client.account_id, path
                );
                client
                    .stream_upload_file(path, &file.remote_filename, &file.id)
                    .await
            }
            UploadSource::FileContent(content) => {
                println!(
                    "Uploading content to bucket: {}, account_id: {}",
                    client.bucket_name, client.account_id
                );
                client.upload_content(content, &file.remote_filename).await
            }
        };

        // 根据上传结果更新状态
        match result {
            Ok(_) => {
                if UPLOAD_STATUS
                    .get(&file.id)
                    .map_or(false, |status| status.as_str() != "cancelled")
                {
                    UPLOAD_STATUS.insert(file.id.clone(), "completed".to_string());
                }
            }
            Err(e) => {
                if UPLOAD_STATUS
                    .get(&file.id)
                    .map_or(false, |status| status.as_str() != "cancelled")
                {
                    UPLOAD_STATUS.insert(file.id.clone(), format!("error: {}", e));
                }
                return Err(e);
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_upload_status() -> Result<HashMap<String, String>, String> {
    let mut status_map = HashMap::new();
    let mut to_remove = Vec::new();

    for entry in UPLOAD_STATUS.iter() {
        let file_id = entry.key().clone();
        let status = entry.value().clone();

        if status == "success" {
            to_remove.push(file_id.clone());
        }
        status_map.insert(file_id, status);
    }

    for file_id in to_remove {
        UPLOAD_STATUS.remove(&file_id);
    }

    Ok(status_map)
}
