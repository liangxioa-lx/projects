#[tauri::command]
pub fn open_file(path: String) -> Result<(), String> {
    // 确保路径有效
    if !std::path::Path::new(&path).exists() {
        return Err(format!("File not found: {}", path));
    }

    // 尝试打开文件
    match open::that(&path) {
        Ok(()) => {
            println!("File opened successfully: {}", path);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            Err(format!("Failed to open file: {}", err))
        }
    }
}
