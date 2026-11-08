<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Header from '$lib/components/Header.svelte';
  import KeyboardCanvas from '$lib/components/KeyboardCanvas.svelte';
  import CategorySection from '$lib/components/CategorySection.svelte';
  import SettingsModal from '$lib/components/SettingsModal.svelte';
  import YamlEditorModal from '$lib/components/YamlEditorModal.svelte';
  import IconLibraryModal from '$lib/components/IconLibraryModal.svelte';
  import KeyActionModal from '$lib/components/KeyActionModal.svelte';
  import type {
    AppConfig,
    ActiveWindowInfo,
    AppCheatsheet,
    AppCheatsheetSummary,
    KeyboardLayout,
    LayoutKey,
    ShortcutItem,
    CategoryGroup,
    CustomTheme,
    FontConfig,
  } from '$lib/types';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { Frown } from '@lucide/svelte';

  let config = $state<AppConfig>({
    theme: { active: 'gruvbox-dark', custom_themes: [] },
    font: { family: 'system-ui, -apple-system, sans-serif', size: 13 },
    view_mode: 'keyboard',
    active_layout: 'dactyl_manuform',
    active_layer: 0,
    modifiers: {
      ctrl: { name: 'Ctrl', color: '#facc15' },
      alt: { name: 'Alt', color: '#c084fc' },
      shift: { name: 'Shift', color: '#38bdf8' },
      super: { name: 'Super', color: '#ef4444' },
    },
    shortcuts_dir: '~/.config/korob/shortcuts',
    behavior: {
      hide_on_blur: false,
      always_on_top: false,
      clear_search_on_open: false,
    },
    favorites: [],
    learned: [],
  });

  let activeWindow = $state<ActiveWindowInfo | null>(null);
  let runningWindows = $state<ActiveWindowInfo[]>([]);
  let availableApps = $state<AppCheatsheetSummary[]>([]);
  let keyboardLayout = $state<KeyboardLayout | null>(null);
  let currentCheatsheet = $state<AppCheatsheet | null>(null);
  let shortcuts = $state<ShortcutItem[]>([]);
  let searchQuery = $state('');
  let showFavoritesOnly = $state(false);
  let collapsedCategories = $state<Record<string, boolean>>({});

  let isSettingsOpen = $state(false);
  let isEditorOpen = $state(false);
  let isIconLibraryOpen = $state(false);
  let isKeyActionOpen = $state(false);
  let selectedKey = $state<LayoutKey | null>(null);
  let selectedShortcut = $state<ShortcutItem | null>(null);
  let headerRef: any = $state(null);

  let filteredShortcuts = $derived(
    shortcuts.filter((item) => {
      if (showFavoritesOnly && !config.favorites.includes(item.id)) {
        return false;
      }
      if (!searchQuery.trim()) {
        return true;
      }
      const q = searchQuery.toLowerCase();
      return (
        item.command.toLowerCase().includes(q) ||
        item.keys.toLowerCase().includes(q) ||
        item.category.toLowerCase().includes(q)
      );
    })
  );

  let categoryGroups = $derived.by(() => {
    const map = new Map<string, ShortcutItem[]>();
    for (const item of filteredShortcuts) {
      if (!map.has(item.category)) {
        map.set(item.category, []);
      }
      map.get(item.category)!.push(item);
    }
    const groups: CategoryGroup[] = [];
    for (const [name, items] of map.entries()) {
      groups.push({ name, items });
    }
    return groups;
  });

  let availableCategories = $derived.by(() => {
    const set = new Set<string>();
    for (const s of shortcuts) {
      if (s.category) set.add(s.category);
    }
    return Array.from(set);
  });

  let usedIconNames = $derived(
    shortcuts
      .map((s) => s.icon)
      .filter((i): i is string => typeof i === 'string' && i.length > 0)
  );

  function applyFont(font: FontConfig) {
    if (typeof document === 'undefined') return;
    const doc = document.documentElement;
    doc.style.setProperty('--app-font-family', font.family || 'system-ui, sans-serif');
    doc.style.setProperty('--app-font-size', `${font.size || 13}px`);
  }

  function applyTheme(themeId: string, customThemeData?: CustomTheme) {
    if (typeof document === 'undefined') return;
    const doc = document.documentElement;

    if (customThemeData) {
      doc.removeAttribute('data-theme');
      doc.style.setProperty('--bg-base', customThemeData.bg_base);
      doc.style.setProperty('--bg-sidebar', customThemeData.bg_sidebar);
      doc.style.setProperty('--bg-card', customThemeData.bg_card);
      doc.style.setProperty('--border-subtle', customThemeData.border_subtle || customThemeData.bg_card);
      doc.style.setProperty('--border-strong', customThemeData.accent);
      doc.style.setProperty('--text-primary', customThemeData.text_primary);
      doc.style.setProperty('--text-muted', customThemeData.text_muted);
      doc.style.setProperty('--accent', customThemeData.accent);
      doc.style.setProperty('--accent-hover', customThemeData.accent_hover || customThemeData.accent);
      doc.style.setProperty('--accent-text', customThemeData.accent_text || customThemeData.bg_base);
      doc.style.setProperty('--badge-bg', customThemeData.bg_sidebar);
      doc.style.setProperty('--badge-text', customThemeData.accent);
      doc.style.setProperty('--kbd-bg', customThemeData.bg_sidebar);
      doc.style.setProperty('--kbd-border', customThemeData.border_subtle || customThemeData.bg_card);
      doc.style.setProperty('--kbd-text', customThemeData.text_primary);
    } else {
      doc.style.removeProperty('--bg-base');
      doc.style.removeProperty('--bg-sidebar');
      doc.style.removeProperty('--bg-card');
      doc.style.removeProperty('--border-subtle');
      doc.style.removeProperty('--border-strong');
      doc.style.removeProperty('--text-primary');
      doc.style.removeProperty('--text-muted');
      doc.style.removeProperty('--accent');
      doc.style.removeProperty('--accent-hover');
      doc.style.removeProperty('--accent-text');
      doc.style.removeProperty('--badge-bg');
      doc.style.removeProperty('--badge-text');
      doc.style.removeProperty('--kbd-bg');
      doc.style.removeProperty('--kbd-border');
      doc.style.removeProperty('--kbd-text');
      doc.setAttribute('data-theme', themeId);
    }
  }

  async function loadData() {
    try {
      config = await invoke<AppConfig>('get_app_config');
      const custom = config.theme.custom_themes.find((t) => t.id === config.theme.active);
      applyTheme(config.theme.active, custom);
      applyFont(config.font);

      await loadLayout();
      await refreshActiveApp();
      await refreshAvailableApps();
      await refreshRunningWindows();
    } catch (e: any) {
      console.error('Failed to load initial data:', e);
    }
  }

  async function loadLayout() {
    try {
      keyboardLayout = await invoke<KeyboardLayout>('get_active_layout');
    } catch (e) {
      console.error('Failed to load active layout:', e);
    }
  }

  async function refreshActiveApp() {
    try {
      activeWindow = await invoke<ActiveWindowInfo>('get_focused_window');
      await loadShortcutsForApp(activeWindow.app_id);
    } catch (e) {
      console.error('Failed to get active window:', e);
    }
  }

  async function refreshAvailableApps() {
    try {
      availableApps = await invoke<AppCheatsheetSummary[]>('list_available_apps');
    } catch (e) {
      console.error('Failed to list apps:', e);
    }
  }

  async function refreshRunningWindows() {
    try {
      runningWindows = await invoke<ActiveWindowInfo[]>('get_running_windows');
    } catch (e) {
      console.error('Failed to get running windows:', e);
    }
  }

  async function loadShortcutsForApp(appId: string) {
    try {
      const cleanId = (appId || 'general').trim();
      const res = await invoke<AppCheatsheet>('load_app_shortcuts', { appId: cleanId });
      currentCheatsheet = res;
      shortcuts = res.shortcuts || [];
    } catch (e: any) {
      console.error('Failed to load shortcuts:', e);
      shortcuts = [];
    }
  }

  function handleSelectApp(appId: string) {
    activeWindow = {
      app_id: appId,
      title: appId,
      geometry: null,
      is_focused: true,
    };
    loadShortcutsForApp(appId);
  }

  function handleSelectKey(key: LayoutKey, matchedShortcut?: ShortcutItem) {
    selectedKey = key;
    selectedShortcut = matchedShortcut || null;
    isKeyActionOpen = true;
  }

  function handleEditShortcut(sc: ShortcutItem) {
    selectedShortcut = sc;
    const foundKey = keyboardLayout?.keys.find(
      (k) => k.base_key.toLowerCase() === sc.base_key.toLowerCase()
    );
    if (foundKey) {
      selectedKey = foundKey;
    } else {
      selectedKey = {
        id: `virtual_${sc.base_key}`,
        label: sc.base_key.toUpperCase(),
        base_key: sc.base_key,
        qmk: `KC_${sc.base_key.toUpperCase()}`,
        x: 0,
        y: 0,
        width: 48,
        height: 48,
        rotation: 0,
        hand: 'left',
        is_thumb: false,
        matrix_row: 0,
        matrix_col: 0,
      };
    }
    isKeyActionOpen = true;
  }

  async function handleSaveKeyAction(item: ShortcutItem) {
    if (!currentCheatsheet) {
      currentCheatsheet = {
        app_id: activeWindow?.app_id || 'general',
        name: activeWindow?.app_id || 'general',
        shortcuts: [],
      };
    }

    const idx = currentCheatsheet.shortcuts.findIndex(
      (s) => s.id === item.id || s.base_key.toLowerCase() === item.base_key.toLowerCase()
    );

    if (idx >= 0) {
      currentCheatsheet.shortcuts[idx] = item;
    } else {
      currentCheatsheet.shortcuts.push(item);
    }

    await invoke('save_app_shortcuts', { cheatsheet: currentCheatsheet });
    shortcuts = [...currentCheatsheet.shortcuts];
    await refreshAvailableApps();
  }

  async function handleDeleteKeyAction(shortcutId: string) {
    if (!currentCheatsheet) return;

    currentCheatsheet.shortcuts = currentCheatsheet.shortcuts.filter((s) => s.id !== shortcutId);
    await invoke('save_app_shortcuts', { cheatsheet: currentCheatsheet });
    shortcuts = [...currentCheatsheet.shortcuts];
    await refreshAvailableApps();
  }

  async function handleToggleFavorite(id: string) {
    if (config.favorites.includes(id)) {
      config.favorites = config.favorites.filter((f) => f !== id);
    } else {
      config.favorites = [...config.favorites, id];
    }
    await invoke('save_app_config', { config });
  }

  async function handleToggleLearned(id: string) {
    if (config.learned.includes(id)) {
      config.learned = config.learned.filter((l) => l !== id);
    } else {
      config.learned = [...config.learned, id];
    }
    await invoke('save_app_config', { config });
  }

  function handleToggleCollapse(category: string) {
    collapsedCategories[category] = !collapsedCategories[category];
  }

  async function handleTogglePin() {
    const next = !config.behavior.always_on_top;
    config.behavior.always_on_top = next;
    try {
      await invoke('set_always_on_top', { pinned: next });
    } catch (e) {
      console.error('Failed to pin window:', e);
    }
  }

  async function handleChangeViewMode(mode: 'keyboard' | 'list') {
    config.view_mode = mode;
    await invoke('save_app_config', { config });
  }

  async function handleToggleLayer() {
    config.active_layer = config.active_layer === 0 ? 1 : 0;
    await invoke('save_app_config', { config });
  }

  async function handleSelectTheme(themeId: string, customThemeData?: CustomTheme) {
    config.theme.active = themeId;
    applyTheme(themeId, customThemeData);
    await invoke('save_app_config', { config });
  }

  async function handleSaveCustomTheme(custom: CustomTheme) {
    const idx = config.theme.custom_themes.findIndex((t) => t.id === custom.id);
    if (idx >= 0) {
      config.theme.custom_themes[idx] = custom;
    } else {
      config.theme.custom_themes.push(custom);
    }
    config.theme.active = custom.id;
    applyTheme(custom.id, custom);
    await invoke('save_app_config', { config });
  }

  async function handleDeleteCustomTheme(id: string) {
    config.theme.custom_themes = config.theme.custom_themes.filter((t) => t.id !== id);
    if (config.theme.active === id) {
      config.theme.active = 'gruvbox-dark';
      applyTheme('gruvbox-dark');
    }
    await invoke('save_app_config', { config });
  }

  async function handleUpdateConfig(updated: AppConfig) {
    config = updated;
    applyFont(config.font);
    await invoke('save_app_config', { config });
  }

  function handleGlobalKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      if (isKeyActionOpen) {
        isKeyActionOpen = false;
        return;
      }
      if (isSettingsOpen) {
        isSettingsOpen = false;
        return;
      }
      if (isEditorOpen) {
        isEditorOpen = false;
        return;
      }
      if (isIconLibraryOpen) {
        isIconLibraryOpen = false;
        return;
      }
      if (searchQuery) {
        searchQuery = '';
        return;
      }
      const win = getCurrentWindow();
      win.close();
      return;
    }

    if (e.key === '/' && !['INPUT', 'TEXTAREA'].includes((e.target as HTMLElement)?.tagName)) {
      e.preventDefault();
      headerRef?.focusSearch();
    }
  }

  onMount(() => {
    loadData();
    window.addEventListener('keydown', handleGlobalKeydown);
    return () => {
      window.removeEventListener('keydown', handleGlobalKeydown);
    };
  });
