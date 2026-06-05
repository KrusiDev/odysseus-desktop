use tauri::{
    Manager,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    image::Image,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use std::fs;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::HashMap;

#[tauri::command]
fn save_and_navigate(window: tauri::WebviewWindow, app: tauri::AppHandle, url: String) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    fs::write(app_dir.join("odysseus_url.txt"), &url).map_err(|e| e.to_string())?;
    window.navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
        .map_err(|e| e.to_string())?;
    start_health_check(app.clone(), url.clone(), true);
    Ok(())
}

#[tauri::command]
fn reconnect(_window: tauri::WebviewWindow, app: tauri::AppHandle, url: String) -> Result<(), String> {
    let probe_url = url.clone();
    let probe_app = app.clone();
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap();
        let has_port = url::Url::parse(&probe_url).ok().and_then(|u| u.port()).is_some();
        if is_reachable(&client, &probe_url, has_port) {
            if let Some(win) = probe_app.get_webview_window("main") {
                let _ = win.navigate(probe_url.parse().unwrap());
                start_health_check_inner(probe_app, probe_url, false, false);
            }
        } else {
            navigate_to_error(&probe_app, "disconnected");
        }
    });
    Ok(())
}

#[tauri::command]
fn navigate_home(app: tauri::AppHandle) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let url_file = app_dir.join("odysseus_url.txt");
    if url_file.exists() {
        fs::remove_file(&url_file).map_err(|e| e.to_string())?;
    }
    let window = app.get_webview_window("main").ok_or("no window")?;
    let home = if cfg!(dev) { "http://localhost:1420" } else { "tauri://localhost" };
    window.navigate(home.parse().map_err(|e: url::ParseError| e.to_string())?)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_saved_url(app: tauri::AppHandle) -> Option<String> {
    let app_dir = app.path().app_data_dir().ok()?;
    let saved = fs::read_to_string(app_dir.join("odysseus_url.txt")).ok()?;
    let trimmed = saved.trim().to_string();
    if trimmed.is_empty() { None } else { Some(trimmed) }
}

