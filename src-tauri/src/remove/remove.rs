#[tauri::command]
pub fn remove(path: String) -> Result<(), String> {
    // 确保路径有效
    if !std::path::Path::new(&path).exists() {
        return Err(format!("File not found: {}", path));
    }
    // 判断是不是文件
    if std::path::Path::new(&path).is_file() {
        match std::fs::remove_file(&path) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Error deleting file: {}", err);
                Err(format!("Failed to remove file: {}", err))
            }
        }
    } else {
        match std::fs::remove_dir_all(&path) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Error deleting file: {}", err);
                Err(format!("Failed to remove file: {}", err))
            }
        }
    }
}
