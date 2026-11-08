export interface ThemeOption {
  id: string;
  name: string;
  category: 'dark' | 'light';
  previewBg: string;
  previewAccent: string;
  isCustom?: boolean;
}

export interface CustomTheme {
  id: string;
  name: string;
  category: string;
  bg_base: string;
  bg_sidebar: string;
  bg_card: string;
  border_subtle: string;
  text_primary: string;
  text_muted: string;
  accent: string;
  accent_hover: string;
  accent_text: string;
}

export interface BehaviorConfig {
  hide_on_blur: boolean;
  always_on_top: boolean;
  clear_search_on_open: boolean;
}

export interface FontConfig {
  family: string;
  size: number;
}

export interface ModifierDef {
  name: string;
  color: string;
}

export interface AppConfig {
  theme: {
    active: string;
    custom_themes: CustomTheme[];
  };
  font: FontConfig;
  view_mode: 'keyboard' | 'list';
  active_layout: string;
  active_layer: number;
  modifiers: Record<string, ModifierDef>;
  shortcuts_dir: string;
  behavior: BehaviorConfig;
  favorites: string[];
  learned: string[];
}

export interface ShortcutItem {
  id: string;
  command: string;
  description: string;
  keys: string;
  category: string;
  icon?: string;
  base_key: string;
  modifiers: string[];
}

export interface AppCheatsheet {
  app_id: string;
  name: string;
  description?: string;
  shortcuts: ShortcutItem[];
}

export interface AppCheatsheetSummary {
  app_id: string;
  name: string;
  file_name: string;
  item_count: number;
}

export interface CategoryGroup {
  name: string;
  items: ShortcutItem[];
}

export interface ActiveWindowInfo {
  app_id: string;
  title: string;
  geometry: string | null;
  is_focused: boolean;
}

export interface LayoutKey {
  id: string;
  label: string;
  base_key: string;
  qmk: string;
  x: number;
  y: number;
  width: number;
  height: number;
  rotation: number;
  hand: 'left' | 'right';
  is_thumb: boolean;
  matrix_row: number;
  matrix_col: number;
}

export interface CanvasSize {
  width: number;
  height: number;
}

export interface KeyboardLayout {
  id: string;
  name: string;
  author?: string;
  vial_uid?: string;
  canvas: CanvasSize;
  keys: LayoutKey[];
}

export interface IconInfo {
  name: string;
  file_name: string;
  path: string;
  ext: string;
  size_bytes: number;
}

export const THEMES: ThemeOption[] = [
  { id: 'gruvbox-dark', name: 'Gruvbox Dark', category: 'dark', previewBg: '#282828', previewAccent: '#b8bb26' },
  { id: 'gruvbox-light', name: 'Gruvbox Light', category: 'light', previewBg: '#fbf1c7', previewAccent: '#af3a03' },
  { id: 'monokai', name: 'Monokai', category: 'dark', previewBg: '#272822', previewAccent: '#a6e22e' },
  { id: 'catppuccin-mocha', name: 'Catppuccin Mocha', category: 'dark', previewBg: '#1e1e2e', previewAccent: '#cba6f7' },
  { id: 'catppuccin-latte', name: 'Catppuccin Latte', category: 'light', previewBg: '#eff1f5', previewAccent: '#8839ef' },
  { id: 'dracula', name: 'Dracula', category: 'dark', previewBg: '#282a36', previewAccent: '#bd93f9' },
  { id: 'solarized-dark', name: 'Solarized Dark', category: 'dark', previewBg: '#002b36', previewAccent: '#2aa198' },
  { id: 'solarized-light', name: 'Solarized Light', category: 'light', previewBg: '#fdf6e3', previewAccent: '#268bd2' },
];
