<script lang="ts">
  import { X, Save, Trash2, Image, Check } from '@lucide/svelte';
  import type { LayoutKey, ShortcutItem, ModifierDef } from '$lib/types';
  import KeyIcon from './KeyIcon.svelte';

  let {
    isOpen = false,
    key,
    shortcut,
    availableModifiers = {},
    availableCategories = [],
    onClose,
    onSaveShortcut,
    onDeleteShortcut,
    onOpenIconLibrary,
  }: {
    isOpen: boolean;
    key: LayoutKey | null;
    shortcut: ShortcutItem | null;
    availableModifiers: Record<string, ModifierDef>;
    availableCategories: string[];
    onClose: () => void;
    onSaveShortcut: (item: ShortcutItem) => Promise<void>;
    onDeleteShortcut: (shortcutId: string) => Promise<void>;
    onOpenIconLibrary?: () => void;
  } = $props();

  let command = $state('');
  let description = $state('');
  let category = $state('General');
  let icon = $state('plus');
  let selectedModifiers = $state<string[]>([]);
  let isSaving = $state(false);

  const PRESET_ICONS = [
    'folder-1', 'folder-2', 'folder-3', 'folder-4', 'folder-5',
    'folder-6', 'folder-7', 'folder-8', 'folder-9', 'folder-x',
    'plus', 'x', 'pin', 'rotate-cw', 'history', 'clock',
    'scissors', 'copy', 'clipboard', 'undo-2', 'redo',
    'search', 'bookmark', 'download', 'download-cloud',
    'code', 'terminal', 'printer', 'book-open', 'external-link',
    'venetian-mask', 'volume-x', 'zoom-in', 'zoom-out', 'maximize-2',
    'save', 'check-square', 'file'
  ];

  $effect(() => {
    if (isOpen) {
      if (shortcut) {
        command = shortcut.command;
        description = shortcut.description || '';
        category = shortcut.category || 'General';
        icon = shortcut.icon || 'plus';
        selectedModifiers = [...(shortcut.modifiers || [])];
      } else if (key) {
        command = '';
        description = '';
        category = availableCategories[0] || 'General';
        icon = 'plus';
        selectedModifiers = ['ctrl'];
      }
    }
  });

  function toggleModifier(modId: string) {
    if (selectedModifiers.includes(modId)) {
      selectedModifiers = selectedModifiers.filter((m) => m !== modId);
    } else {
      selectedModifiers = [...selectedModifiers, modId];
    }
  }

  // Combined key combination string (e.g. "ctrl+shift+w")
  const fullKeysString = $derived.by(() => {
    const base = key?.base_key || '';
    if (selectedModifiers.length === 0) {
      return base;
    }
    return `${selectedModifiers.join('+')}+${base}`;
  });

  async function handleSave() {
    if (!command.trim() || !key) return;
    isSaving = true;

    const shortcutId = shortcut?.id || `sc-${key.base_key}-${Date.now().toString().slice(-4)}`;
    const updated: ShortcutItem = {
      id: shortcutId,
      command: command.trim(),
      description: description.trim(),
      category: category.trim() || 'General',
      keys: fullKeysString,
      base_key: key.base_key.toLowerCase(),
      modifiers: [...selectedModifiers],
      icon: icon || 'sparkles',
    };

    try {
      await onSaveShortcut(updated);
      onClose();
    } catch (e) {
      console.error('Failed to save shortcut:', e);
    } finally {
      isSaving = false;
    }
  }

  async function handleDelete() {
    if (!shortcut) return;
    if (!confirm(`Remove shortcut "${shortcut.command}"?`)) return;
    try {
      await onDeleteShortcut(shortcut.id);
      onClose();
    } catch (e) {
      console.error('Failed to delete shortcut:', e);
    }
  }
</script>

