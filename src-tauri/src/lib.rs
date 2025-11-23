mod window_manager;

use window_manager::{get_frontmost_window, list_windows, resize_window, resize_window_by_id, check_accessibility_permissions, WindowInfo, ResizeRequest};

#[tauri::command]
fn get_windows() -> Result<Vec<WindowInfo>, String> {
    list_windows()
}

#[tauri::command]
fn resize_frontmost_window(width: i32, height: i32, center: bool) -> Result<(), String> {
    let window_info = get_frontmost_window()?;
    resize_window(&window_info, width, height, center)
}

#[tauri::command]
fn resize_specific_window(window_id: u32, width: i32, height: i32, center: bool) -> Result<(), String> {
    let request = ResizeRequest {
        window_id,
        width,
        height,
    };
    resize_window_by_id(&request, center)
}

#[tauri::command]
fn check_permissions() -> bool {
    check_accessibility_permissions()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_windows,
            resize_frontmost_window,
            resize_specific_window,
            check_permissions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
