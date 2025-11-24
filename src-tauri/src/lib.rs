mod window_manager;
use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder, Manager, Emitter};
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
        .on_menu_event(|app_handle, event| {
            match event.id.0.as_str() {
                "quit" | "quit_tray" => std::process::exit(0),
                "show_window" => {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                // Preset handlers - emit any preset menu action to frontend
                id => {
                    if id.starts_with("preset_") || id.starts_with("custom_preset_") {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.emit("tray_menu_action", id);
                        }
                    }
                }
            }
        })
        .setup(|app| {
            // Build menu only on macOS
            #[cfg(target_os = "macos")]
            {
                let app_menu = SubmenuBuilder::new(app, "FrameFit")
                    .about(Default::default())
                    .separator()
                    .quit()
                    .build()?;

                let edit_menu = SubmenuBuilder::new(app, "Edit")
                    .undo()
                    .redo()
                    .separator()
                    .cut()
                    .copy()
                    .paste()
                    .select_all()
                    .build()?;

                let window_menu = SubmenuBuilder::new(app, "Window")
                    .minimize()
                    .build()?;

                let menu = MenuBuilder::new(app)
                    .items(&[&app_menu, &edit_menu, &window_menu])
                    .build()?;

                app.set_menu(menu)?;

                // Create tray menu with presets - build dynamically based on custom presets
                // Build default presets submenu
                let preset_iphone_se = tauri::menu::MenuItem::with_id(app, "preset_iphone_se", "iPhone SE (375×667)", true, None::<&str>)?;
                let preset_iphone_14 = tauri::menu::MenuItem::with_id(app, "preset_iphone_14", "iPhone 14 (390×844)", true, None::<&str>)?;
                let preset_ipad = tauri::menu::MenuItem::with_id(app, "preset_ipad", "iPad (768×1024)", true, None::<&str>)?;
                let preset_hd = tauri::menu::MenuItem::with_id(app, "preset_hd", "HD (1280×720)", true, None::<&str>)?;
                let preset_fhd = tauri::menu::MenuItem::with_id(app, "preset_fhd", "FHD (1920×1080)", true, None::<&str>)?;

                let presets_menu = SubmenuBuilder::new(app, "Presets")
                    .items(&[&preset_iphone_se, &preset_iphone_14, &preset_ipad, &preset_hd, &preset_fhd])
                    .build()?;

                let show_window_item = tauri::menu::MenuItem::with_id(app, "show_window", "Show Window", true, None::<&str>)?;
                let quit_item = tauri::menu::MenuItem::with_id(app, "quit_tray", "Quit", true, None::<&str>)?;

                // Build tray menu with static presets
                let tray_menu = SubmenuBuilder::new(app, "FrameFit")
                    .item(&presets_menu)
                    .separator()
                    .item(&show_window_item)
                    .item(&quit_item)
                    .build()?;

                let _tray = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&tray_menu)
                    .build(app)?;
            }

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("FrameFit")
                .inner_size(600.0, 850.0)
                .resizable(false);

            // set transparent title bar only when building for macOS
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window = win_builder.build().unwrap();

            // Handle window close event - minimize to tray instead of closing
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                use tauri::WindowEvent;
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window_clone.hide();
                }
            });

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