#[tauri::command]
fn reset_url(app: tauri::AppHandle) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let url_file = app_dir.join("odysseus_url.txt");
    if url_file.exists() {
        fs::remove_file(&url_file).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn zoom_in(app: tauri::AppHandle) -> Result<(), String> {
    adjust_zoom(&app, 0.1)
}

#[tauri::command]
fn zoom_out(app: tauri::AppHandle) -> Result<(), String> {
    adjust_zoom(&app, -0.1)
}

#[tauri::command]
fn zoom_reset(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("main") {
        let zoom_state = app.state::<Arc<Mutex<f64>>>();
        let mut zoom = zoom_state.lock().unwrap();
        *zoom = 1.0;
        win.set_zoom(1.0).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn adjust_zoom(app: &tauri::AppHandle, delta: f64) -> Result<(), String> {
    let key = if delta > 0.0 { "in" } else { "out" };
    let key_times = app.state::<Arc<Mutex<HashMap<String, Instant>>>>();
    let mut map = key_times.lock().unwrap();
    let now = Instant::now();
    if let Some(last) = map.get(key) {
        if last.elapsed() < Duration::from_millis(200) {
            map.insert(key.to_string(), now);
            return Ok(());
        }
    }
    map.insert(key.to_string(), now);
    drop(map);

    if let Some(win) = app.get_webview_window("main") {
        let zoom_state = app.state::<Arc<Mutex<f64>>>();
        let mut zoom = zoom_state.lock().unwrap();
        *zoom = (*zoom + delta).clamp(0.3, 3.0);
        win.set_zoom(*zoom).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn start_health_check(app: tauri::AppHandle, url: String, initial_probe: bool) {
    start_health_check_inner(app, url, initial_probe, false);
}

fn start_health_check_inner(app: tauri::AppHandle, url: String, initial_probe: bool, reconnecting: bool) {
    let initial_delay = if reconnecting { 500 } else { 2000 };
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap();
        let has_port = url::Url::parse(&url).ok().and_then(|u| u.port()).is_some();

        if initial_probe {
            std::thread::sleep(Duration::from_millis(initial_delay));
            if !is_reachable(&client, &url, has_port) {
                let error_type = if reconnecting { "disconnected" } else { "unreachable" };
                navigate_to_error(&app, error_type);
                return;
            }
        }

        loop {
            std::thread::sleep(Duration::from_secs(if cfg!(dev) { 10 } else { 60 }));
            let app_dir = app.path().app_data_dir().unwrap();
            if !app_dir.join("odysseus_url.txt").exists() {
                break;
            }
            if !is_reachable(&client, &url, has_port) {
                navigate_to_error(&app, "disconnected");
                break;
            }
        }
    });
}

fn is_reachable(client: &reqwest::blocking::Client, url: &str, has_port: bool) -> bool {
    match client.get(url).send() {
        Ok(resp) => {
            if has_port {
                true
            } else {
                let status = resp.status().as_u16();
                status != 502 && status != 503
            }
        }
        Err(_) => false,
    }
}

fn navigate_to_error(app: &tauri::AppHandle, error_type: &str) {
    if let Some(win) = app.get_webview_window("main") {
        if error_type == "unreachable" {
            let app_dir = app.path().app_data_dir().unwrap();
            let _ = fs::remove_file(app_dir.join("odysseus_url.txt"));
        }
        let home = if cfg!(dev) {
            format!("http://localhost:1420?error={}", error_type)
        } else {
            format!("tauri://localhost?error={}", error_type)
        };
        let _ = win.navigate(home.parse().unwrap());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            save_and_navigate, reset_url, navigate_home,
            get_saved_url, reconnect, zoom_in, zoom_out, zoom_reset
        ])
        .manage(Arc::new(Mutex::new(1.0f64)))
        .manage(Arc::new(Mutex::new(HashMap::<String, Instant>::new())))
        .setup(|app| {
            // System tray
            let zoom_in_item = MenuItem::with_id(app, "zoom_in", "Zoom In (Ctrl +)", true, None::<&str>)?;
            let zoom_out_item = MenuItem::with_id(app, "zoom_out", "Zoom Out (Ctrl -)", true, None::<&str>)?;
            let zoom_reset_item = MenuItem::with_id(app, "zoom_reset", "Reset Zoom (Ctrl 0)", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let disconnect_item = MenuItem::with_id(app, "disconnect", "Disconnect", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit Odysseus", true, None::<&str>)?;

            let tray_menu = Menu::with_items(app, &[
                &zoom_in_item,
                &zoom_out_item,
                &zoom_reset_item,
                &separator,
                &disconnect_item,
                &quit_item,
            ])?;

            let icon = Image::from_bytes(include_bytes!("../icons/32x32.png"))?;

            TrayIconBuilder::new()
                .icon(icon)
                .menu(&tray_menu)
                .tooltip("Odysseus")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "zoom_in" => { let _ = adjust_zoom(app, 0.1); }
                        "zoom_out" => { let _ = adjust_zoom(app, -0.1); }
                        "zoom_reset" => { let _ = zoom_reset(app.clone()); }
                        "disconnect" => { let _ = navigate_home(app.clone()); }
                        "quit" => app.exit(0),
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        if let Some(win) = tray.app_handle().get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Global shortcuts
            let app_handle = app.handle().clone();
            let _ = app.global_shortcut().on_shortcuts(
                [
                    Shortcut::new(Some(Modifiers::CONTROL), Code::Equal),
                    Shortcut::new(Some(Modifiers::CONTROL), Code::Minus),
                    Shortcut::new(Some(Modifiers::CONTROL), Code::Digit0),
                    Shortcut::new(None, Code::F11),
                ],
                move |_app, shortcut, _event| {
                    match shortcut.key {
                        Code::Equal  => { let _ = adjust_zoom(&app_handle, 0.1); }
                        Code::Minus  => { let _ = adjust_zoom(&app_handle, -0.1); }
                        Code::Digit0 => { let _ = zoom_reset(app_handle.clone()); }
                        Code::F11    => {
                            let key_times = app_handle.state::<Arc<Mutex<HashMap<String, Instant>>>>();
                            let mut map = key_times.lock().unwrap();
                            let now = Instant::now();
                            let should_fire = map.get("f11")
                                .map(|last| last.elapsed() >= Duration::from_millis(300))
                                .unwrap_or(true);
                            map.insert("f11".to_string(), now);
                            drop(map);
                            if should_fire {
                                if let Some(win) = app_handle.get_webview_window("main") {
                                    let is_fullscreen = win.is_fullscreen().unwrap_or(false);
                                    let _ = win.set_fullscreen(!is_fullscreen);
                                }
                            }
                        }
                        _ => {}
                    }
                },
            )?;

            // Restore saved URL on startup
            let app_dir = app.path().app_data_dir().unwrap();
            let url_file = app_dir.join("odysseus_url.txt");
            if let Ok(saved_url) = fs::read_to_string(&url_file) {
                let saved_url = saved_url.trim().to_string();
                if !saved_url.is_empty() {
                    let app_handle = app.handle().clone();
                    std::thread::spawn(move || {
                        let client = reqwest::blocking::Client::builder()
                            .timeout(Duration::from_secs(3))
                            .build()
                            .unwrap();
                        let has_port = url::Url::parse(&saved_url).ok().and_then(|u| u.port()).is_some();
                        if is_reachable(&client, &saved_url, has_port) {
                            if let Some(win) = app_handle.get_webview_window("main") {
                                let _ = win.navigate(saved_url.parse().unwrap());
                                start_health_check_inner(app_handle, saved_url, false, false);
                            }
                        } else {
                            navigate_to_error(&app_handle, "disconnected");
                        }
                    });
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
