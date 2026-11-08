<script lang="ts">
  import { X, Check, Palette, Plus, Trash2, Keyboard, LayoutList, Sliders, Type } from '@lucide/svelte';
  import { THEMES, type ThemeOption, type CustomTheme, type AppConfig, type ModifierDef } from '$lib/types';
  import { invoke } from '@tauri-apps/api/core';
  import { revealItemInDir } from '@tauri-apps/plugin-opener';

  let {
    isOpen = false,
    config,
    onClose,
    onSelectTheme,
    onSaveCustomTheme,
    onDeleteCustomTheme,
    onUpdateConfig,
  }: {
    isOpen: boolean;
    config: AppConfig;
    onClose: () => void;
    onSelectTheme: (themeId: string, customThemeData?: CustomTheme) => void;
    onSaveCustomTheme: (theme: CustomTheme) => Promise<void>;
    onDeleteCustomTheme: (id: string) => Promise<void>;
    onUpdateConfig: (updated: AppConfig) => Promise<void>;
  } = $props();

  let activeTab = $state<'keyboard' | 'appearance' | 'system'>('keyboard');

  // New modifier form state
  let newModId = $state('');
  let newModName = $state('');
  let newModColor = $state('#ec4899');
  let isAddingModifier = $state(false);

  // Popular font presets
  const FONT_PRESETS = [
    { label: 'System Default', value: 'system-ui, -apple-system, sans-serif' },
    { label: 'Inter', value: 'Inter, system-ui, sans-serif' },
    { label: 'JetBrains Mono', value: '"JetBrains Mono", monospace' },
    { label: 'Fira Code', value: '"Fira Code", monospace' },
    { label: 'Roboto', value: 'Roboto, sans-serif' },
    { label: 'Ubuntu', value: 'Ubuntu, sans-serif' },
    { label: 'Cantarell', value: 'Cantarell, sans-serif' },
  ];

  async function handleOpenConfigFile() {
    try {
      const configPath = await invoke<string>('get_config_yaml_file');
      await revealItemInDir(configPath);
    } catch (e) {
      console.error('Failed to open config file:', e);
    }
  }

  function handleAddModifier() {
    const cleanId = newModId.trim().toLowerCase().replace(/[^a-z0-9_-]/g, '');
    const cleanName = newModName.trim() || cleanId;
    if (!cleanId) return;

    if (!config.modifiers) {
      config.modifiers = {};
    }

    config.modifiers[cleanId] = {
      name: cleanName,
      color: newModColor,
    };

    onUpdateConfig(config);
    newModId = '';
    newModName = '';
    isAddingModifier = false;
  }

  function handleDeleteModifier(modId: string) {
    if (confirm(`Remove modifier "${modId}"?`)) {
      delete config.modifiers[modId];
      onUpdateConfig(config);
    }
  }

  function handleFontFamilyChange(val: string) {
    config.font.family = val;
    onUpdateConfig(config);
  }

  function handleFontSizeChange(size: number) {
    config.font.size = size;
    onUpdateConfig(config);
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-xs"
    onclick={onClose}
    aria-hidden="true"
  >
    <div
      class="w-full max-w-xl max-h-[85vh] flex flex-col rounded-xl bg-[var(--bg-base)] border border-[var(--border-strong)] shadow-2xl overflow-hidden animate-in fade-in duration-150"
      onclick={(e) => e.stopPropagation()}
      aria-hidden="true"
    >
      <!-- Modal Header -->
      <div class="flex items-center justify-between px-5 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-sidebar)]">
        <div class="flex items-center gap-2">
          <Palette class="w-4 h-4 text-[var(--accent)]" />
          <h2 class="text-sm font-semibold text-[var(--text-primary)]">Preferences & Setup</h2>
        </div>
        <button
          type="button"
          class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors"
          onclick={onClose}
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Navigation Tabs -->
      <div class="flex items-center gap-1 px-5 pt-2 border-b border-[var(--border-subtle)] bg-[var(--bg-sidebar)]">
        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium border-b-2 transition-colors {activeTab === 'keyboard' ? 'border-[var(--accent)] text-[var(--accent)]' : 'border-transparent text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
          onclick={() => (activeTab = 'keyboard')}
        >
          <span class="flex items-center gap-1.5">
            <Keyboard class="w-3.5 h-3.5" />
            <span>Keyboard & Modifiers</span>
          </span>
        </button>

        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium border-b-2 transition-colors {activeTab === 'appearance' ? 'border-[var(--accent)] text-[var(--accent)]' : 'border-transparent text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
          onclick={() => (activeTab = 'appearance')}
        >
          <span class="flex items-center gap-1.5">
            <Type class="w-3.5 h-3.5" />
            <span>Typography & Themes</span>
          </span>
        </button>

        <button
          type="button"
          class="px-3 py-1.5 text-xs font-medium border-b-2 transition-colors {activeTab === 'system' ? 'border-[var(--accent)] text-[var(--accent)]' : 'border-transparent text-[var(--text-muted)] hover:text-[var(--text-primary)]'}"
          onclick={() => (activeTab = 'system')}
        >
          <span class="flex items-center gap-1.5">
            <Sliders class="w-3.5 h-3.5" />
            <span>Paths & Config</span>
          </span>
        </button>
      </div>

      <!-- Tab Content Area -->
      <div class="flex-1 overflow-y-auto p-5 space-y-4">
        {#if activeTab === 'keyboard'}
          <div class="p-3.5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-2">
            <span class="text-xs font-semibold text-[var(--text-primary)] block">Default Startup View</span>
            <div class="grid grid-cols-2 gap-2 text-xs">
              <button
                type="button"
                class="px-3 py-2 rounded border text-left transition-colors {config.view_mode === 'keyboard' ? 'border-[var(--accent)] bg-[var(--accent)]/10 text-[var(--accent)]' : 'border-[var(--border-subtle)] text-[var(--text-primary)]'}"
                onclick={() => {
                  config.view_mode = 'keyboard';
                  onUpdateConfig(config);
                }}
              >
                <div class="flex items-center gap-2 font-semibold">
                  <Keyboard class="w-4 h-4 text-[var(--accent)]" />
                  <span>Keyboard Canvas</span>
                </div>
                <div class="text-[10px] text-[var(--text-muted)] mt-0.5">Interactive graphical keyboard layout</div>
              </button>

              <button
                type="button"
                class="px-3 py-2 rounded border text-left transition-colors {config.view_mode === 'list' ? 'border-[var(--accent)] bg-[var(--accent)]/10 text-[var(--accent)]' : 'border-[var(--border-subtle)] text-[var(--text-primary)]'}"
                onclick={() => {
                  config.view_mode = 'list';
                  onUpdateConfig(config);
                }}
              >
                <div class="flex items-center gap-2 font-semibold">
                  <LayoutList class="w-4 h-4 text-[var(--accent)]" />
                  <span>Cheatsheet List</span>
                </div>
                <div class="text-[10px] text-[var(--text-muted)] mt-0.5">Traditional categorized hotkey catalog</div>
              </button>
            </div>
          </div>

          <div class="p-3.5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-3">
            <div class="flex items-center justify-between">
              <div>
                <span class="text-xs font-semibold text-[var(--text-primary)] block">Modifiers & Palette</span>
                <span class="text-[10px] text-[var(--text-muted)]">Add custom modifier keys and customize their highlight colors</span>
              </div>
              <button
                type="button"
                class="flex items-center gap-1 px-2.5 py-1 text-xs rounded bg-[var(--accent)] text-[var(--accent-text)] font-semibold hover:brightness-110 transition-colors"
                onclick={() => (isAddingModifier = !isAddingModifier)}
              >
                <Plus class="w-3.5 h-3.5" />
                <span>Add Modifier</span>
              </button>
            </div>

            {#if isAddingModifier}
              <div class="p-3 rounded bg-[var(--bg-base)] border border-[var(--accent)] space-y-2">
                <span class="text-xs font-semibold text-[var(--text-primary)] block">New Modifier Key</span>
                <div class="grid grid-cols-3 gap-2 text-xs">
                  <div>
                    <label for="new-mod-id-input" class="text-[10px] text-[var(--text-muted)] block mb-0.5">ID (lowercase)</label>
                    <input
                      id="new-mod-id-input"
                      type="text"
                      bind:value={newModId}
                      placeholder="e.g. hyper, fn, leader"
                      class="w-full px-2 py-1 rounded bg-[var(--bg-card)] border border-[var(--border-subtle)] text-[var(--text-primary)] font-mono text-xs focus:outline-none focus:border-[var(--accent)]"
                    />
                  </div>
                  <div>
                    <label for="new-mod-name-input" class="text-[10px] text-[var(--text-muted)] block mb-0.5">Display Name</label>
                    <input
                      id="new-mod-name-input"
                      type="text"
                      bind:value={newModName}
                      placeholder="e.g. Hyper, Fn"
                      class="w-full px-2 py-1 rounded bg-[var(--bg-card)] border border-[var(--border-subtle)] text-[var(--text-primary)] text-xs focus:outline-none focus:border-[var(--accent)]"
                    />
                  </div>
                  <div>
                    <label for="new-mod-color-input" class="text-[10px] text-[var(--text-muted)] block mb-0.5">Color</label>
                    <div class="flex items-center gap-1.5">
                      <input
                        id="new-mod-color-input"
                        type="color"
                        bind:value={newModColor}
                        class="w-7 h-7 rounded cursor-pointer border-0 bg-transparent"
                      />
                      <span class="font-mono text-[10px] text-[var(--text-muted)]">{newModColor}</span>
                    </div>
                  </div>
                </div>
                <div class="flex justify-end gap-2 pt-1">
                  <button
                    type="button"
                    class="px-2.5 py-1 text-xs rounded text-[var(--text-muted)]"
                    onclick={() => (isAddingModifier = false)}
                  >
                    Cancel
                  </button>
                  <button
                    type="button"
                    class="px-3 py-1 text-xs rounded bg-[var(--accent)] text-[var(--accent-text)] font-semibold"
                    onclick={handleAddModifier}
                  >
                    Save Modifier
                  </button>
                </div>
              </div>
            {/if}

            <!-- Existing Modifiers List -->
            <div class="grid grid-cols-2 gap-2 text-xs">
              {#each Object.entries(config.modifiers || {}) as [modId, def]}
                <div class="flex items-center justify-between p-2 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)]">
                  <div class="flex items-center gap-2">
                    <input
                      type="color"
                      bind:value={def.color}
                      onchange={() => onUpdateConfig(config)}
                      class="w-6 h-6 rounded cursor-pointer border-0 bg-transparent shrink-0"
                    />
                    <div>
                      <span class="font-semibold text-[var(--text-primary)] block leading-tight">{def.name}</span>
                      <span class="font-mono text-[9px] text-[var(--text-muted)]">{modId} • {def.color}</span>
                    </div>
                  </div>

                  {#if !['ctrl', 'alt', 'shift', 'super'].includes(modId)}
                    <button
                      type="button"
                      class="p-1 rounded text-[var(--text-muted)] hover:text-rose-400 transition-colors"
                      onclick={() => handleDeleteModifier(modId)}
                      title="Delete custom modifier"
                    >
                      <Trash2 class="w-3.5 h-3.5" />
                    </button>
                  {/if}
                </div>
              {/each}
            </div>
          </div>

        {:else if activeTab === 'appearance'}
          <!-- Font Family & Font Size -->
          <div class="p-3.5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-3">
            <span class="text-xs font-semibold text-[var(--text-primary)] block">Typography (Font & Size)</span>

            <!-- Font Family -->
            <div>
              <span class="text-[11px] text-[var(--text-muted)] block mb-1">Font Family</span>
              <div class="grid grid-cols-2 gap-2 mb-2">
                {#each FONT_PRESETS as preset}
                  <button
                    type="button"
                    class="px-2.5 py-1.5 rounded border text-left text-xs transition-colors {config.font.family === preset.value ? 'border-[var(--accent)] bg-[var(--accent)]/10 text-[var(--accent)] font-semibold' : 'border-[var(--border-subtle)] bg-[var(--bg-base)] text-[var(--text-primary)] hover:border-[var(--border-strong)]'}"
                    onclick={() => handleFontFamilyChange(preset.value)}
                  >
                    {preset.label}
                  </button>
                {/each}
              </div>

              <div class="flex items-center gap-2 text-xs">
                <label for="custom-font-input" class="text-[10px] text-[var(--text-muted)] shrink-0">Custom font:</label>
                <input
                  id="custom-font-input"
                  type="text"
                  bind:value={config.font.family}
                  onchange={() => onUpdateConfig(config)}
                  placeholder="e.g. JetBrains Mono, Inter, Ubuntu..."
                  class="flex-1 px-2.5 py-1 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-primary)] font-mono text-xs focus:border-[var(--accent)] focus:outline-none"
                />
              </div>
            </div>

            <div class="pt-2 border-t border-[var(--border-subtle)]">
              <div class="flex items-center justify-between mb-1.5">
                <label for="font-size-slider" class="text-[11px] text-[var(--text-muted)]">Application Font Size</label>
                <span class="text-xs font-mono font-bold text-[var(--accent)]">{config.font.size}px</span>
              </div>
              <div class="flex items-center gap-3">
                <input
                  id="font-size-slider"
                  type="range"
                  min="11"
                  max="18"
                  step="1"
                  bind:value={config.font.size}
                  oninput={(e) => handleFontSizeChange(Number(e.currentTarget.value))}
                  class="flex-1 accent-[var(--accent)] cursor-pointer"
                />
                <span class="text-[10px] text-[var(--text-muted)] font-mono">11px — 18px</span>
              </div>
            </div>
          </div>

          <!-- Built-in Themes -->
          <div class="space-y-2">
            <span class="text-xs font-semibold uppercase tracking-wider text-[var(--text-muted)] block">
              Color Themes
            </span>
            <div class="grid grid-cols-2 gap-2">
              {#each THEMES as theme}
                <button
                  type="button"
                  class="flex items-center justify-between p-2.5 rounded border text-left transition-all {config.theme.active === theme.id ? 'border-[var(--accent)] bg-[var(--bg-card-active)]' : 'border-[var(--border-subtle)] bg-[var(--bg-card)] hover:border-[var(--border-strong)]'}"
                  onclick={() => onSelectTheme(theme.id)}
                >
                  <div class="flex items-center gap-2">
                    <div class="w-4 h-4 rounded-full border border-black/20 shrink-0" style="background-color: {theme.previewAccent};"></div>
                    <span class="text-xs font-medium text-[var(--text-primary)]">{theme.name}</span>
                  </div>
                  {#if config.theme.active === theme.id}
                    <Check class="w-3.5 h-3.5 text-[var(--accent)]" />
                  {/if}
                </button>
              {/each}
            </div>
          </div>

        {:else if activeTab === 'system'}
          <!-- Paths & Declarative Config -->
          <div class="p-3.5 rounded-lg bg-[var(--bg-card)] border border-[var(--border-subtle)] space-y-3">
            <span class="text-xs font-semibold text-[var(--text-primary)] block">Declarative Configuration Files</span>
            <div class="space-y-2 text-xs">
              <div class="flex items-center justify-between p-2.5 rounded bg-[var(--bg-base)]">
                <div>
                  <span class="font-medium text-[var(--text-primary)] block">Main Config</span>
                  <code class="text-[10px] text-[var(--text-muted)]">~/.config/korob/config.yaml</code>
                </div>
                <button
                  type="button"
                  class="px-2.5 py-1 rounded bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] text-xs text-[var(--accent)] transition-colors"
                  onclick={handleOpenConfigFile}
                >
                  Open in Folder
                </button>
              </div>

              <div class="p-2.5 rounded bg-[var(--bg-base)] space-y-1">
                <span class="font-medium text-[var(--text-primary)] block">Shortcuts Directory</span>
                <code class="text-[10px] text-[var(--text-muted)] block">~/.config/korob/shortcuts/</code>
                <p class="text-[10px] text-[var(--text-muted)]">Each application is stored in its own clean .yaml file.</p>
              </div>

              <div class="p-2.5 rounded bg-[var(--bg-base)] space-y-1">
                <span class="font-medium text-[var(--text-primary)] block">Custom Icons Directory</span>
                <code class="text-[10px] text-[var(--text-muted)] block">~/.local/share/korob/icons/</code>
                <p class="text-[10px] text-[var(--text-muted)]">Icons pasted from clipboard or imported from disk.</p>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Modal Footer -->
      <div class="flex items-center justify-end px-5 py-3 border-t border-[var(--border-subtle)] bg-[var(--bg-sidebar)]">
        <button
          type="button"
          class="px-4 py-1.5 text-xs rounded bg-[var(--accent)] text-[var(--accent-text)] font-semibold hover:brightness-110 transition-colors"
          onclick={onClose}
        >
          Done
        </button>
      </div>
    </div>
  </div>
{/if}
