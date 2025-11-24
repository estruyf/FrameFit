use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowInfo {
    pub id: u32,
    pub title: String,
    pub app_name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResizeRequest {
    pub window_id: u32,
    pub width: i32,
    pub height: i32,
}

#[cfg(target_os = "macos")]
#[allow(deprecated)]
use cocoa::base::id;

#[cfg(target_os = "macos")]
use core_foundation::{
    array::{CFArrayGetCount, CFArrayGetValueAtIndex},
    base::TCFType,
    dictionary::{CFDictionaryGetValueIfPresent, CFDictionaryRef},
    number::{CFNumberGetValue, CFNumberRef},
    string::{CFString, CFStringGetCString, CFStringRef},
};

#[cfg(target_os = "macos")]
use core_graphics::window::{kCGWindowListOptionOnScreenOnly, CGWindowListCopyWindowInfo};

#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

#[cfg(target_os = "macos")]
pub fn check_accessibility_permissions() -> bool {
    use core_graphics::event::CGEvent;

    let event = CGEvent::new(core_graphics::event_source::CGEventSource::new(
        core_graphics::event_source::CGEventSourceStateID::CombinedSessionState,
    ).unwrap());

    event.is_ok()
}

#[cfg(target_os = "macos")]
pub fn list_windows() -> Result<Vec<WindowInfo>, String> {
    unsafe {
        let window_list = CGWindowListCopyWindowInfo(kCGWindowListOptionOnScreenOnly, 0);

        if window_list.is_null() {
            return Err("Failed to get window list".to_string());
        }

        let count = CFArrayGetCount(window_list);
        let mut windows = Vec::new();
        let excluded_apps = vec!["Dock", "Window Server", "FrameFit", "framefit"];

        for i in 0..count {
            let window_info = CFArrayGetValueAtIndex(window_list, i) as CFDictionaryRef;

            if let Some(window) = parse_window_info(window_info) {
                if window.width > 50 
                    && window.height > 50 
                    && !excluded_apps.iter().any(|app| app.eq_ignore_ascii_case(&window.app_name))
                {
                    windows.push(window);
                }
            }
        }

        Ok(windows)
    }
}

#[cfg(target_os = "macos")]
unsafe fn parse_window_info(window_dict: CFDictionaryRef) -> Option<WindowInfo> {
    let mut window_id: u32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut title = String::new();
    let mut app_name = String::new();

    let key_window_id = CFString::new("kCGWindowNumber");
    let key_bounds = CFString::new("kCGWindowBounds");
    let key_name = CFString::new("kCGWindowName");
    let key_owner = CFString::new("kCGWindowOwnerName");

    let mut value: *const std::ffi::c_void = std::ptr::null();

    if CFDictionaryGetValueIfPresent(
        window_dict,
        key_window_id.as_CFTypeRef() as *const _,
        &mut value,
    ) != 0
    {
        let num = value as CFNumberRef;
        CFNumberGetValue(
            num,
            core_foundation::number::kCFNumberSInt32Type,
            &mut window_id as *mut _ as *mut _,
        );
    }

    if CFDictionaryGetValueIfPresent(
        window_dict,
        key_name.as_CFTypeRef() as *const _,
        &mut value,
    ) != 0
    {
        let str_ref = value as CFStringRef;
        let mut buffer = vec![0u8; 256];
        if CFStringGetCString(
            str_ref,
            buffer.as_mut_ptr() as *mut _,
            256,
            core_foundation::string::kCFStringEncodingUTF8,
        ) != 0
        {
            if let Ok(s) = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const _).to_str() {
                title = s.to_string();
            }
        }
    }

    if CFDictionaryGetValueIfPresent(
        window_dict,
        key_owner.as_CFTypeRef() as *const _,
        &mut value,
    ) != 0
    {
        let str_ref = value as CFStringRef;
        let mut buffer = vec![0u8; 256];
        if CFStringGetCString(
            str_ref,
            buffer.as_mut_ptr() as *mut _,
            256,
            core_foundation::string::kCFStringEncodingUTF8,
        ) != 0
        {
            if let Ok(s) = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const _).to_str() {
                app_name = s.to_string();
            }
        }
    }

    if CFDictionaryGetValueIfPresent(
        window_dict,
        key_bounds.as_CFTypeRef() as *const _,
        &mut value,
    ) != 0
    {
        let bounds_dict = value as CFDictionaryRef;

        let key_x = CFString::new("X");
        let key_y = CFString::new("Y");
        let key_width = CFString::new("Width");
        let key_height = CFString::new("Height");

        let mut bound_value: *const std::ffi::c_void = std::ptr::null();

        if CFDictionaryGetValueIfPresent(
            bounds_dict,
            key_x.as_CFTypeRef() as *const _,
            &mut bound_value,
        ) != 0
        {
            let num = bound_value as CFNumberRef;
            let mut val: f64 = 0.0;
            CFNumberGetValue(
                num,
                core_foundation::number::kCFNumberFloat64Type,
                &mut val as *mut _ as *mut _,
            );
            x = val as i32;
        }

        if CFDictionaryGetValueIfPresent(
            bounds_dict,
            key_y.as_CFTypeRef() as *const _,
            &mut bound_value,
        ) != 0
        {
            let num = bound_value as CFNumberRef;
            let mut val: f64 = 0.0;
            CFNumberGetValue(
                num,
                core_foundation::number::kCFNumberFloat64Type,
                &mut val as *mut _ as *mut _,
            );
            y = val as i32;
        }

        if CFDictionaryGetValueIfPresent(
            bounds_dict,
            key_width.as_CFTypeRef() as *const _,
            &mut bound_value,
        ) != 0
        {
            let num = bound_value as CFNumberRef;
            let mut val: f64 = 0.0;
            CFNumberGetValue(
                num,
                core_foundation::number::kCFNumberFloat64Type,
                &mut val as *mut _ as *mut _,
            );
            width = val as i32;
        }

        if CFDictionaryGetValueIfPresent(
            bounds_dict,
            key_height.as_CFTypeRef() as *const _,
            &mut bound_value,
        ) != 0
        {
            let num = bound_value as CFNumberRef;
            let mut val: f64 = 0.0;
            CFNumberGetValue(
                num,
                core_foundation::number::kCFNumberFloat64Type,
                &mut val as *mut _ as *mut _,
            );
            height = val as i32;
        }
    }

    Some(WindowInfo {
        id: window_id,
        title,
        app_name,
        x,
        y,
        width,
        height,
    })
}

