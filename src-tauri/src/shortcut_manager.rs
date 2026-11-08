use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutItem {
    pub id: String,
    pub command: String,
    #[serde(default)]
    pub description: String,
    pub keys: String,
    pub category: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub base_key: String,
    #[serde(default)]
    pub modifiers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppCheatsheet {
    pub app_id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub shortcuts: Vec<ShortcutItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppCheatsheetSummary {
    pub app_id: String,
    pub name: String,
    pub file_name: String,
    pub item_count: usize,
}

pub fn parse_key_combination(keys: &str) -> (String, Vec<String>) {
    let clean = keys.trim().to_lowercase();
    let parts: Vec<&str> = clean
        .split('+')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut modifiers = Vec::new();
    let mut base_key = String::new();

    for part in parts {
        match part {
            "ctrl" | "control" | "lctrl" | "rctrl" => modifiers.push("ctrl".to_string()),
            "alt" | "lalt" | "ralt" | "option" => modifiers.push("alt".to_string()),
            "shift" | "lshift" | "rshift" => modifiers.push("shift".to_string()),
            "super" | "win" | "cmd" | "meta" | "gui" => modifiers.push("super".to_string()),
            other => {
                let mapped = match other {
                    "minus" | "-" => "minus",
                    "equal" | "plus" | "=" => "equal",
                    "space" => "space",
                    "enter" | "return" => "enter",
                    "esc" | "escape" => "esc",
                    "tab" => "tab",
                    "backspace" | "bksp" => "backspace",
                    "delete" | "del" => "delete",
                    "up" => "up",
                    "down" => "down",
                    "left" => "left",
                    "right" => "right",
                    "[" => "bracket_left",
                    "]" => "bracket_right",
                    ";" => "semicolon",
                    "'" => "quote",
                    "," => "comma",
                    "." => "dot",
                    "/" => "slash",
                    "\\" => "backslash",
                    "`" | "~" | "grave" => "grave",
                    k => k,
                };
                base_key = mapped.to_string();
            }
        }
    }

    if base_key.is_empty() && !modifiers.is_empty() {
        base_key = modifiers.last().cloned().unwrap_or_default();
    }

    (base_key, modifiers)
}

pub fn normalize_shortcut_item(mut item: ShortcutItem) -> ShortcutItem {
    let (base, mods) = parse_key_combination(&item.keys);
    if item.base_key.is_empty() {
        item.base_key = base;
    }
    if item.modifiers.is_empty() {
        item.modifiers = mods;
    }
    item
}

pub fn load_app_shortcuts(dir: &Path, app_id: &str) -> Result<AppCheatsheet, String> {
    let clean_id = app_id.trim().trim_end_matches(".yaml").trim_end_matches(".scut");
    let yaml_path = dir.join(format!("{}.yaml", clean_id));

    if yaml_path.exists() {
        let content = fs::read_to_string(&yaml_path)
            .map_err(|e| format!("Failed to read {}: {}", yaml_path.display(), e))?;

        let mut sheet: AppCheatsheet = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse YAML from {}: {}", yaml_path.display(), e))?;

        sheet.shortcuts = sheet
            .shortcuts
            .into_iter()
            .map(normalize_shortcut_item)
            .collect();

        return Ok(sheet);
    }

    // Try fallback to general.yaml
    let general_path = dir.join("general.yaml");
    if general_path.exists() {
        if let Ok(content) = fs::read_to_string(&general_path) {
            if let Ok(mut sheet) = serde_yaml::from_str::<AppCheatsheet>(&content) {
                sheet.shortcuts = sheet
                    .shortcuts
                    .into_iter()
                    .map(normalize_shortcut_item)
                    .collect();
                return Ok(sheet);
            }
        }
    }

    Ok(AppCheatsheet {
        app_id: clean_id.to_string(),
        name: clean_id.to_string(),
        description: None,
        shortcuts: Vec::new(),
    })
}

pub fn save_app_shortcuts(dir: &Path, sheet: &AppCheatsheet) -> Result<(), String> {
    let file_name = format!("{}.yaml", sheet.app_id);
    let path = dir.join(file_name);

    let yaml_str = serde_yaml::to_string(sheet)
        .map_err(|e| format!("Failed to serialize cheatsheet to YAML: {}", e))?;

    fs::write(&path, yaml_str)
        .map_err(|e| format!("Failed to write YAML file: {}", e))?;

    Ok(())
}

pub fn list_app_shortcuts(dir: &Path) -> Vec<AppCheatsheetSummary> {
    let mut summaries = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(sheet) = serde_yaml::from_str::<AppCheatsheet>(&content) {
                        let file_name = path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string();

                        summaries.push(AppCheatsheetSummary {
                            app_id: sheet.app_id,
                            name: sheet.name,
                            file_name,
                            item_count: sheet.shortcuts.len(),
                        });
                    }
                }
            }
        }
    }

    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    summaries
}