</script>

<div class="flex flex-col h-full w-full select-none bg-[var(--bg-base)]">
  <Header
    bind:this={headerRef}
    {activeWindow}
    {runningWindows}
    {availableApps}
    {searchQuery}
    totalCount={shortcuts.length}
    filteredCount={filteredShortcuts.length}
    {showFavoritesOnly}
    isPinned={config.behavior.always_on_top}
    viewMode={config.view_mode}
    activeLayer={config.active_layer}
    onSearchChange={(q: string) => (searchQuery = q)}
    onSelectApp={handleSelectApp}
    onRefreshActiveApp={refreshActiveApp}
    onToggleFavoritesFilter={() => (showFavoritesOnly = !showFavoritesOnly)}
    onTogglePin={handleTogglePin}
    onChangeViewMode={handleChangeViewMode}
    onToggleLayer={handleToggleLayer}
    onOpenIconLibrary={() => (isIconLibraryOpen = true)}
    onOpenEditor={() => (isEditorOpen = true)}
    onOpenSettings={() => (isSettingsOpen = true)}
  />

  <main class="flex-1 overflow-hidden flex flex-col relative">
    {#if config.view_mode === 'keyboard'}
      <div class="flex-1 w-full h-full flex items-center justify-center p-2 overflow-hidden">
        <KeyboardCanvas
          layout={keyboardLayout}
          shortcuts={filteredShortcuts}
          modifiers={config.modifiers}
          {searchQuery}
          onSelectKey={handleSelectKey}
        />
      </div>
    {:else}
      <div class="flex-1 overflow-y-auto p-4 space-y-4">
        {#if categoryGroups.length > 0}
          {#each categoryGroups as group (group.name)}
            <CategorySection
              {group}
              favorites={config.favorites}
              learned={config.learned}
              isCollapsed={!!collapsedCategories[group.name]}
              onToggleCollapse={handleToggleCollapse}
              onToggleFavorite={handleToggleFavorite}
              onToggleLearned={handleToggleLearned}
              onEdit={handleEditShortcut}
            />
          {/each}
        {:else}
          <div class="h-full flex flex-col items-center justify-center text-center p-8 space-y-2">
            <Frown class="w-8 h-8 text-[var(--text-muted)]" />
            <p class="text-sm font-medium text-[var(--text-primary)]">No shortcuts found</p>
            <p class="text-xs text-[var(--text-muted)] max-w-xs">
              {#if searchQuery}
                No hotkeys match your search query "{searchQuery}".
              {:else if showFavoritesOnly}
                No shortcuts marked as favorites yet. Click the star icon to bookmark frequently used keys.
              {:else}
                No cheatsheet exists for <span class="font-mono text-[var(--accent)]">{activeWindow?.app_id || 'this application'}</span>.
                Click any key on the keyboard canvas or the editor icon to create one.
              {/if}
            </p>
          </div>
        {/if}
      </div>
    {/if}
  </main>

  <SettingsModal
    isOpen={isSettingsOpen}
    {config}
    onClose={() => (isSettingsOpen = false)}
    onSelectTheme={handleSelectTheme}
    onSaveCustomTheme={handleSaveCustomTheme}
    onDeleteCustomTheme={handleDeleteCustomTheme}
    onUpdateConfig={handleUpdateConfig}
  />

  <YamlEditorModal
    isOpen={isEditorOpen}
    currentAppId={activeWindow?.app_id || 'general'}
    onClose={() => (isEditorOpen = false)}
    onSaved={() => {
      refreshAvailableApps();
      if (activeWindow) {
        loadShortcutsForApp(activeWindow.app_id);
      }
    }}
  />

  <IconLibraryModal
    isOpen={isIconLibraryOpen}
    {usedIconNames}
    onClose={() => (isIconLibraryOpen = false)}
    onSelectIcon={(iconName) => {
      isIconLibraryOpen = false;
      if (selectedKey) {
        selectedShortcut = selectedShortcut
          ? { ...selectedShortcut, icon: iconName }
          : {
              id: `sc-${selectedKey.base_key}`,
              command: '',
              description: '',
              keys: `ctrl+${selectedKey.base_key}`,
              category: 'General',
              icon: iconName,
              base_key: selectedKey.base_key,
              modifiers: ['ctrl'],
            };
        isKeyActionOpen = true;
      }
    }}
  />

  <KeyActionModal
    isOpen={isKeyActionOpen}
    key={selectedKey}
    shortcut={selectedShortcut}
    availableModifiers={config.modifiers}
    {availableCategories}
    onClose={() => (isKeyActionOpen = false)}
    onSaveShortcut={handleSaveKeyAction}
    onDeleteShortcut={handleDeleteKeyAction}
    onOpenIconLibrary={() => {
      isKeyActionOpen = false;
      isIconLibraryOpen = true;
    }}
  />
</div>
