mod manager;
mod r2;
mod typ;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard::init())
        .invoke_handler(tauri::generate_handler![
            manager::preview_file,
            manager::get_file_details,
            r2::ping_bucket,
            r2::r2_upload,
            r2::cancel_upload,
            r2::get_upload_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
