mod commands;
mod create;
mod find;
mod remove;
mod update;
mod clipboard;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            create::create_file::create_file,
            create::create_dir::create_dir,
            clipboard::copy::copy,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
