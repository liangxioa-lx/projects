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
        .invoke_handler(tauri::generate_handler![
            commands::listen::start_watching,
            commands::listen::stop_watching,
            commands::open_file::open_file,
            create::create_file::create_file,
            create::create_dir::create_dir,
            find::get_files::get_files_by_folder_path,
            find::get_driver_list::get_driver_list,
            remove::remove::remove,
            update::rename::rename,
            clipboard::copy::copy,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
