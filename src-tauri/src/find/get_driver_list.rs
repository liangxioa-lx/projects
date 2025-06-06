extern crate systemstat;

use serde::Serialize;
use systemstat::{Platform, System};

#[derive(Serialize)]
pub struct DriveInfo {
    pub name: String,
    pub mount_point: String,
    pub drive_type: String,
    pub total_space: u64,
    pub free_space: u64,
}

#[tauri::command]
pub async fn get_driver_list() -> Result<Vec<DriveInfo>, String> {
    let sys = System::new();
    let mounts = sys
        .mounts()
        .map_err(|e| format!("Failed to get mounts: {}", e))?;

    let drive_list: Vec<DriveInfo> = mounts
        .iter()
        .map(|mount| DriveInfo {
            name: mount.fs_mounted_from.clone(),
            drive_type: mount.fs_type.clone(),
            mount_point: mount.fs_mounted_on.clone(),
            free_space: mount.avail.as_u64(),
            total_space: mount.total.as_u64(),
        })
        .collect();

    Ok(drive_list)
}
