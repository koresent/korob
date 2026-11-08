<script lang="ts">
  import { X, Save, Trash2, FileText, Check } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  let {
    isOpen = false,
    currentAppId = 'general',
    onClose,
    onSaved,
  }: {
    isOpen: boolean;
    currentAppId: string;
    onClose: () => void;
    onSaved: () => void;
  } = $props();

  let appId = $state('');
  let rawContent = $state('');
  let isLoading = $state(false);
  let statusMessage = $state('');

  $effect(() => {
    if (isOpen) {
      appId = currentAppId.trim().replace(/\.yaml$/, '');
      loadFileContent(appId);
    }
  });

  async function loadFileContent(id: string) {
    isLoading = true;
    statusMessage = '';
    try {
      rawContent = await invoke<string>('read_shortcut_yaml_raw', { appId: id });
    } catch {
      // If file doesn't exist, provide a helpful YAML template
      rawContent = `app_id: "${id}"
name: "${id}"
description: "${id} Hotkeys"
shortcuts:
  - id: "${id}-example-1"
    command: "Example Action"
    description: "Detailed description of action"
    keys: "ctrl+t"
    category: "General"
    icon: "plus"
`;
    } finally {
      isLoading = false;
    }
  }

  async function handleSave() {
    if (!appId.trim()) return;
    statusMessage = 'Saving...';
    try {
      await invoke('save_shortcut_yaml_raw', {
        appId: appId.trim(),
        content: rawContent,
      });
      statusMessage = 'Saved YAML successfully!';
      setTimeout(() => {
        statusMessage = '';
      }, 1500);
      onSaved();
    } catch (e: any) {
      statusMessage = `Error saving: ${e?.message || e}`;
    }
  }

  async function handleDelete() {
    if (!confirm(`Are you sure you want to delete ${appId}.yaml?`)) return;
    try {
      await invoke('delete_shortcut_file', { appId });
      onSaved();
      onClose();
    } catch (e: any) {
      statusMessage = `Error deleting: ${e?.message || e}`;
    }
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-xs"
    onclick={onClose}
    aria-hidden="true"
  >
    <div
      class="w-full max-w-3xl h-[560px] max-h-[85vh] flex flex-col rounded-xl bg-[var(--bg-sidebar)] border border-[var(--border-strong)] shadow-2xl overflow-hidden"
      onclick={(e) => e.stopPropagation()}
      aria-hidden="true"
    >
      <div class="flex items-center justify-between px-5 py-3 bg-[var(--bg-sidebar)] border-b border-[var(--border-subtle)]">
        <div class="flex items-center gap-2">
          <FileText class="w-4 h-4 text-[var(--accent)]" />
          <h2 class="text-sm font-semibold text-[var(--text-primary)]">Edit YAML Cheatsheet</h2>
          <span class="text-xs text-[var(--text-muted)] font-mono">({appId}.yaml)</span>
        </div>

        <button
          type="button"
          class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-card)] transition-colors"
          onclick={onClose}
          aria-label="Close editor"
        >
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="flex items-center gap-2 px-5 py-2.5 bg-[var(--bg-card)] border-b border-[var(--border-subtle)] text-xs">
        <label for="editor-appid-input" class="text-[var(--text-muted)] shrink-0 font-medium">App ID:</label>
        <input
          id="editor-appid-input"
          type="text"
          bind:value={appId}
          onchange={() => loadFileContent(appId)}
          placeholder="e.g. zen, ghostty, antigravity"
          class="px-2.5 py-1 rounded bg-[var(--bg-base)] border border-[var(--border-subtle)] text-[var(--text-primary)] font-mono text-xs focus:border-[var(--accent)] focus:outline-none min-w-[200px]"
        />
        <span class="text-[11px] text-[var(--text-muted)] ml-auto font-mono">~/.config/korob/shortcuts/{appId}.yaml</span>
      </div>

      <div class="flex-1 min-h-0 p-4 bg-[var(--bg-base)] flex flex-col">
        <textarea
          bind:value={rawContent}
          spellcheck="false"
          placeholder="Write YAML here..."
          aria-label="YAML cheatsheet content"
          class="flex-1 min-h-0 w-full p-3 font-mono text-xs leading-relaxed bg-[var(--bg-card)] border border-[var(--border-subtle)] rounded-lg text-[var(--text-primary)] focus:border-[var(--accent)] focus:outline-none resize-none overflow-y-auto selection:bg-[var(--accent)] selection:text-[var(--accent-text)]"
        ></textarea>
      </div>

      <div class="flex items-center justify-between px-4 py-2.5 bg-[var(--bg-sidebar)] border-t border-[var(--border-subtle)]">
        <div class="flex items-center gap-2">
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1 text-xs rounded bg-rose-500/10 hover:bg-rose-500/20 text-rose-400 border border-rose-500/30 transition-colors"
            onclick={handleDelete}
          >
            <Trash2 class="w-3.5 h-3.5" />
            <span>Delete</span>
          </button>
          {#if statusMessage}
            <span class="text-xs text-[var(--accent)] font-medium flex items-center gap-1">
              <Check class="w-3.5 h-3.5" /> {statusMessage}
            </span>
          {/if}
        </div>

        <div class="flex items-center gap-2">
          <button
            type="button"
            class="px-3 py-1 text-xs rounded text-[var(--text-muted)] hover:text-[var(--text-primary)] transition-colors"
            onclick={onClose}
          >
            Cancel
          </button>
          <button
            type="button"
            class="flex items-center gap-1.5 px-3 py-1 text-xs rounded bg-[var(--accent)] text-[var(--accent-text)] font-medium hover:brightness-110 transition-colors"
            onclick={handleSave}
          >
            <Save class="w-3.5 h-3.5" />
            <span>Save YAML</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
