<script lang="ts">
  import {
    X,
    Upload,
    Clipboard,
    Trash2,
    Image,
    Search,
    RefreshCw,
    FolderSync,
    Check,
  } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { IconInfo } from '$lib/types';
  import { onMount } from 'svelte';

  let {
    isOpen = false,
    usedIconNames = [],
    onClose,
    onSelectIcon,
  }: {
    isOpen: boolean;
    usedIconNames: string[];
    onClose: () => void;
    onSelectIcon?: (iconName: string) => void;
  } = $props();

  let icons = $state<IconInfo[]>([]);
  let searchQuery = $state('');
  let isLoading = $state(false);
  let statusMessage = $state('');

  async function loadIcons() {
    isLoading = true;
    try {
      icons = await invoke<IconInfo[]>('list_user_icons');
    } catch (e: any) {
      console.error('Failed to load icons:', e);
    } finally {
      isLoading = false;
    }
  }

  async function handlePickFile() {
    try {
      const selected = await invoke<string | null>('pick_icon_file');
      if (selected) {
        await invoke('import_icon_file', { sourcePath: selected });
        await loadIcons();
        statusMessage = 'Icon imported successfully!';
        setTimeout(() => (statusMessage = ''), 2500);
      }
    } catch (e: any) {
      console.error('Failed to import file:', e);
    }
  }

  async function handlePasteFromClipboard() {
    try {
      const items = await navigator.clipboard.read();
      for (const item of items) {
        const imageType = item.types.find((t) => t.startsWith('image/'));
        if (imageType) {
          const blob = await item.getType(imageType);
          const ext = imageType.split('/')[1] || 'png';
          const reader = new FileReader();
          reader.onload = async () => {
            const dataUrl = reader.result as string;
            await invoke('save_clipboard_icon', { dataBase64: dataUrl, ext });
            await loadIcons();
            statusMessage = 'Pasted icon from clipboard!';
            setTimeout(() => (statusMessage = ''), 2500);
          };
          reader.readAsDataURL(blob);
          return;
        }
      }
      statusMessage = 'No image found on clipboard';
      setTimeout(() => (statusMessage = ''), 2500);
    } catch (e: any) {
      console.error('Failed to read clipboard:', e);
      statusMessage = 'Could not read clipboard';
      setTimeout(() => (statusMessage = ''), 2500);
    }
  }

  async function handleDeleteIcon(fileName: string) {
    try {
      await invoke('delete_user_icon', { fileName });
      await loadIcons();
    } catch (e) {
      console.error('Failed to delete icon:', e);
    }
  }

  async function handleCleanUnused() {
    isLoading = true;
    try {
      const deleted = await invoke<string[]>('clean_unused_icons', { usedIconNames });
      await loadIcons();
      statusMessage = `Cleaned ${deleted.length} unused icons`;
      setTimeout(() => (statusMessage = ''), 3000);
    } catch (e: any) {
      console.error('Failed to clean unused icons:', e);
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!isOpen) return;
    if (e.key === 'Escape') {
      onClose();
    }
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'v') {
      handlePasteFromClipboard();
    }
  }

  $effect(() => {
    if (isOpen) {
      loadIcons();
    }
  });

  const filteredIcons = $derived(
    icons.filter((icon) => {
      if (!searchQuery.trim()) return true;
      const q = searchQuery.toLowerCase();
      return icon.name.toLowerCase().includes(q) || icon.ext.includes(q);
    })
  );
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    onclick={onClose}
    aria-hidden="true"
  >
    <div
      class="w-full max-w-2xl bg-[var(--bg-sidebar)] border border-[var(--border-strong)] rounded-xl shadow-2xl flex flex-col max-h-[85vh] overflow-hidden"
      onclick={(e) => e.stopPropagation()}
      aria-hidden="true"
    >
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-[var(--border-subtle)]">
        <div class="flex items-center gap-2">
          <Image class="w-4 h-4 text-[var(--accent)]" />
          <h2 class="text-sm font-semibold text-[var(--text-primary)]">Custom Icon Library</h2>
          <span class="text-xs text-[var(--text-muted)] font-mono">({icons.length} icons)</span>
        </div>

        <button
          type="button"
          class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors"
          onclick={onClose}
          aria-label="Close icon library"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="flex items-center justify-between gap-3 px-5 py-2.5 bg-[var(--bg-card)] border-b border-[var(--border-subtle)] text-xs">
        <div class="relative flex-1 max-w-xs">
          <Search class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-[var(--text-muted)]" />
          <input
            type="text"
            placeholder="Search icons..."
            aria-label="Search icons"
            bind:value={searchQuery}
            class="w-full pl-8 pr-3 py-1 rounded bg-[var(--bg-base)] text-[var(--text-primary)] placeholder-[var(--text-muted)] border border-[var(--border-subtle)] focus:border-[var(--accent)] focus:outline-none"
          />
        </div>

        <div class="flex items-center gap-2">
          <button
            type="button"
            class="flex items-center gap-1.5 px-2.5 py-1 rounded bg-[var(--bg-base)] hover:bg-[var(--bg-card-hover)] border border-[var(--border-subtle)] text-[var(--text-primary)] transition-colors"
            onclick={handlePasteFromClipboard}
            title="Paste image directly from clipboard (Ctrl+V)"
          >
            <Clipboard class="w-3.5 h-3.5 text-[var(--accent)]" />
            <span>Paste (Ctrl+V)</span>
          </button>

          <button
            type="button"
            class="flex items-center gap-1.5 px-2.5 py-1 rounded bg-[var(--bg-base)] hover:bg-[var(--bg-card-hover)] border border-[var(--border-subtle)] text-[var(--text-primary)] transition-colors"
            onclick={handlePickFile}
            title="Import SVG / PNG image from disk"
          >
            <Upload class="w-3.5 h-3.5" />
            <span>Import File</span>
          </button>

          <button
            type="button"
            class="flex items-center gap-1.5 px-2.5 py-1 rounded hover:bg-rose-500/10 hover:border-rose-500/30 border border-transparent text-[var(--text-muted)] hover:text-rose-400 transition-colors"
            onclick={handleCleanUnused}
            title="Delete custom icons not used in any shortcut"
          >
            <Trash2 class="w-3.5 h-3.5" />
            <span>Clean Unused</span>
          </button>
        </div>
      </div>

      {#if statusMessage}
        <div class="px-5 py-1.5 bg-[var(--accent)]/10 text-[var(--accent)] text-xs flex items-center gap-1.5">
          <Check class="w-3 h-3" />
          <span>{statusMessage}</span>
        </div>
      {/if}

      <div class="flex-1 overflow-y-auto p-5">
        {#if filteredIcons.length === 0}
          <div class="flex flex-col items-center justify-center py-12 text-center space-y-2 text-[var(--text-muted)]">
            <Image class="w-8 h-8 opacity-40" />
            <p class="text-xs">No custom icons found.</p>
            <p class="text-[11px] max-w-sm">
              Press <kbd class="px-1 py-0.5 rounded bg-[var(--bg-card)] border border-[var(--border-subtle)] font-mono">Ctrl+V</kbd> to paste an image directly from your clipboard or click "Import File".
            </p>
          </div>
        {:else}
          <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 gap-3">
            {#each filteredIcons as icon (icon.file_name)}
              <div
                class="group relative flex flex-col items-center justify-center p-2.5 rounded-lg bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border-subtle)] hover:border-[var(--accent)] transition-all cursor-pointer aspect-square"
                onclick={() => onSelectIcon?.(icon.file_name)}
                onkeydown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    onSelectIcon?.(icon.file_name);
                  }
                }}
                title="{icon.name} ({icon.ext.toUpperCase()}, {Math.round(icon.size_bytes / 1024)} KB)"
                role="button"
                tabindex="0"
              >
                <img
                  src="tauri://localhost/{icon.path}"
                  alt={icon.name}
                  class="w-8 h-8 object-contain pointer-events-none filter drop-shadow"
                  onerror={(e) => {
                    invoke<string>('read_icon_base64', { name: icon.file_name }).then((data) => {
                      (e.currentTarget as HTMLImageElement).src = data;
                    });
                  }}
                />

                <span class="text-[10px] text-[var(--text-muted)] truncate w-full text-center mt-1.5 font-mono">
                  {icon.name}
                </span>

                <button
                  type="button"
                  class="absolute top-1 right-1 p-1 rounded bg-black/60 text-white/70 hover:text-rose-400 opacity-0 group-hover:opacity-100 transition-opacity"
                  title="Delete icon"
                  aria-label={`Delete icon ${icon.name}`}
                  onclick={(e) => {
                    e.stopPropagation();
                    handleDeleteIcon(icon.file_name);
                  }}
                >
                  <Trash2 class="w-3 h-3" />
                </button>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <div class="px-5 py-2.5 border-t border-[var(--border-subtle)] text-[11px] text-[var(--text-muted)] flex items-center justify-between bg-[var(--bg-card)]">
        <span>Storage: <code class="text-[var(--text-primary)]">~/.local/share/korob/icons</code></span>
        <span>Supports SVG, PNG, WebP</span>
      </div>
    </div>
  </div>
{/if}
