use crate::typ::{File, UploadHistory, UploadSource, UploadStatus};
use aws_config::ConfigLoader;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client;
use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
use dashmap::DashMap;
use hyper::client::HttpConnector;
use hyper_proxy::ProxyConnector;
use mime_guess::from_path;
use once_cell::sync::Lazy;
use std::{
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncReadExt;
use tokio::sync::{mpsc, Semaphore};

// 键是 file_id，值是一个元组，包含一个 JoinHandle 和一个 Option<String>，用于存储 upload_id，upload_id 用于分段上传
static UPLOAD_TASKS: Lazy<
    DashMap<String, (tokio::task::JoinHandle<Result<(), String>>, Option<String>)>,
> = Lazy::new(DashMap::new);

static UPLOAD_TASKS_INFO: Lazy<DashMap<String, (Arc<R2Client>, String)>> = Lazy::new(DashMap::new);

#[tauri::command]
pub async fn r2_ping(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
) -> Result<(), String> {
    let client = R2Client::new(bucket_name, account_id, access_key, secret_key, None).await?;
    client.ping().await
}

#[tauri::command]
pub async fn r2_upload(
    app: AppHandle,
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    domain: Option<&str>,
    files: Vec<File>,
) -> Result<(), String> {
    let client =
        Arc::new(R2Client::new(bucket_name, account_id, access_key, secret_key, domain).await?);
    let semaphore = Arc::new(Semaphore::new(5));

    for file in files {
        let client = client.clone();
        let semaphore = semaphore.clone();
        let app = app.clone();
        let filename = file.remote_filename.clone();
        let file_id = file.id.clone();
        let file_id_for_emit = file_id.clone();
        let file_id_for_tasks = file_id.clone();
        let file_id_for_spawn = file_id.clone();

        emit_progress(
            &app,
            format!("{}/{}", client.domain, filename),
            file_id_for_emit,
            filename.clone(),
            UploadStatus::Uploading {
                progress: 0.0,
                bytes_uploaded: 0,
                total_bytes: 0,
                speed: 0.0,
            },
        );

        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let result = match &file.source {
                UploadSource::FilePath(path) => {
                    client
                        .stream_upload_file(&app, &path, &filename, &file_id_for_spawn)
                        .await
                }
                UploadSource::FileContent(content) => {
                    client.upload_content(content, &filename).await
                }
            };

            emit_progress(
                &app,
                format!("{}/{}", client.domain, filename),
                file_id,
                filename,
                match &result {
                    Ok(_) => UploadStatus::Success,
                    Err(e) => UploadStatus::Error {
                        message: e.to_string(),
                        code: "UPLOAD_ERROR".to_string(),
                    },
                },
            );

            result
        });

        UPLOAD_TASKS.insert(file_id_for_tasks, (handle, None));
    }

    Ok(())
}

