mod manager;
mod r2;
mod typ;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init());

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    let builder = builder.plugin(tauri_plugin_clipboard::init());

    builder
        .invoke_handler(tauri::generate_handler![
            manager::preview_file,
            manager::get_file_details,
            r2::r2_ping,
            r2::r2_upload,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
