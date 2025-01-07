use crate::typ::{File, UploadSource};
use aws_config::ConfigLoader;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client;
use base64::engine::general_purpose;
use base64::Engine;
use dashmap::DashMap;
use hyper::client::HttpConnector;
use hyper_proxy::{Proxy, ProxyConnector};
use mime_guess::from_path;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Arc};
use tokio::io::AsyncReadExt;
use tokio::sync::Semaphore;
use uuid::Uuid;

static UPLOAD_STATUS: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);

#[tauri::command]
pub async fn ping_bucket(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
) -> Result<(), String> {
    let client = R2Client::new(bucket_name, account_id, access_key, secret_key).await?;
    client.ping().await
}

#[tauri::command]
pub async fn r2_upload(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    files: Vec<File>,
) -> Result<(), String> {
    let client = Arc::new(R2Client::new(bucket_name, account_id, access_key, secret_key).await?);
    let semaphore = Arc::new(Semaphore::new(5));

    let mut handles = vec![];
    for file in files {
        let client = client.clone();
        let semaphore = semaphore.clone();
        let task_id = Uuid::new_v4().to_string();
        UPLOAD_STATUS.insert(task_id.clone(), "uploading".to_string());

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let result = match &file.source {
                UploadSource::FilePath(path) => {
                    client
                        .stream_upload_file(&path, &file.remote_filename, &task_id, |id| {
                            UPLOAD_STATUS
                                .get(id)
                                .map_or(false, |status| status.as_str() == "cancelled")
                        })
                        .await
                }
                UploadSource::FileContent(content) => {
                    let decoded = general_purpose::STANDARD
                        .decode(content)
                        .map_err(|e| e.to_string())?;
                    let content = String::from_utf8(decoded).map_err(|e| e.to_string())?;
                    client.upload_content(&content, &file.remote_filename).await
                }
            };

            match result {
                Ok(_) => UPLOAD_STATUS.insert(task_id, "success".to_string()),
                Err(e) => UPLOAD_STATUS.insert(task_id, format!("error: {}", e)),
            };
            Ok::<(), String>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.map_err(|e| e.to_string())??;
    }

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
pub async fn get_upload_status() -> Result<HashMap<String, String>, String> {
    let mut status_map = HashMap::new();
    let mut to_remove = Vec::new();

    for entry in UPLOAD_STATUS.iter() {
        let file_id = entry.key().clone();
        let status = entry.value().clone();

        if status == "success" || status.starts_with("error:") || status == "completed" {
            to_remove.push(file_id.clone());
        }
        status_map.insert(file_id, status);
    }

    for file_id in to_remove {
        UPLOAD_STATUS.remove(&file_id);
    }

    Ok(status_map)
}

pub struct R2Client {
    client: Client,
    bucket_name: String,
    account_id: String,
}

impl R2Client {
    pub async fn new(
        bucket_name: &str,
        account_id: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self, String> {
        let credentials = Credentials::new(access_key, secret_key, None, None, "r2-uploader");
        let proxy_connector = create_proxy_connector();
        let http_client = aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder::new()
            .build(proxy_connector.unwrap());

        let config = ConfigLoader::default()
            .region(Region::new("auto"))
            .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
            .credentials_provider(credentials)
            .http_client(http_client)
            .load()
            .await;

        Ok(Self {
            client: Client::new(&config),
            bucket_name: bucket_name.to_string(),
            account_id: account_id.to_string(),
        })
    }

    pub async fn upload_content(&self, content: &str, remote_filename: &str) -> Result<(), String> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .content_type(from_path(remote_filename).first_or_octet_stream().as_ref())
            .body(aws_sdk_s3::primitives::ByteStream::from(
                content.as_bytes().to_vec(),
            ))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn create_multipart_upload(&self, remote_filename: &str) -> Result<String, String> {
        self.client
            .create_multipart_upload()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .content_type(from_path(remote_filename).first_or_octet_stream().as_ref())
            .send()
            .await
            .map_err(|e| e.to_string())?
            .upload_id()
            .ok_or_else(|| "Failed to get upload ID".to_string())
            .map(|id| id.to_string())
    }

    pub async fn abort_multipart_upload(
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

    pub async fn complete_multipart_upload(
        &self,
        remote_filename: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<(), String> {
        self.client
            .complete_multipart_upload()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .upload_id(upload_id)
            .multipart_upload(
                CompletedMultipartUpload::builder()
                    .set_parts(Some(parts))
                    .build(),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn upload_part(
        &self,
        remote_filename: &str,
        upload_id: &str,
        part_number: i32,
        body: Vec<u8>,
    ) -> Result<CompletedPart, String> {
        self.client
            .upload_part()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .upload_id(upload_id)
            .part_number(part_number)
            .body(aws_sdk_s3::primitives::ByteStream::from(body))
            .send()
            .await
            .map_err(|e| e.to_string())?
            .e_tag()
            .ok_or_else(|| "Failed to get ETag".to_string())
            .map(|e_tag| {
                CompletedPart::builder()
                    .e_tag(e_tag)
                    .part_number(part_number)
                    .build()
            })
    }

    pub async fn stream_upload_file(
        &self,
        path: &str,
        remote_filename: &str,
        task_id: &str,
        is_cancelled: impl Fn(&str) -> bool,
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
            if is_cancelled(task_id) {
                self.abort_multipart_upload(remote_filename, &upload_id)
                    .await?;
                return Err("Upload cancelled".to_string());
            }

            let n = file
                .read_buf(&mut buffer)
                .await
                .map_err(|e| e.to_string())?;
            if n == 0 && buffer.is_empty() {
                break;
            }

            if buffer.len() >= CHUNK_SIZE || (n == 0 && !buffer.is_empty()) {
                let upload_data = std::mem::replace(&mut buffer, Vec::with_capacity(CHUNK_SIZE));
                let completed_part = self
                    .upload_part(remote_filename, &upload_id, part_number, upload_data)
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

    pub async fn ping(&self) -> Result<(), String> {
        self.client
            .head_bucket()
            .bucket(&self.bucket_name)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn create_proxy_connector() -> Option<ProxyConnector<HttpConnector>> {
    let proxy = std::env::var("HTTP_PROXY")
        .or_else(|_| std::env::var("http_proxy"))
        .ok();

    match proxy {
        Some(proxy_url) => {
            let proxy = Proxy::new(hyper_proxy::Intercept::All, proxy_url.parse().unwrap());
            Some(ProxyConnector::from_proxy(HttpConnector::new(), proxy).unwrap())
        }
        None => None,
    }
}
