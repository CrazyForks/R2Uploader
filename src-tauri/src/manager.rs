use aws_config::ConfigLoader;
use aws_sdk_s3::config::{Builder, Credentials, Region};
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

static UPLOAD_STATUS: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);
static UPLOAD_SEMAPHORE: Lazy<Arc<Semaphore>> = Lazy::new(|| Arc::new(Semaphore::new(16)));

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

pub fn get_proxy() -> Result<Proxy, String> {
    let proxy = sysproxy::Sysproxy::get_system_proxy()
        .map_err(|e| format!("can not get system proxy: {}", e))?;
    let proxy_uri = format!("http://{}:{}", proxy.host, proxy.port)
        .parse()
        .map_err(|e| format!("can not parse proxy uri: {}", e))?;
    let mut proxy = hyper_proxy::Proxy::new(hyper_proxy::Intercept::All, proxy_uri);
    Ok(proxy)
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
pub async fn r2_upload(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    files: Vec<File>,
) -> Result<(), String> {
    let credentials = Credentials::new(access_key, secret_key, None, None, "r2-uploader");

    let proxy = sysproxy::Sysproxy::get_system_proxy()
        .map_err(|e| format!("can not get system proxy: {}", e))?;
    let proxy_uri = format!("http://{}:{}", proxy.host, proxy.port)
        .parse()
        .map_err(|e| format!("can not parse proxy uri: {}", e))?;
    let proxy = hyper_proxy::Proxy::new(hyper_proxy::Intercept::All, proxy_uri);
    let connector = HttpConnector::new();
    let a_proxy = ProxyConnector::from_proxy(connector, proxy).unwrap();
    let client =
        aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder::new().build(a_proxy);

    let config = ConfigLoader::default()
        .region(Region::new("auto"))
        .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
        .credentials_provider(credentials)
        .http_client(client)
        .load()
        .await;

    let client = Client::new(&config);

    let bucket_name = bucket_name.to_string();
    let account_id = account_id.to_string();

    tokio::spawn(async move {
        let client = Arc::new(client);

        for file in files {
            let permit = UPLOAD_SEMAPHORE.clone().acquire_owned().await.unwrap();
            let client = client.clone();
            let bucket_name = bucket_name.clone();
            let account_id = account_id.clone();

            tokio::spawn(async move {
                let file_id = file.id.clone();
                UPLOAD_STATUS.insert(file_id.clone(), "uploading".to_string());

                let content_type = from_path(&file.remote_filename).first_or_octet_stream();
                let content_type = content_type.as_ref();

                let result: Result<(), String> = match file.source {
                    UploadSource::FilePath(path) => {
                        println!(
                            "Uploading file to bucket: {}, account_id: {}, file_path: {}",
                            &bucket_name, &account_id, path
                        );

                        let _metadata = tokio::fs::metadata(&path)
                            .await
                            .map_err(|e| e.to_string())?;

                        let body = aws_sdk_s3::primitives::ByteStream::from_path(&path)
                            .await
                            .map_err(|e| e.to_string())?;

                        client
                            .put_object()
                            .bucket(&bucket_name)
                            .key(&file.remote_filename)
                            .content_type(content_type)
                            .body(body)
                            .send()
                            .await
                            .map_err(|e| e.to_string())?;
                        Ok(())
                    }
                    UploadSource::FileContent(content) => {
                        println!(
                            "Uploading content to bucket: {}, account_id: {}",
                            bucket_name, account_id
                        );
                        let body =
                            aws_sdk_s3::primitives::ByteStream::from(content.as_bytes().to_vec());

                        client
                            .put_object()
                            .bucket(&bucket_name)
                            .key(&file.remote_filename)
                            .content_type(content_type)
                            .body(body)
                            .send()
                            .await
                            .map_err(|e| e.to_string())?;
                        Ok(())
                    }
                };

                match result {
                    Ok(_) => {
                        UPLOAD_STATUS.insert(file_id, "success".to_string());
                    }
                    Err(e) => {
                        UPLOAD_STATUS.insert(file_id, format!("failed: {}", e));
                    }
                }

                drop(permit);
                Ok::<(), String>(())
            });
        }
    });

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