{#if isOpen && key}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-xs"
    onclick={onClose}
    aria-hidden="true"
  >
    <div
      class="w-full max-w-lg bg-[var(--bg-sidebar)] border border-[var(--border-strong)] rounded-xl shadow-2xl flex flex-col overflow-hidden animate-in fade-in duration-150"
      onclick={(e) => e.stopPropagation()}
      aria-hidden="true"
    >
      <div class="flex items-center justify-between px-5 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-sidebar)]">
        <div class="flex items-center gap-2.5">
          <kbd class="text-sm px-2.5 py-1 bg-[var(--bg-card)] border border-[var(--border-strong)] text-[var(--accent)] font-mono font-bold">
            {key.label}
          </kbd>
          <div>
            <h2 class="text-sm font-semibold text-[var(--text-primary)]">
              {shortcut ? 'Edit Key Action' : 'Assign Action to Key'}
            </h2>
            <span class="text-[11px] text-[var(--text-muted)] font-mono">
              Key: {key.base_key} ({key.hand} hand)
            </span>
          </div>
        </div>

        <button
          type="button"
          class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors"
          onclick={onClose}
          aria-label="Close modal"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="p-5 space-y-4 max-h-[75vh] overflow-y-auto">
        <div>
          <span class="text-xs font-semibold text-[var(--text-primary)] block mb-1.5">
            Key Modifiers
          </span>
          <div class="flex flex-wrap items-center gap-1.5">
            {#each Object.entries(availableModifiers) as [modId, def]}
              {@const isSelected = selectedModifiers.includes(modId)}
              <button
                type="button"
                class="flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-semibold border transition-all duration-150 {isSelected ? 'brightness-110 shadow-sm' : 'opacity-60 hover:opacity-100 bg-[var(--bg-card)] border-[var(--border-subtle)] text-[var(--text-primary)]'}"
                style={isSelected
                  ? `background-color: ${def.color}25; border-color: ${def.color}; color: ${def.color};`
                  : ''}
                onclick={() => toggleModifier(modId)}
              >
                <div class="w-2.5 h-2.5 rounded-full" style="background-color: {def.color};"></div>
                <span>{def.name}</span>
                {#if isSelected}
                  <Check class="w-3 h-3 ml-0.5" />
                {/if}
              </button>
            {/each}
          </div>

          <div class="mt-2 text-[11px] text-[var(--text-muted)] flex items-center gap-1.5">
            <span>Resulting Hotkey:</span>
            <div class="flex items-center gap-1 font-mono">
              {#each fullKeysString.split('+') as part}
                <kbd class="px-1.5 py-0.2 rounded bg-[var(--kbd-bg)] border border-[var(--kbd-border)] text-[var(--kbd-text)] font-semibold text-[10px]">
                  {part}
                </kbd>
              {/each}
            </div>
          </div>
        </div>

        <div>
          <label for="action-command-input" class="text-xs font-semibold text-[var(--text-primary)] block mb-1">
            Action Command / Title *
          </label>
          <input
            id="action-command-input"
            type="text"
            bind:value={command}
            placeholder="e.g. New Tab, Close Tab, Copy, Find..."
            class="w-full px-3 py-1.5 text-xs rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none"
          />
        </div>

        <div>
          <label for="action-description-input" class="text-xs font-semibold text-[var(--text-primary)] block mb-1">
            Description (Optional tooltip context)
          </label>
          <input
            id="action-description-input"
            type="text"
            bind:value={description}
            placeholder="e.g. Closes current active tab or buffer"
            class="w-full px-3 py-1.5 text-xs rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none"
          />
        </div>

        <div>
          <label for="action-category-input" class="text-xs font-semibold text-[var(--text-primary)] block mb-1">
            Category
          </label>
          <div class="flex items-center gap-2">
            <input
              id="action-category-input"
              type="text"
              bind:value={category}
              placeholder="e.g. Tabs & Navigation, Editing, View"
              class="flex-1 px-3 py-1.5 text-xs rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none"
            />
            {#if availableCategories.length > 0}
              <select
                aria-label="Preset category selector"
                class="px-2.5 py-1.5 text-xs rounded bg-[var(--bg-card)] border border-[var(--border-subtle)] text-[var(--text-primary)] focus:outline-none cursor-pointer"
                onchange={(e) => {
                  const val = (e.target as HTMLSelectElement).value;
                  if (val) category = val;
                }}
              >
                <option value="">Presets...</option>
                {#each availableCategories as cat}
                  <option value={cat}>{cat}</option>
                {/each}
              </select>
            {/if}
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between mb-1.5">
            <span class="text-xs font-semibold text-[var(--text-primary)]">Key Icon</span>
            {#if onOpenIconLibrary}
              <button
                type="button"
                class="text-[11px] text-[var(--accent)] hover:underline flex items-center gap-1"
                onclick={onOpenIconLibrary}
              >
                <Image class="w-3.5 h-3.5" />
                <span>Open Custom Icon Library</span>
              </button>
            {/if}
          </div>

          <div class="grid grid-cols-8 gap-1.5 p-2 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] max-h-36 overflow-y-auto">
            {#each PRESET_ICONS as iconItem}
              <button
                type="button"
                class="flex items-center justify-center p-2 rounded hover:bg-[var(--bg-card)] border transition-all aspect-square {icon === iconItem ? 'border-[var(--accent)] bg-[var(--bg-card-active)] text-[var(--accent)]' : 'border-transparent text-[var(--text-muted)]'}"
                onclick={() => (icon = iconItem)}
                title={iconItem}
                aria-label={`Select icon ${iconItem}`}
              >
                <KeyIcon iconName={iconItem} size={18} color="currentColor" />
              </button>
            {/each}
          </div>

          <div class="flex items-center gap-2 mt-2">
            <label for="action-icon-custom" class="text-[10px] text-[var(--text-muted)]">Selected:</label>
            <div class="flex items-center gap-1.5 px-2 py-0.5 rounded bg-[var(--bg-card)] border border-[var(--border-subtle)] text-xs text-[var(--text-primary)] font-mono">
              <KeyIcon iconName={icon} size={14} color="var(--accent)" />
              <span>{icon}</span>
            </div>
            <input
              id="action-icon-custom"
              type="text"
              bind:value={icon}
              placeholder="custom-icon-name"
              class="flex-1 px-2 py-0.5 text-xs rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-primary)] font-mono focus:border-[var(--accent)] focus:outline-none"
            />
          </div>
        </div>
      </div>

      <div class="flex items-center justify-between px-5 py-3 border-t border-[var(--border-subtle)] bg-[var(--bg-sidebar)]">
        {#if shortcut}
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1.5 text-xs rounded bg-rose-500/10 hover:bg-rose-500/20 text-rose-400 border border-rose-500/30 transition-colors"
            onclick={handleDelete}
          >
            <Trash2 class="w-3.5 h-3.5" />
            <span>Remove Action</span>
          </button>
        {:else}
          <div></div>
        {/if}

        <div class="flex items-center gap-2">
          <button
            type="button"
            class="px-3 py-1.5 text-xs rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors"
            onclick={onClose}
          >
            Cancel
          </button>
          <button
            type="button"
            class="flex items-center gap-1.5 px-4 py-1.5 text-xs rounded bg-[var(--accent)] text-[var(--accent-text)] font-semibold hover:brightness-110 transition-colors disabled:opacity-50"
            disabled={!command.trim() || isSaving}
            onclick={handleSave}
          >
            <Save class="w-3.5 h-3.5" />
            <span>{isSaving ? 'Saving...' : 'Save Action'}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
