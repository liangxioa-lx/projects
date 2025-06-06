extern crate systemstat;

use chrono::prelude::*;
use serde::Serialize;
use std::fs::DirEntry;
use std::os::windows::fs::MetadataExt;
use std::time::UNIX_EPOCH;
use std::{fs, path::Path};

#[derive(Serialize)]
pub struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
    file_size: Option<u64>,
    created_at: Option<String>,
    modified_at: Option<String>,
    suffix: Option<String>,
    is_hide: bool,
    is_readonly: bool,
}

#[tauri::command]
pub fn get_files_by_folder_path(path: &str) -> Result<Vec<FileEntry>, String> {
    let path = Path::new(path);

    // 合并路径检查逻辑
    if !path.exists() || !path.is_dir() {
        return Err("Invalid directory path".into());
    }

    let entries = fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| get_file_entry(entry.ok()?))
        .collect();

    Ok(entries)
}

// 获取文件时间
fn get_file_time(date_time: std::time::SystemTime) -> Option<String> {
    date_time.duration_since(UNIX_EPOCH).ok().and_then(|dur| {
        DateTime::<Utc>::from_timestamp(dur.as_secs() as i64, dur.subsec_nanos())
            .map(|dt| dt.to_rfc3339())
    })
}

// 获取文件信息
fn get_file_entry(entry: DirEntry) -> Option<FileEntry> {
    let path = entry.path();
    let metadata = path.metadata().ok()?;

    let created_at = get_file_time(metadata.created().expect("时间解析错误"));

    let modified_at = get_file_time(metadata.modified().expect("时间解析错误"));

    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;
    const FILE_ATTRIBUTE_READONLY: u32 = 0x00000001;

    let metadata_attr = metadata.file_attributes();
    Some(FileEntry {
        name: path.file_name()?.to_str()?.to_string(),
        path: path.to_str()?.to_string(),
        is_dir: path.is_dir(),
        file_size: if path.is_file() {
            Some(metadata.len())
        } else {
            None
        },
        created_at,
        modified_at,
        suffix: if path.is_file() {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_string())
        } else {
            None
        },
        is_hide: metadata_attr & FILE_ATTRIBUTE_HIDDEN != 0,
        is_readonly: metadata_attr & FILE_ATTRIBUTE_READONLY != 0,
    })
}
