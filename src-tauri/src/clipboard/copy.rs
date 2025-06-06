
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{CF_HDROP, OpenClipboard, EmptyClipboard, SetClipboardData, CloseClipboard};
use winapi::um::winbase::{GlobalLock, GlobalUnlock, GlobalAlloc, GMEM_MOVEABLE, GMEM_ZEROINIT};
use winapi::shared::minwindef::{DWORD, UINT};
use winapi::um::shellapi::{DragQueryFileW, HDROP}; // 修正导入路径

#[tauri::command]
pub fn copy(files: Vec<String>) -> Result<(), String> {
    unsafe {
        if OpenClipboard(null_mut()) == 0 {
            return Err("Failed to open clipboard".to_string());
        }

        EmptyClipboard();

        // 计算总大小：包括文件路径、NULL 终止符和 HDROP 结构体
        let mut total_size = std::mem::size_of::<DWORD>(); // 文件数量
        for path in &files {
            let os_str: &OsStr = path.as_ref();
            let wide_chars: Vec<u16> = os_str.encode_wide().chain(std::iter::once(0)).collect();
            total_size += wide_chars.len() * std::mem::size_of::<u16>();
        }
        total_size += std::mem::size_of::<u16>(); // 最后一个 NULL 终止符

        let h_global = GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, total_size) as *mut u8;
        if h_global.is_null() {
            CloseClipboard();
            return Err("Failed to allocate global memory".to_string());
        }

        let locked_memory = GlobalLock(h_global as *mut _) as *mut u8;
        if locked_memory.is_null() {
            GlobalUnlock(h_global as *mut _);
            CloseClipboard();
            return Err("Failed to lock global memory".to_string());
        }

        // 写入文件数量
        let file_count_ptr = locked_memory as *mut DWORD;
        *file_count_ptr = files.len() as DWORD;

        // 写入文件路径
        let mut ptr = locked_memory.offset(std::mem::size_of::<DWORD>() as isize);
        for path in &files {
            let os_str: &OsStr = path.as_ref();
            let wide_chars: Vec<u16> = os_str.encode_wide().chain(std::iter::once(0)).collect();
            for &wchar in &wide_chars {
                *((ptr as *mut u16).offset((ptr as usize / std::mem::size_of::<u16>()) as isize)) = wchar;
                ptr = ptr.offset(std::mem::size_of::<u16>() as isize);
            }
        }

        // 最终的 NULL 终止符
        *((ptr as *mut u16).offset((ptr as usize / std::mem::size_of::<u16>()) as isize)) = 0;

        GlobalUnlock(h_global as *mut _);

        if SetClipboardData(CF_HDROP, h_global as *mut _) == null_mut() {
            CloseClipboard();
            return Err("Failed to set clipboard data".to_string());
        }

        CloseClipboard();
    }

    println!("Copied {} files to clipboard", files.len());
    Ok(())
}