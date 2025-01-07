mod manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard::init())
        .invoke_handler(tauri::generate_handler![
            manager::r2_upload,
            manager::get_upload_status,
            manager::preview_file,
            manager::get_file_details,
            manager::ping_bucket,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
