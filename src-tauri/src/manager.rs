use mime_guess::from_path;
use s3::{creds::Credentials, Bucket, Region};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // 统一使用小驼峰命名法
pub enum UploadSource {
    FilePath(String),
    FileContent(String),
}

#[tauri::command]
pub async fn read_file_data(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read(file_path).map_err(|e| e.to_string())
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
    source: UploadSource,
    remote_file_name: &str,
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

    let content_type = from_path(remote_file_name).first_or_octet_stream();
    let content_type = content_type.as_ref();

    match source {
        UploadSource::FilePath(path) => {
            println!(
                "Uploading file to bucket: {}, account_id: {}, file_path: {}",
                bucket_name, account_id, path
            );

            let metadata = tokio::fs::metadata(&path)
                .await
                .map_err(|e| e.to_string())?;
            if metadata.len() < 1 * 1024 * 1024 {
                let file_data = read_file_data(&path).await?;
                bucket
                    .put_object_with_content_type(remote_file_name, &file_data, content_type)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                let mut file = tokio::fs::File::open(&path)
                    .await
                    .map_err(|e| e.to_string())?;
                bucket
                    .put_object_stream_with_content_type(&mut file, remote_file_name, content_type)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        UploadSource::FileContent(content) => {
            println!(
                "Uploading content to bucket: {}, account_id: {}",
                bucket_name, account_id
            );
            let file_data = content.as_bytes().to_vec();
            bucket
                .put_object_with_content_type(remote_file_name, &file_data, content_type)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
