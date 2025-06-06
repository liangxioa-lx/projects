#[tauri::command]
pub fn rename(path: String, new_name: String) -> Result<(), String> {
    // 确保路径有效
    if !std::path::Path::new(&path).exists() {
        return Err(format!("File not found: {}", path));
    }
    println!("{}", path);

    let new_path = std::path::Path::new(&path).parent().unwrap().join(new_name);

    if new_path.exists() {
        let mut i = 1;
        loop {
            let new_name = format!(
                "{}{}",
                std::path::Path::new(&path)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                i
            );
            let new_path = std::path::Path::new(&path).parent().unwrap().join(new_name);
            if !new_path.exists() {
                break;
            }
            i += 1;
        }
    }
    match std::fs::rename(&path, &new_path) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Errorrenaming file: {}", err);
            Err(format!("Failed to remove file: {}", err))
        }
    }
}
