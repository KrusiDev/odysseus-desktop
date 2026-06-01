use tauri::Manager;
use std::fs;

#[tauri::command]
fn save_and_navigate(window: tauri::WebviewWindow, app: tauri::AppHandle, url: String) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    fs::write(app_dir.join("odysseus_url.txt"), &url).map_err(|e| e.to_string())?;
    window.navigate(url.parse().map_err(|e: url::ParseError| e.to_string())?)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save_and_navigate])
        .setup(|app| {
    let app_dir = app.path().app_data_dir().unwrap();
    let url_file = app_dir.join("odysseus_url.txt");
    if let Ok(saved_url) = fs::read_to_string(&url_file) {
        let saved_url = saved_url.trim().to_string();
        if !saved_url.is_empty() {
            let window = app.get_webview_window("main").unwrap();
            std::thread::sleep(std::time::Duration::from_millis(500));
            window.navigate(saved_url.parse().unwrap())?;
        }
    }
    Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}