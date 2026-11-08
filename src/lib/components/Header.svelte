<script lang="ts">
  import {
    Search,
    X,
    RotateCw,
    Pin,
    PinOff,
    Settings,
    FileEdit,
    Star,
    Layers,
    ChevronDown,
    Keyboard,
    ListFilter,
    Image,
  } from '@lucide/svelte';
  import type { ActiveWindowInfo, AppCheatsheetSummary } from '$lib/types';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let {
    activeWindow,
    runningWindows = [],
    availableApps = [],
    searchQuery = '',
    totalCount = 0,
    filteredCount = 0,
    showFavoritesOnly = false,
    isPinned = false,
    viewMode = 'keyboard',
    activeLayer = 0,
    onSearchChange,
    onSelectApp,
    onRefreshActiveApp,
    onToggleFavoritesFilter,
    onTogglePin,
    onChangeViewMode,
    onToggleLayer,
    onOpenIconLibrary,
    onOpenEditor,
    onOpenSettings,
  }: {
    activeWindow: ActiveWindowInfo | null;
    runningWindows: ActiveWindowInfo[];
    availableApps: AppCheatsheetSummary[];
    searchQuery: string;
    totalCount: number;
    filteredCount: number;
    showFavoritesOnly: boolean;
    isPinned: boolean;
    viewMode: 'keyboard' | 'list';
    activeLayer: number;
    onSearchChange: (q: string) => void;
    onSelectApp: (appId: string) => void;
    onRefreshActiveApp: () => void;
    onToggleFavoritesFilter: () => void;
    onTogglePin: () => void;
    onChangeViewMode: (mode: 'keyboard' | 'list') => void;
    onToggleLayer: () => void;
    onOpenIconLibrary: () => void;
    onOpenEditor: () => void;
    onOpenSettings: () => void;
  } = $props();

  let isDropdownOpen = $state(false);
  let searchInputEl: HTMLInputElement | null = $state(null);

  async function handleClose() {
    try {
      const win = getCurrentWindow();
      await win.close();
    } catch {}
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && searchQuery) {
      onSearchChange('');
    }
  }

  export function focusSearch() {
    searchInputEl?.focus();
    searchInputEl?.select();
  }
</script>

<header
  data-tauri-drag-region
  class="relative flex items-center justify-between gap-2.5 px-3 py-2 bg-[var(--bg-sidebar)] border-b border-[var(--border-subtle)] select-none shrink-0"
