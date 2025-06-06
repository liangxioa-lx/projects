use std::path::PathBuf;

#[tauri::command]
pub fn create_dir(parent_path: String, dir_name: String) -> Result<(), String> {
    // 确保路径有效
    let parent_path = PathBuf::from(&parent_path);
    if !parent_path.exists() {
        return Err(format!("Dir not found: {}", parent_path.display()));
    }

    let mut new_path = parent_path.join(&dir_name);
    // 验证文件夹是否存在，如果存在则自动加1
    if new_path.exists() {
        let mut i = 1;
        loop {
            new_path = parent_path.join(format!("{}{}", dir_name, i));
            if !new_path.exists() {
                break;
            }
            i += 1;
        }
    }

    match std::fs::create_dir(&new_path) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Error creating directory: {}", err);
            Err(format!("Failed to create directory: {}", err))
        }
    }
}
