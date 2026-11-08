use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutKey {
    pub id: String,
    pub label: String,
    pub base_key: String,
    pub qmk: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
    pub hand: String,
    pub is_thumb: bool,
    pub matrix_row: usize,
    pub matrix_col: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardLayout {
    pub id: String,
    pub name: String,
    pub author: Option<String>,
    pub vial_uid: Option<String>,
    pub canvas: CanvasSize,
    pub keys: Vec<LayoutKey>,
}

pub fn get_layouts_dir() -> PathBuf {
    if let Some(cfg_dir) = crate::config_manager::get_config_dir() {
        let dir = cfg_dir.join("layouts");
        let _ = fs::create_dir_all(&dir);
        dir
    } else {
        PathBuf::from("layouts")
    }
}

pub fn list_layouts() -> Vec<String> {
    let dir = get_layouts_dir();
    let mut layouts = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    layouts.push(stem.to_string());
                }
            }
        }
    }
    if layouts.is_empty() {
        layouts.push("dactyl_manuform".to_string());
    }
    layouts
}

pub fn load_layout(id: &str) -> Result<KeyboardLayout, String> {
    let dir = get_layouts_dir();
    let file_name = if id.ends_with(".yaml") {
        id.to_string()
    } else {
        format!("{}.yaml", id)
    };
    let path = dir.join(&file_name);

    if !path.exists() {
        return Err(format!("Layout file not found at {:?}", path));
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read layout file: {}", e))?;

    let layout: KeyboardLayout = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse layout YAML: {}", e))?;

    Ok(layout)
}

pub fn save_layout(layout: &KeyboardLayout) -> Result<(), String> {
    let dir = get_layouts_dir();
    let file_name = format!("{}.yaml", layout.id);
    let path = dir.join(file_name);

    let yaml_str = serde_yaml::to_string(layout)
        .map_err(|e| format!("Failed to serialize layout: {}", e))?;

    fs::write(&path, yaml_str)
        .map_err(|e| format!("Failed to write layout file: {}", e))?;

    Ok(())
}
