use mime_guess::from_path;
use s3::{creds::Credentials, Bucket, Region};
use std::fs;

#[tauri::command]
pub async fn read_file_data(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read(file_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn r2_upload(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    file_path: &str,
    remote_file_name: &str,
) -> Result<(), String> {
    println!(
        "Uploading file to bucket: {}, account_id: {}, file_path: {}",
        bucket_name, account_id, file_path
    );
    let file_data = read_file_data(file_path).await?;
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

    let mime = from_path(file_path).first_or_octet_stream();
    let content_type = mime.as_ref();
    let response_data = bucket
        .put_object_with_content_type(remote_file_name, &file_data, content_type)
        .await
        .map_err(|e| e.to_string())?;

    if response_data.status_code() == 200 {
        Ok(())
    } else {
        Err(format!(
            "Failed to upload file to S3: {}",
            response_data.as_str().map_err(|e| e.to_string())?
        ))
    }
}
