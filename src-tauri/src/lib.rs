mod config_manager;
mod icon_manager;
mod layout_manager;
mod shortcut_manager;
mod umbriel_manager;

use config_manager::AppConfig;
use icon_manager::IconInfo;
use layout_manager::KeyboardLayout;
use shortcut_manager::{AppCheatsheet, AppCheatsheetSummary};
use std::path::{Path, PathBuf};
use tauri::{Manager, WebviewWindow};
use umbriel_manager::ActiveWindowInfo;

fn resolve_shortcuts_dir(config: &AppConfig) -> PathBuf {
    let p = Path::new(&config.shortcuts_dir);
    if config.shortcuts_dir.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(config.shortcuts_dir.trim_start_matches("~/"));
        }
    }
    if p.is_relative() {
        if let Some(cfg_dir) = config_manager::get_config_dir() {
            return cfg_dir.join(p);
        }
    }
    p.to_path_buf()
}

#[tauri::command]
fn get_focused_window() -> Result<ActiveWindowInfo, String> {
    umbriel_manager::get_focused_window()
}

#[tauri::command]
fn get_running_windows() -> Result<Vec<ActiveWindowInfo>, String> {
    umbriel_manager::get_all_windows()
}

#[tauri::command]
fn get_app_config() -> AppConfig {
    config_manager::load_config()
}

#[tauri::command]
fn save_app_config(config: AppConfig) -> Result<(), String> {
    config_manager::save_config(&config)
}

#[tauri::command]
fn get_config_yaml_file() -> Result<String, String> {
    config_manager::get_config_file_path_str()
}

#[tauri::command]
fn load_app_shortcuts(app_id: Option<String>) -> Result<AppCheatsheet, String> {
    let cfg = config_manager::load_config();
    let dir = resolve_shortcuts_dir(&cfg);
    let target = app_id
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "general".to_string());

    shortcut_manager::load_app_shortcuts(&dir, &target)
}

#[tauri::command]
fn save_app_shortcuts(cheatsheet: AppCheatsheet) -> Result<(), String> {
    let cfg = config_manager::load_config();
    let dir = resolve_shortcuts_dir(&cfg);
    shortcut_manager::save_app_shortcuts(&dir, &cheatsheet)
}

#[tauri::command]
fn list_available_apps() -> Result<Vec<AppCheatsheetSummary>, String> {
    let cfg = config_manager::load_config();
    let dir = resolve_shortcuts_dir(&cfg);
    Ok(shortcut_manager::list_app_shortcuts(&dir))
}

#[tauri::command]
fn read_shortcut_yaml_raw(app_id: String) -> Result<String, String> {
    let cfg = config_manager::load_config();
    let dir = resolve_shortcuts_dir(&cfg);
    let path = dir.join(format!("{}.yaml", app_id.trim_end_matches(".yaml")));
    if path.exists() {
        std::fs::read_to_string(&path).map_err(|e| e.to_string())
    } else {
        Err("Shortcut YAML file not found".to_string())
    }
}

#[tauri::command]
fn save_shortcut_yaml_raw(app_id: String, content: String) -> Result<(), String> {
    let cfg = config_manager::load_config();
    let dir = resolve_shortcuts_dir(&cfg);
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(format!("{}.yaml", app_id.trim_end_matches(".yaml")));
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_shortcut_file(app_id: String) -> Result<(), String> {
    let cfg = config_manager::load_config();
    let dir = resolve_shortcuts_dir(&cfg);
    let path = dir.join(format!("{}.yaml", app_id.trim_end_matches(".yaml")));
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn get_active_layout() -> Result<KeyboardLayout, String> {
    let cfg = config_manager::load_config();
    layout_manager::load_layout(&cfg.active_layout)
}

#[tauri::command]
fn list_available_layouts() -> Result<Vec<String>, String> {
    Ok(layout_manager::list_layouts())
}

#[tauri::command]
fn load_layout_by_id(id: String) -> Result<KeyboardLayout, String> {
    layout_manager::load_layout(&id)
}

#[tauri::command]
fn save_layout(layout: KeyboardLayout) -> Result<(), String> {
    layout_manager::save_layout(&layout)
}

#[tauri::command]
async fn pick_vial_file() -> Result<Option<String>, String> {
    let file = rfd::AsyncFileDialog::new()
        .set_title("Select Vial Layout File (.vil)")
        .add_filter("Vial Layout", &["vil", "json"])
        .pick_file()
        .await;

    Ok(file.map(|f| f.path().to_string_lossy().to_string()))
}

#[tauri::command]
fn list_user_icons() -> Result<Vec<IconInfo>, String> {
    icon_manager::list_icons()
}

#[tauri::command]
fn save_clipboard_icon(data_base64: String, ext: String) -> Result<IconInfo, String> {
    icon_manager::save_clipboard_image(&data_base64, &ext)
}

#[tauri::command]
fn import_icon_file(source_path: String) -> Result<IconInfo, String> {
    icon_manager::import_icon_file(&source_path)
}

#[tauri::command]
fn delete_user_icon(file_name: String) -> Result<(), String> {
    icon_manager::delete_icon(&file_name)
}

#[tauri::command]
fn clean_unused_icons(used_icon_names: Vec<String>) -> Result<Vec<String>, String> {
    icon_manager::clean_unused_icons(&used_icon_names)
}

#[tauri::command]
fn read_icon_base64(name: String) -> Result<String, String> {
    icon_manager::read_icon_base64(&name)
}

#[tauri::command]
async fn pick_icon_file() -> Result<Option<String>, String> {
    let file = rfd::AsyncFileDialog::new()
        .set_title("Select Icon Image")
        .add_filter("Images", &["svg", "png", "webp", "jpg", "jpeg"])
        .pick_file()
        .await;

    Ok(file.map(|f| f.path().to_string_lossy().to_string()))
}

#[tauri::command]
fn set_always_on_top(window: WebviewWindow, pinned: bool) -> Result<(), String> {
    window
        .set_always_on_top(pinned)
        .map_err(|e| e.to_string())?;
    let mut cfg = config_manager::load_config();
    cfg.behavior.always_on_top = pinned;
    let _ = config_manager::save_config(&cfg);
    Ok(())
}

#[tauri::command]
fn toggle_window(window: WebviewWindow) -> Result<(), String> {
    if let Ok(visible) = window.is_visible() {
        if visible {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
    Ok(())
}

#[tauri::command]
fn hide_window(window: WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
fn close_window(window: WebviewWindow) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_focused_window,
            get_running_windows,
            get_app_config,
            save_app_config,
            get_config_yaml_file,
            load_app_shortcuts,
            save_app_shortcuts,
            list_available_apps,
            read_shortcut_yaml_raw,
            save_shortcut_yaml_raw,
            delete_shortcut_file,
            get_active_layout,
            list_available_layouts,
            load_layout_by_id,
            save_layout,
            pick_vial_file,
            list_user_icons,
            save_clipboard_icon,
            import_icon_file,
            delete_user_icon,
            clean_unused_icons,
            read_icon_base64,
            pick_icon_file,
            set_always_on_top,
            toggle_window,
            hide_window,
            close_window,
        ])
        .setup(|app| {
            let cfg = config_manager::load_config();
            if let Some(win) = app.get_webview_window("main") {
                if cfg.behavior.always_on_top {
                    let _ = win.set_always_on_top(true);
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running korob application");
}
