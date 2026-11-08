use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTheme {
    pub id: String,
    pub name: String,
    #[serde(default = "default_category")]
    pub category: String,
    pub bg_base: String,
    pub bg_sidebar: String,
    pub bg_card: String,
    #[serde(default)]
    pub border_subtle: String,
    pub text_primary: String,
    #[serde(default)]
    pub text_muted: String,
    pub accent: String,
    #[serde(default)]
    pub accent_hover: String,
    #[serde(default)]
    pub accent_text: String,
}

fn default_category() -> String {
    "dark".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_active_theme")]
    pub active: String,
    #[serde(default)]
    pub custom_themes: Vec<CustomTheme>,
}

fn default_active_theme() -> String {
    "gruvbox-dark".to_string()
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            active: default_active_theme(),
            custom_themes: vec![CustomTheme {
                id: "tokyo-night".to_string(),
                name: "Tokyo Night".to_string(),
                category: "dark".to_string(),
                bg_base: "#1a1b26".to_string(),
                bg_sidebar: "#16161e".to_string(),
                bg_card: "#24283b".to_string(),
                border_subtle: "#292e42".to_string(),
                text_primary: "#c0caf5".to_string(),
                text_muted: "#565f89".to_string(),
                accent: "#7aa2f7".to_string(),
                accent_hover: "#89ddff".to_string(),
                accent_text: "#15161e".to_string(),
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    #[serde(default = "default_false")]
    pub hide_on_blur: bool,
    #[serde(default = "default_false")]
    pub always_on_top: bool,
    #[serde(default = "default_false")]
    pub clear_search_on_open: bool,
}

fn default_false() -> bool {
    false
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            hide_on_blur: false,
            always_on_top: false,
            clear_search_on_open: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    #[serde(default = "default_font_family")]
    pub family: String,
    #[serde(default = "default_font_size")]
    pub size: u32,
}

fn default_font_family() -> String {
    "system-ui".to_string()
}
fn default_font_size() -> u32 {
    13
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: default_font_family(),
            size: default_font_size(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierDef {
    pub name: String,
    pub color: String,
}

pub fn default_modifiers() -> BTreeMap<String, ModifierDef> {
    let mut map = BTreeMap::new();
    map.insert(
        "ctrl".to_string(),
        ModifierDef {
            name: "Ctrl".to_string(),
            color: "#facc15".to_string(),
        },
    );
    map.insert(
        "alt".to_string(),
        ModifierDef {
            name: "Alt".to_string(),
            color: "#c084fc".to_string(),
        },
    );
    map.insert(
        "shift".to_string(),
        ModifierDef {
            name: "Shift".to_string(),
            color: "#38bdf8".to_string(),
        },
    );
    map.insert(
        "super".to_string(),
        ModifierDef {
            name: "Super".to_string(),
            color: "#ef4444".to_string(),
        },
    );
    map
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub theme: ThemeConfig,
    #[serde(default)]
    pub font: FontConfig,
    #[serde(default = "default_view_mode")]
    pub view_mode: String,
    #[serde(default = "default_layout")]
    pub active_layout: String,
    #[serde(default)]
    pub active_layer: usize,
    #[serde(default = "default_modifiers")]
    pub modifiers: BTreeMap<String, ModifierDef>,
    #[serde(default = "default_shortcuts_dir_str")]
    pub shortcuts_dir: String,
    #[serde(default)]
    pub behavior: BehaviorConfig,
    #[serde(default)]
    pub favorites: Vec<String>,
    #[serde(default)]
    pub learned: Vec<String>,
}

fn default_view_mode() -> String {
    "keyboard".to_string()
}

fn default_layout() -> String {
    "dactyl_manuform".to_string()
}

fn default_shortcuts_dir_str() -> String {
    get_default_shortcuts_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| String::from("~/.config/korob/shortcuts"))
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            font: FontConfig::default(),
            view_mode: default_view_mode(),
            active_layout: default_layout(),
            active_layer: 0,
            modifiers: default_modifiers(),
            shortcuts_dir: default_shortcuts_dir_str(),
            behavior: BehaviorConfig::default(),
            favorites: Vec::new(),
            learned: Vec::new(),
        }
    }
}

pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("korob"))
}

pub fn get_config_file_path() -> Option<PathBuf> {
    get_config_dir().map(|p| p.join("config.yaml"))
}

pub fn get_default_shortcuts_dir() -> Option<PathBuf> {
    get_config_dir().map(|p| p.join("shortcuts"))
}

pub fn load_config() -> AppConfig {
    let file = match get_config_file_path() {
        Some(f) => f,
        None => return AppConfig::default(),
    };

    if !file.exists() {
        let def = AppConfig::default();
        let _ = save_config(&def);
        return def;
    }

    match fs::read_to_string(&file) {
        Ok(content) => serde_yaml::from_str(&content).unwrap_or_else(|_| {
            let def = AppConfig::default();
            let _ = save_config(&def);
            def
        }),
        Err(_) => AppConfig::default(),
    }
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let file = get_config_file_path().ok_or_else(|| "Failed to get config path".to_string())?;
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let yaml = serde_yaml::to_string(config).map_err(|e| e.to_string())?;
    fs::write(&file, yaml).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_config_file_path_str() -> Result<String, String> {
    let file = get_config_file_path().ok_or_else(|| "Failed to get config path".to_string())?;
    if !file.exists() {
        let def = AppConfig::default();
        let _ = save_config(&def);
    }
    Ok(file.to_string_lossy().to_string())
}
