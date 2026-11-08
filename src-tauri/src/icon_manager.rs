use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconInfo {
    pub name: String,
    pub file_name: String,
    pub path: String,
    pub ext: String,
    pub size_bytes: u64,
}

pub fn get_icons_dir() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from(".local/share"));
    let dir = base.join("korob").join("icons");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn list_icons() -> Result<Vec<IconInfo>, String> {
    let dir = get_icons_dir();
    let mut icons = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if ["svg", "png", "webp", "jpg", "jpeg"].contains(&ext.as_str()) {
                    let file_name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();
                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string();
                    let size_bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

                    icons.push(IconInfo {
                        name,
                        file_name,
                        path: path.to_string_lossy().to_string(),
                        ext,
                        size_bytes,
                    });
                }
            }
        }
    }

    icons.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(icons)
}

pub fn save_clipboard_image(data_base64: &str, ext: &str) -> Result<IconInfo, String> {
    let dir = get_icons_dir();
    let clean_ext = ext.trim_start_matches('.').to_lowercase();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let name = format!("icon_{}", timestamp);
    let file_name = format!("{}.{}", name, clean_ext);
    let target_path = dir.join(&file_name);

    // Strip data URL header if present (e.g. "data:image/png;base64,")
    let b64_clean = if let Some(idx) = data_base64.find(',') {
        &data_base64[idx + 1..]
    } else {
        data_base64
    };

    use base64::Engine;
    let engine = base64::engine::general_purpose::STANDARD;
    let bytes = engine
        .decode(b64_clean.trim())
        .map_err(|e| format!("Invalid base64 payload: {}", e))?;

    fs::write(&target_path, &bytes)
        .map_err(|e| format!("Failed to write icon file: {}", e))?;

    Ok(IconInfo {
        name,
        file_name,
        path: target_path.to_string_lossy().to_string(),
        ext: clean_ext,
        size_bytes: bytes.len() as u64,
    })
}

pub fn import_icon_file(source_path_str: &str) -> Result<IconInfo, String> {
    let source_path = Path::new(source_path_str);
    if !source_path.exists() {
        return Err("Source file does not exist".to_string());
    }

    let file_name = source_path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid source filename".to_string())?
        .to_string();

    let name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("icon")
        .to_string();

    let ext = source_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let dir = get_icons_dir();
    let target_path = dir.join(&file_name);

    fs::copy(source_path, &target_path)
        .map_err(|e| format!("Failed to copy file to icons directory: {}", e))?;

    let size_bytes = fs::metadata(&target_path).map(|m| m.len()).unwrap_or(0);

    Ok(IconInfo {
        name,
        file_name,
        path: target_path.to_string_lossy().to_string(),
        ext,
        size_bytes,
    })
}

pub fn delete_icon(file_name: &str) -> Result<(), String> {
    let dir = get_icons_dir();
    let path = dir.join(file_name);
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete icon: {}", e))?;
    }
    Ok(())
}

pub fn clean_unused_icons(used_icon_names: &[String]) -> Result<Vec<String>, String> {
    let dir = get_icons_dir();
    let mut deleted = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let file_name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();

                let is_used = used_icon_names.iter().any(|u| u == &file_name || u == &stem);
                if !is_used {
                    if fs::remove_file(&path).is_ok() {
                        deleted.push(file_name);
                    }
                }
            }
        }
    }

    Ok(deleted)
}

pub fn read_icon_base64(name_or_file: &str) -> Result<String, String> {
    let dir = get_icons_dir();
    let mut path = dir.join(name_or_file);
    if !path.exists() {
        // Try common extensions
        for ext in &["svg", "png", "webp"] {
            let candidate = dir.join(format!("{}.{}", name_or_file, ext));
            if candidate.exists() {
                path = candidate;
                break;
            }
        }
    }

    if !path.exists() {
        return Err("Icon not found".to_string());
    }

    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("png")
        .to_lowercase();

    use base64::Engine;
    let engine = base64::engine::general_purpose::STANDARD;
    let b64 = engine.encode(&bytes);

    let mime = match ext.as_str() {
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "webp" => "image/webp",
        "jpg" | "jpeg" => "image/jpeg",
        _ => "application/octet-stream",
    };

    Ok(format!("data:{};base64,{}", mime, b64))
}
