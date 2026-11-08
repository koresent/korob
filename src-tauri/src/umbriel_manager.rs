use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveWindowInfo {
    pub app_id: String,
    pub title: String,
    pub geometry: Option<String>,
    pub is_focused: bool,
}

fn is_korob(app_id: &str) -> bool {
    let lower = app_id.to_lowercase();
    lower == "korob" || lower == "com.korob.app"
}

fn parse_line(line: &str) -> Option<ActiveWindowInfo> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    let is_focused = trimmed.starts_with('*');
    let content = if is_focused {
        trimmed.strip_prefix('*')?.trim_start()
    } else {
        trimmed
    };

    let parts: Vec<&str> = content.split('\t').collect();
    if parts.is_empty() {
        return None;
    }

    let app_id = parts[0].trim().to_string();
    let title = if parts.len() > 1 {
        parts[1].trim().to_string()
    } else {
        String::new()
    };
    let geometry = if parts.len() > 2 {
        Some(parts[2].trim().to_string())
    } else {
        None
    };

    Some(ActiveWindowInfo {
        app_id,
        title,
        geometry,
        is_focused,
    })
}

pub fn get_focused_window() -> Result<ActiveWindowInfo, String> {
    // First, try running `umbriel focused-window`
    let output = Command::new("umbriel")
        .arg("focused-window")
        .output()
        .map_err(|e| format!("Failed to execute 'umbriel focused-window': {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(info) = parse_line(line) {
                if !is_korob(&info.app_id) {
                    return Ok(info);
                }
            }
        }
    }

    // If focused window is Korob or focused-window was empty, inspect `umbriel windows` to find the last active non-Korob window
    let all_output = Command::new("umbriel")
        .arg("windows")
        .output()
        .map_err(|e| format!("Failed to execute 'umbriel windows': {}", e))?;

    if all_output.status.success() {
        let stdout = String::from_utf8_lossy(&all_output.stdout);
        let mut first_non_korob = None;
        for line in stdout.lines() {
            if let Some(info) = parse_line(line) {
                if !is_korob(&info.app_id) {
                    if info.is_focused {
                        return Ok(info);
                    }
                    if first_non_korob.is_none() {
                        first_non_korob = Some(info);
                    }
                }
            }
        }
        if let Some(first) = first_non_korob {
            return Ok(first);
        }
    }

    Ok(ActiveWindowInfo {
        app_id: "general".to_string(),
        title: "Desktop / General".to_string(),
        geometry: None,
        is_focused: true,
    })
}

pub fn get_all_windows() -> Result<Vec<ActiveWindowInfo>, String> {
    let output = Command::new("umbriel")
        .arg("windows")
        .output()
        .map_err(|e| format!("Failed to execute 'umbriel windows': {}", e))?;

    if !output.status.success() {
        return Err("umbriel windows returned non-zero status".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut windows = Vec::new();

    for line in stdout.lines() {
        if let Some(info) = parse_line(line) {
            if !is_korob(&info.app_id) {
                windows.push(info);
            }
        }
    }

    Ok(windows)
}
