mod window_manager;
use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder, Manager, Emitter};
use tauri::tray::TrayIconBuilder;
use tauri::menu::{MenuBuilder, SubmenuBuilder};
use tauri_plugin_store::StoreExt;
use window_manager::{get_frontmost_window, list_windows, resize_window, resize_window_by_id, check_accessibility_permissions, WindowInfo, ResizeRequest};
use serde::{Deserialize, Serialize};

// State to store the current tray icon (kept for future tray state management)
#[allow(dead_code)]
struct TrayState {
    #[allow(dead_code)]
    tray_id: Option<String>,
}

const TRAY_ID: &str = "main_tray";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub width: i32,
    pub height: i32,
}

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
    eprintln!("Resizing window ID: {} to {}x{}, center: {}", window_id, width, height, center);
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

#[tauri::command]
async fn rebuild_tray_menu(app_handle: tauri::AppHandle, custom_presets: Vec<Preset>) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        eprintln!("Rebuilding tray menu with {} custom presets", custom_presets.len());
        
        // Get the existing tray
        let tray = app_handle.tray_by_id(TRAY_ID)
            .ok_or("Tray not found")?;
        
        // Build default presets submenu with consistent IDs
        let preset_iphone_se = tauri::menu::MenuItem::with_id(&app_handle, "preset_iphone_se", "iPhone SE (375×667)", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let preset_iphone_14 = tauri::menu::MenuItem::with_id(&app_handle, "preset_iphone_14", "iPhone 14 (390×844)", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let preset_ipad = tauri::menu::MenuItem::with_id(&app_handle, "preset_ipad", "iPad (768×1024)", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let preset_hd = tauri::menu::MenuItem::with_id(&app_handle, "preset_hd", "HD (1280×720)", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let preset_fhd = tauri::menu::MenuItem::with_id(&app_handle, "preset_fhd", "FHD (1920×1080)", true, None::<&str>)
            .map_err(|e| e.to_string())?;

        // Build custom presets with consistent IDs based on preset name
        let mut custom_items = Vec::new();
        for preset in custom_presets.iter() {
            let label = format!("{} ({}×{})", preset.name, preset.width, preset.height);
            let safe_id = format!("custom_preset_{}", preset.name.to_lowercase().replace(" ", "_"));
            eprintln!("Adding custom preset: {} with id: {}", label, safe_id);
            let item = tauri::menu::MenuItem::with_id(&app_handle, &safe_id, &label, true, None::<&str>)
                .map_err(|e| e.to_string())?;
            custom_items.push(item);
        }

        // Build presets submenu
        let presets_menu = if custom_items.is_empty() {
            SubmenuBuilder::new(&app_handle, "Presets")
                .item(&preset_iphone_se)
                .item(&preset_iphone_14)
                .item(&preset_ipad)
                .item(&preset_hd)
                .item(&preset_fhd)
                .build()
                .map_err(|e| e.to_string())?
        } else {
            let mut builder = SubmenuBuilder::new(&app_handle, "Presets")
                .item(&preset_iphone_se)
                .item(&preset_iphone_14)
                .item(&preset_ipad)
                .item(&preset_hd)
                .item(&preset_fhd)
                .separator();
            for item in &custom_items {
                builder = builder.item(item);
            }
            builder.build().map_err(|e| e.to_string())?
        };

        let show_window_item = tauri::menu::MenuItem::with_id(&app_handle, "show_window", "Show Window", true, None::<&str>)
            .map_err(|e| e.to_string())?;
        let quit_item = tauri::menu::MenuItem::with_id(&app_handle, "quit_tray", "Quit", true, None::<&str>)
            .map_err(|e| e.to_string())?;

        // Build new tray menu
        let tray_menu = SubmenuBuilder::new(&app_handle, "FrameFit")
            .item(&presets_menu)
            .separator()
            .item(&show_window_item)
            .item(&quit_item)
            .build()
            .map_err(|e| e.to_string())?;

        // Update the tray menu
        tray.set_menu(Some(tray_menu))
            .map_err(|e| format!("Failed to update tray menu: {}", e))?;

        eprintln!("Successfully updated tray menu");
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(TrayState {
            tray_id: None,
        })
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
                // Get custom presets from store
                let mut custom_presets = Vec::new();
                
                if let Ok(store_instance) = app.store("presets.json") {
                    if let Some(presets_value) = store_instance.get("customPresets") {
                        if let Ok(presets) = serde_json::from_value::<Vec<Preset>>(presets_value) {
                            custom_presets = presets;
                        }
                    }
                }

                // Build default presets submenu
                let preset_iphone_se = tauri::menu::MenuItem::with_id(app, "preset_iphone_se", "iPhone SE (375×667)", true, None::<&str>)?;
                let preset_iphone_14 = tauri::menu::MenuItem::with_id(app, "preset_iphone_14", "iPhone 14 (390×844)", true, None::<&str>)?;
                let preset_ipad = tauri::menu::MenuItem::with_id(app, "preset_ipad", "iPad (768×1024)", true, None::<&str>)?;
                let preset_hd = tauri::menu::MenuItem::with_id(app, "preset_hd", "HD (1280×720)", true, None::<&str>)?;
                let preset_fhd = tauri::menu::MenuItem::with_id(app, "preset_fhd", "FHD (1920×1080)", true, None::<&str>)?;

                // Build custom presets
                let mut custom_items = Vec::new();
                for preset in custom_presets.iter() {
                    let label = format!("{} ({}×{})", preset.name, preset.width, preset.height);
                    let safe_id = format!("custom_preset_{}", preset.name.to_lowercase().replace(" ", "_"));
                    let item = tauri::menu::MenuItem::with_id(app, &safe_id, &label, true, None::<&str>)?;
                    custom_items.push(item);
                }

                // Build presets submenu
                let presets_menu = if custom_items.is_empty() {
                    SubmenuBuilder::new(app, "Presets")
                        .item(&preset_iphone_se)
                        .item(&preset_iphone_14)
                        .item(&preset_ipad)
                        .item(&preset_hd)
                        .item(&preset_fhd)
                        .build()?
                } else {
                    let mut builder = SubmenuBuilder::new(app, "Presets")
                        .item(&preset_iphone_se)
                        .item(&preset_iphone_14)
                        .item(&preset_ipad)
                        .item(&preset_hd)
                        .item(&preset_fhd)
                        .separator();
                    for item in &custom_items {
                        builder = builder.item(item);
                    }
                    builder.build()?
                };

                let show_window_item = tauri::menu::MenuItem::with_id(app, "show_window", "Show Window", true, None::<&str>)?;
                let quit_item = tauri::menu::MenuItem::with_id(app, "quit_tray", "Quit", true, None::<&str>)?;

                // Build tray menu
                let tray_menu = SubmenuBuilder::new(app, "FrameFit")
                    .item(&presets_menu)
                    .separator()
                    .item(&show_window_item)
                    .item(&quit_item)
                    .build()?;

                let _tray = TrayIconBuilder::with_id(TRAY_ID)
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
                #[allow(deprecated)]
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
            check_permissions,
            rebuild_tray_menu
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