pub fn emit_progress(
    app: &AppHandle,
    url: String,
    file_id: String,
    filename: String,
    status: UploadStatus,
) {
    let _ = app.emit(
        "upload-progress",
        UploadHistory {
            url,
            file_id,
            filename,
            status,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    );
}

#[tauri::command]
pub async fn r2_cancel_upload(file_id: String) -> Result<(), String> {
    // First get all the information we need
    let task_info = UPLOAD_TASKS
        .get(&file_id)
        .map(|entry| (entry.0.abort(), entry.1.clone()));

    let client_info = UPLOAD_TASKS_INFO
        .get(&file_id)
        .map(|entry| (entry.0.clone(), entry.1.clone()));

    // Then handle the cancellation
    if let Some((_, upload_id)) = task_info {
        if let Some(upload_id) = upload_id {
            if let Some((client, remote_filename)) = client_info {
                let _ = client
                    .abort_multipart_upload(&remote_filename, &upload_id)
                    .await;
            }
        }

        // Finally remove the entries
        UPLOAD_TASKS.remove(&file_id);
        UPLOAD_TASKS_INFO.remove(&file_id);
    }

    Ok(())
}

#[derive(Clone)]
pub struct R2Client {
    client: Client,
    bucket_name: String,
    domain: String,
}

impl R2Client {
    pub async fn new(
        bucket_name: &str,
        account_id: &str,
        access_key: &str,
        secret_key: &str,
        domain: Option<&str>,
    ) -> Result<Self, String> {
        let credentials = Credentials::new(access_key, secret_key, None, None, "R2Uploader");

        // Create HTTP client with or without proxy
        let http_client = match create_proxy_connector() {
            Some(proxy_connector) => HyperClientBuilder::new().build(proxy_connector),
            None => HyperClientBuilder::new().build(HttpConnector::new()),
        };

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
            domain: domain
                .unwrap_or(format!("https://{}.r2.cloudflarestorage.com", account_id).as_str())
                .to_string(),
        })
    }

    // 上传文件内容，一般是文字或图片，内容不会太大，直接上传，且不需要进度
    pub async fn upload_content(&self, content: &str, remote_filename: &str) -> Result<(), String> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(remote_filename)
            .body(content.as_bytes().to_vec().into())
            .content_type(
                from_path(remote_filename)
                    .first_or_octet_stream()
                    .to_string(),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // 创建多部分上传
    async fn create_multipart_upload(&self, remote_filename: &str) -> Result<String, String> {
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

    async fn complete_multipart_upload(
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
            .map_err(|e| {
                println!("完成多部分上传时遇到错误：{}", e.to_string());
                e.to_string()
            })?;
        Ok(())
    }

    async fn upload_part(
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

    // 流式上传文件
    async fn stream_upload_file(
        &self,
        app: &tauri::AppHandle,
        path: &str,
        remote_filename: &str,
        file_id: &str,
    ) -> Result<(), String> {
        const CHUNK_SIZE: usize = 5 * 1024 * 1024; // 5MB chunks

        // 读取文件信息
        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|e| e.to_string())?;
        let file_size = file.metadata().await.map_err(|e| e.to_string())?.len() as usize;

        // 如果文件小于 CHUNK_SIZE，直接上传
        if file_size < CHUNK_SIZE {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .await
                .map_err(|e| e.to_string())?;
            self.client
                .put_object()
                .bucket(&self.bucket_name)
                .key(remote_filename)
                .body(buffer.into())
                .content_type(
                    from_path(remote_filename)
                        .first_or_octet_stream()
                        .to_string(),
                )
                .send()
                .await
                .map_err(|e| e.to_string())?;
            return Ok(());
        }

        // 大文件，分块上传
        let upload_id = self.create_multipart_upload(remote_filename).await?;

        // Store upload_id in UPLOAD_TASKS
        if let Some(mut entry) = UPLOAD_TASKS.get_mut(file_id) {
            entry.1 = Some(upload_id.clone());
        }

        // Store client and remote_filename for potential abort
        UPLOAD_TASKS_INFO.insert(
            file_id.to_string(),
            (Arc::new(self.clone()), remote_filename.to_string()),
        );

        let mut part_number = 1;
        let mut completed_parts = Vec::new();
        let mut bytes_uploaded = 0;
        let start_time = SystemTime::now();

        loop {
            let remaining_bytes = file_size - bytes_uploaded;
            let buffer_size = if remaining_bytes > CHUNK_SIZE {
                CHUNK_SIZE
            } else {
                remaining_bytes
            };

            if buffer_size == 0 {
                break; // 文件已读取完毕
            }

            let mut buffer = vec![0; buffer_size];
            file.read_exact(&mut buffer)
                .await
                .map_err(|e| e.to_string())?;

            let part = self
                .upload_part(remote_filename, &upload_id, part_number, buffer.to_vec())
                .await?;

            completed_parts.push(part);
            bytes_uploaded += buffer_size;
            part_number += 1;

            // 更新进度
            let elapsed = SystemTime::now()
                .duration_since(start_time)
                .unwrap_or_default();
            let speed = bytes_uploaded as f64 / elapsed.as_secs_f64();
            emit_progress(
                app,
                format!("{}/{}", self.domain, remote_filename),
                file_id.to_string(),
                remote_filename.to_string(),
                UploadStatus::Uploading {
                    progress: bytes_uploaded as f64 / file_size as f64,
                    bytes_uploaded: bytes_uploaded as u64,
                    total_bytes: file_size as u64,
                    speed,
                },
            );
        }

        self.complete_multipart_upload(remote_filename, &upload_id, completed_parts)
            .await
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
    #[cfg(any(target_os = "ios", target_os = "android"))]
    return None;

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    match sysproxy::Sysproxy::get_system_proxy() {
        Ok(proxy) if !proxy.host.is_empty() && proxy.port > 0 && proxy.enable => {
            // Try to create proxy URI and connector
            let proxy_uri = format!("http://{}:{}", proxy.host, proxy.port)
                .parse()
                .ok()?;
            let proxy = hyper_proxy::Proxy::new(hyper_proxy::Intercept::All, proxy_uri);
            ProxyConnector::from_proxy(HttpConnector::new(), proxy).ok()
        }
        _ => None, // Return None if no proxy or error getting proxy
    }
}
