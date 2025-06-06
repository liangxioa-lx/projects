use notify::{RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use uuid::Uuid;

// 全局状态来存储监听器
static WATCHERS: Lazy<Mutex<HashMap<Uuid, Arc<Mutex<notify::RecommendedWatcher>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[tauri::command]
pub fn start_watching(path: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    let uuid = Uuid::new_v4();
    match start_file_watcher(&path, app_handle) {
        Ok(watcher) => {
            WATCHERS.lock().unwrap().insert(uuid, watcher);
            Ok(uuid.to_string())
        }
        Err(e) => Err(format!("Failed to start watcher: {:?}", e)),
    }
}

#[tauri::command]
pub fn stop_watching(id: Option<String>) -> Result<String, String> {
    if id.is_none() {
        // 清空全部监听器
        for (_, watcher) in WATCHERS.lock().unwrap().iter() {
            if let Err(e) = stop_file_watcher(watcher.clone()) {
                return Err(format!("Failed to stop watcher: {}", e)); // 使用 format! 宏将错误转换为 String
            }
        }
        return Ok("All watchers stopped".to_string());
    }
    let uuid =
        Uuid::parse_str(&id.unwrap().to_string()).map_err(|e| format!("Invalid UUID: {:?}", e))?;
    if let Some(watcher) = WATCHERS.lock().unwrap().remove(&uuid) {
        match stop_file_watcher(watcher) {
            Ok(_) => Ok("Watcher stopped".to_string()),
            Err(e) => Err(format!("Failed to stop watcher: {:?}", e)),
        }
    } else {
        Err("No watcher found for this UUID".to_string())
    }
}

#[derive(serde::Serialize, Clone)]
struct FileEvent {
    path: String,
    op: String,
    timestamp: i64,
}
pub fn start_file_watcher(
    path: &str,
    app_handle: tauri::AppHandle,
) -> notify::Result<Arc<Mutex<notify::RecommendedWatcher>>> {
    let (tx, rx) = channel();
    // 使用正确的recommended_watcher构造函数
    let mut watcher = notify::recommended_watcher(tx)?; // 移除错误的时间参数

    let path_str = path.to_string();
    // 确保路径是有效的
    let listen_path = std::path::Path::new(path);

    // 启动监听
    watcher.watch(listen_path, RecursiveMode::Recursive)?;

    let watcher = Arc::new(Mutex::new(watcher));
    let main_window = app_handle
        .get_window("main")
        .ok_or_else(|| notify::Error::generic("主窗口不存在"))?;

    // 启动一个线程来处理事件
    std::thread::spawn(move || {
        for event in rx {
            let e = event.unwrap();
            let changed_path = e.paths[0].to_string_lossy().into_owned();

            let path2: Vec<&str> = changed_path.split(&path_str).collect();

            // 判断变更文件是否在监听路径下
            if path2[1].split("\\").collect::<Vec<_>>().len() > 1 {
                continue;
            }

            let custom_event = FileEvent {
                path: changed_path,
                op: format!("{:?}", e.kind),
                timestamp: chrono::Local::now().timestamp_millis(),
            };
            let _ = main_window.emit("file_change_event", custom_event);
        }
    });

    Ok(watcher)
}

pub fn stop_file_watcher(watcher: Arc<Mutex<notify::RecommendedWatcher>>) -> notify::Result<()> {
    // 直接释放Mutex锁中的watcher对象
    // notify库的Watcher在drop时会自动清理资源
    drop(watcher.lock().unwrap());
    Ok(())
}
