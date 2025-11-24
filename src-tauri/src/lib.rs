mod window_manager;
use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::menu::{MenuBuilder, SubmenuBuilder};
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
        .on_menu_event(|window, event| {
            match event.id.as_ref() {
                "quit" => std::process::exit(0),
                _ => {}
            }
        })
        .setup(|app| {
            // Build menu only on macOS
            #[cfg(target_os = "macos")]
            {
                let tray = TrayIconBuilder::new().icon(app.default_window_icon().unwrap().clone()).build(app)?;
            }

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("FrameFit")
                .inner_size(600.0, 850.0)
                .resizable(false);

            // set transparent title bar only when building for macOS
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window = win_builder.build().unwrap();

            // set background color only when building for macOS
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSColor, NSWindow};
                use cocoa::base::{id, nil};

                let ns_window = window.ns_window().unwrap() as id;
                unsafe {
                    // Set background color to match the gradient start #667eea
                    let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                        nil,
                        89.0 / 255.0,   // #5966bf red
                        102.0 / 255.0,  // #5966bf green
                        191.0 / 255.0,  // #5966bf blue
                        1.0,
                    );
                    ns_window.setBackgroundColor_(bg_color);
                }
            }

            Ok(())
        })
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
