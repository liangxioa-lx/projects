#[tauri::command]
pub fn create_file(parent_path: String, file_name: String) -> Result<(), String> {
    // 确保路径有效
    if !std::path::Path::new(&parent_path).exists() {
        return Err(format!("Dir not found: {}", parent_path));
    }

    // 验证文件是否存在
    if std::path::Path::new(&(parent_path.clone() + "\\" + &file_name)).exists() {
        return Err(format!("File already exists: {}", file_name));
    }

    match std::fs::File::create(parent_path + "\\" + &file_name) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            Err(format!("Failed to open file: {}", err))
        }
    }
}