#[cfg(target_os = "macos")]
pub fn get_frontmost_window() -> Result<WindowInfo, String> {
    let window_list = list_windows()?;
    
    let excluded_apps = vec!["framefit", "Dock", "Window Server"];
    
    for window in window_list {
        if !excluded_apps.contains(&window.app_name.as_str()) {
            return Ok(window);
        }
    }
    
    Err("No suitable window found. Please open another application.".to_string())
}

#[cfg(target_os = "macos")]
#[allow(dead_code, unexpected_cfgs, deprecated)]
unsafe fn nsstring_to_string(ns_string: id) -> String {
    let utf8: *const u8 = msg_send![ns_string, UTF8String];
    let len: usize = msg_send![ns_string, lengthOfBytesUsingEncoding: 4];
    let bytes = std::slice::from_raw_parts(utf8, len);
    String::from_utf8_lossy(bytes).to_string()
}

#[cfg(target_os = "macos")]
pub fn resize_window(window: &WindowInfo, width: i32, height: i32, center: bool) -> Result<(), String> {
    if window.app_name == "framefit" {
        return Err("Cannot resize the FrameFit app itself".to_string());
    }

    let escaped_app = window.app_name.replace("\"", "\\\"");
    
    // Get actual screen dimensions using Core Graphics
    let screen_bounds = if center {
        #[cfg(target_os = "macos")]
        unsafe {
            use core_graphics::display::{CGMainDisplayID, CGDisplayBounds};
            let main_display = CGMainDisplayID();
            let bounds = CGDisplayBounds(main_display);
            Some((bounds.size.width as i32, bounds.size.height as i32))
        }
        #[cfg(not(target_os = "macos"))]
        None
    } else {
        None
    };
    
    let script = if center {
        let (screen_width, screen_height) = screen_bounds.unwrap_or((1920, 1080));
        format!(
            r#"
            activate application "{}"
            tell application "System Events"
                tell application process "{}"
                    if (count of windows) > 0 then
                        set frontWindow to first window
                        -- First resize the window
                        set size of frontWindow to {{{}, {}}}
                        
                        -- Then center it
                        -- Use actual screen dimensions from Rust
                        set screenWidth to {}
                        set screenHeight to {}

                        -- Add a pause to ensure the window has resized
                        delay 0.1
                        
                        -- Try again to ensure correct screen dimensions
                        set screenWidth to {}
                        set screenHeight to {}
                        
                        -- Calculate center position
                        set xPos to (screenWidth - {}) / 2
                        set yPos to (screenHeight - {}) / 2 + 50
                        
                        -- Ensure positive coordinates
                        if xPos < 0 then set xPos to 50
                        if yPos < 0 then set yPos to 100
                        
                        set position of frontWindow to {{xPos, yPos}}
                    else
                        error "No windows found"
                    end if
                end tell
            end tell
            "#,
            escaped_app, escaped_app, width, height, width, height, screen_width, screen_height, width, height
        )
    } else {
        format!(
            r#"
            activate application "{}"
            tell application "System Events"
                tell application process "{}"
                    if (count of windows) > 0 then
                        set frontWindow to first window
                        set size of frontWindow to {{{}, {}}}
                    else
                        error "No windows found"
                    end if
                end tell
            end tell
            "#,
            escaped_app, escaped_app, width, height
        )
    };

    execute_applescript(&script)
}

#[cfg(target_os = "macos")]
pub fn resize_window_by_id(request: &ResizeRequest, center: bool) -> Result<(), String> {
    let windows = list_windows()?;

    for window in windows {
        if window.id == request.window_id {
            return resize_window(&window, request.width, request.height, center);
        }
    }

    Err("Window not found".to_string())
}

#[cfg(target_os = "macos")]
fn execute_applescript(script: &str) -> Result<(), String> {
    use std::process::Command;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute AppleScript: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("AppleScript error: {}", error))
    }
}

#[cfg(not(target_os = "macos"))]
pub fn check_accessibility_permissions() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn list_windows() -> Result<Vec<WindowInfo>, String> {
    Err("Not supported on this platform".to_string())
}

#[cfg(not(target_os = "macos"))]
pub fn get_frontmost_window() -> Result<WindowInfo, String> {
    Err("Not supported on this platform".to_string())
}

#[cfg(not(target_os = "macos"))]
pub fn resize_window(_window: &WindowInfo, _width: i32, _height: i32, _center: bool) -> Result<(), String> {
    Err("Not supported on this platform".to_string())
}

#[cfg(not(target_os = "macos"))]
pub fn resize_window_by_id(_request: &ResizeRequest, _center: bool) -> Result<(), String> {
    Err("Not supported on this platform".to_string())
}
