use dashmap::DashMap;
use mime_guess::from_path;
use once_cell::sync::Lazy;
use s3::{creds::Credentials, Bucket, Region};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Semaphore;

static UPLOAD_STATUS: Lazy<DashMap<String, String>> = Lazy::new(DashMap::new);
static UPLOAD_SEMAPHORE: Lazy<Arc<Semaphore>> = Lazy::new(|| Arc::new(Semaphore::new(5))); // 限制同时上传 5 个文件

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // 统一使用小驼峰命名法
pub enum UploadSource {
    FilePath(String),
    FileContent(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // 统一使用小驼峰命名法
pub struct File {
    pub id: String,
    pub source: UploadSource,
    pub remote_filename: String,
    pub remote_filename_prefix: String,
}

pub fn get_proxy() -> Result<reqwest::Proxy, String> {
    let proxy = sysproxy::Sysproxy::get_system_proxy()
        .map_err(|e| format!("can not get system proxy: {}", e.to_string()))?;
    let proxy = reqwest::Proxy::https(format!("http://{}:{}", proxy.host, proxy.port).as_str())
        .map_err(|e| format!("can not create proxy: {}", e.to_string()))?;
    Ok(proxy)
}

#[tauri::command]
pub async fn r2_upload(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    files: Vec<File>,
) -> Result<(), String> {
    let bucket = Bucket::new(
        bucket_name,
        Region::R2 {
            account_id: account_id.to_string(),
        },
        Credentials::new(Some(access_key), Some(secret_key), None, None, None)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?
    .with_path_style();

    bucket.set_proxy(get_proxy()?).map_err(|e| e.to_string())?;

    // 立即返回，在后台处理上传
    let bucket_name = bucket_name.to_string();
    let account_id = account_id.to_string();

    tokio::spawn(async move {
        let bucket = Arc::new(bucket);

        for file in files {
            let permit = UPLOAD_SEMAPHORE.clone().acquire_owned().await.map_err(|e| e.to_string())?;
            let bucket = bucket.clone();
            let bucket_name = bucket_name.clone();
            let account_id = account_id.clone();

            tokio::spawn(async move {
                let file_id = file.id.clone();
                UPLOAD_STATUS.insert(file_id.clone(), "uploading".to_string());

                let remote_file_name =
                    format!("{}{}", file.remote_filename_prefix, file.remote_filename);
                let content_type = from_path(&remote_file_name).first_or_octet_stream();
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

                        let mut file = tokio::fs::File::open(&path)
                            .await
                            .map_err(|e| e.to_string())?;
                        bucket
                            .put_object_stream_with_content_type(
                                &mut file,
                                &remote_file_name,
                                content_type,
                            )
                            .await
                            .map_err(|e| e.to_string())?;
                        Ok(())
                    }
                    UploadSource::FileContent(content) => {
                        println!(
                            "Uploading content to bucket: {}, account_id: {}",
                            bucket_name, account_id
                        );
                        let file_data = content.as_bytes().to_vec();
                        bucket
                            .put_object_with_content_type(
                                &remote_file_name,
                                &file_data,
                                content_type,
                            )
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

    // 收集所有状态
    for entry in UPLOAD_STATUS.iter() {
        let file_id = entry.key().clone();
        let status = entry.value().clone();

        // 如果状态为 success 则标记删除
        if status == "success" {
            to_remove.push(file_id.clone());
        }
        status_map.insert(file_id, status);
    }

    // 迭代结束后统一删除
    for file_id in to_remove {
        UPLOAD_STATUS.remove(&file_id);
    }

    Ok(status_map)
}
