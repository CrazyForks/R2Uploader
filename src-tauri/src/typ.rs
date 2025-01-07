use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum UploadStatus {
    Uploading {
        progress: f64,
        bytes_uploaded: u64,
        total_bytes: u64,
        speed: f64,
    },
    Success,
    Error {
        message: String,
        code: String,
    },
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadProgress {
    pub file_id: String,
    pub filename: String,
    pub status: UploadStatus,
    pub timestamp: u64,
}