>
  <div class="relative flex items-center gap-1.5 shrink-0">
    <button
      type="button"
      class="flex items-center gap-2 px-2.5 py-1 rounded bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border-subtle)] text-xs font-medium text-[var(--text-primary)] transition-colors"
      onclick={() => (isDropdownOpen = !isDropdownOpen)}
      title="Switch active application or cheatsheet"
    >
      <Layers class="w-3.5 h-3.5 text-[var(--accent)]" />
      <span class="max-w-[120px] truncate font-semibold">
        {activeWindow?.app_id || 'general'}
      </span>
      <ChevronDown class="w-3 h-3 text-[var(--text-muted)]" />
    </button>

    <button
      type="button"
      class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors"
      title="Refresh active window from compositor"
      aria-label="Refresh active window"
      onclick={onRefreshActiveApp}
    >
      <RotateCw class="w-3.5 h-3.5" />
    </button>

    {#if isDropdownOpen}
      <div
        class="fixed inset-0 z-40"
        onclick={() => (isDropdownOpen = false)}
        aria-hidden="true"
      ></div>

      <div
        class="absolute top-full left-0 mt-1 w-64 max-h-72 overflow-y-auto bg-[var(--bg-sidebar)] border border-[var(--border-strong)] rounded-md shadow-xl z-50 py-1"
      >
        {#if runningWindows.length > 0}
          <div class="px-3 py-1 text-[10px] font-semibold tracking-wider text-[var(--text-muted)] uppercase">
            Running windows (from compositor)
          </div>
          {#each runningWindows as win}
            <button
              type="button"
              class="w-full flex flex-col items-start px-3 py-1.5 text-left text-xs hover:bg-[var(--bg-card-hover)] transition-colors {win.app_id === activeWindow?.app_id ? 'text-[var(--accent)] font-medium' : 'text-[var(--text-primary)]'}"
              onclick={() => {
                onSelectApp(win.app_id);
                isDropdownOpen = false;
              }}
            >
              <span class="font-medium truncate w-full">{win.app_id}</span>
              {#if win.title}
                <span class="text-[10px] text-[var(--text-muted)] truncate w-full">{win.title}</span>
              {/if}
            </button>
          {/each}
          <div class="my-1 border-t border-[var(--border-subtle)]"></div>
        {/if}

        <div class="px-3 py-1 text-[10px] font-semibold tracking-wider text-[var(--text-muted)] uppercase">
          Available Apps (.yaml)
        </div>
        {#each availableApps as app}
          <button
            type="button"
            class="w-full flex items-center justify-between px-3 py-1.5 text-left text-xs hover:bg-[var(--bg-card-hover)] transition-colors {app.app_id === activeWindow?.app_id ? 'text-[var(--accent)] font-medium' : 'text-[var(--text-primary)]'}"
            onclick={() => {
              onSelectApp(app.app_id);
              isDropdownOpen = false;
            }}
          >
            <span class="truncate">{app.name || app.app_id}</span>
            <span class="text-[10px] text-[var(--text-muted)]">{app.item_count} keys</span>
          </button>
        {/each}
      </div>
    {/if}

    <button
      type="button"
      class="px-2 py-0.5 rounded bg-[var(--bg-card)] border border-[var(--border-subtle)] hover:border-[var(--accent)] text-[10px] font-mono text-[var(--accent)] transition-colors"
      title="Toggle keyboard layer (Layer 0: Base / Layer 1: Nav)"
      aria-label="Toggle keyboard layer"
      onclick={onToggleLayer}
    >
      L{activeLayer}
    </button>
  </div>

  <div class="relative flex-1 max-w-sm">
    <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-[var(--text-muted)] pointer-events-none" />
    <input
      bind:this={searchInputEl}
      type="text"
      placeholder="Search hotkeys (press / to focus)..."
      aria-label="Search hotkeys"
      value={searchQuery}
      oninput={(e) => onSearchChange(e.currentTarget.value)}
      onkeydown={handleKeydown}
      class="w-full pl-8 pr-16 py-1 text-xs rounded bg-[var(--bg-card)] text-[var(--text-primary)] placeholder-[var(--text-muted)] border border-[var(--border-subtle)] focus:border-[var(--accent)] focus:outline-none transition-colors"
    />
    <div class="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-1 text-[10px] text-[var(--text-muted)] pointer-events-none">
      {#if searchQuery}
        <span>{filteredCount}/{totalCount}</span>
      {:else}
        <kbd class="text-[10px] py-0 px-1">/</kbd>
      {/if}
    </div>
  </div>

  <div class="flex items-center gap-1 shrink-0">
    <div class="flex items-center rounded bg-[var(--bg-card)] border border-[var(--border-subtle)] p-0.5 mr-1">
      <button
        type="button"
        class="p-1 rounded transition-colors {viewMode === 'keyboard' ? 'bg-[var(--accent)] text-[var(--bg-base)]' : 'text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
        title="Keyboard Canvas View"
        aria-label="Keyboard Canvas View"
        onclick={() => onChangeViewMode('keyboard')}
      >
        <Keyboard class="w-3.5 h-3.5" />
      </button>
      <button
        type="button"
        class="p-1 rounded transition-colors {viewMode === 'list' ? 'bg-[var(--accent)] text-[var(--bg-base)]' : 'text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
        title="Cheatsheet List View"
        aria-label="Cheatsheet List View"
        onclick={() => onChangeViewMode('list')}
      >
        <ListFilter class="w-3.5 h-3.5" />
      </button>
    </div>

    <button
      type="button"
      class="p-1.5 rounded text-[var(--text-muted)] hover:text-[var(--accent)] hover:bg-[var(--bg-card)] transition-colors"
      title="Custom Icon Library"
      aria-label="Open custom icon library"
      onclick={onOpenIconLibrary}
    >
      <Image class="w-3.5 h-3.5" />
    </button>

    <button
      type="button"
      class="p-1.5 rounded transition-colors {showFavoritesOnly ? 'bg-[var(--bg-card-active)] text-amber-400' : 'text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)]'}"
      title={showFavoritesOnly ? 'Show all shortcuts' : 'Filter favorites'}
      aria-label={showFavoritesOnly ? 'Show all shortcuts' : 'Filter favorites'}
      onclick={onToggleFavoritesFilter}
    >
      <Star class="w-3.5 h-3.5 {showFavoritesOnly ? 'fill-amber-400' : ''}" />
    </button>

    <button
      type="button"
      class="p-1.5 rounded transition-colors {isPinned ? 'bg-[var(--bg-card-active)] text-[var(--accent)]' : 'text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)]'}"
      title={isPinned ? 'Unpin window (Always on Top is ON)' : 'Pin window (Keep Always on Top)'}
      aria-label={isPinned ? 'Unpin window' : 'Pin window'}
      onclick={onTogglePin}
    >
      {#if isPinned}
        <Pin class="w-3.5 h-3.5 fill-[var(--accent)]" />
      {:else}
        <PinOff class="w-3.5 h-3.5" />
      {/if}
    </button>

    <button
      type="button"
      class="p-1.5 rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors"
      title="Open YAML Cheatsheet Editor"
      aria-label="Open YAML Cheatsheet Editor"
      onclick={onOpenEditor}
    >
      <FileEdit class="w-3.5 h-3.5" />
    </button>

    <button
      type="button"
      class="p-1.5 rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors"
      title="Settings & Themes"
      aria-label="Settings & Themes"
      onclick={onOpenSettings}
    >
      <Settings class="w-3.5 h-3.5" />
    </button>

    <div class="w-px h-4 bg-[var(--border-subtle)] mx-0.5"></div>

    <button
      type="button"
      class="p-1.5 rounded text-[var(--text-muted)] hover:text-rose-400 hover:bg-rose-500/10 transition-colors"
      title="Close window (Esc)"
      aria-label="Close window"
      onclick={handleClose}
    >
      <X class="w-3.5 h-3.5" />
    </button>
  </div>
</header>
