<script lang="ts">
  import { Star, Check, Copy, CheckCheck, Pencil } from '@lucide/svelte';
  import type { ShortcutItem } from '$lib/types';

  let {
    shortcut,
    isFavorite = false,
    isLearned = false,
    onToggleFavorite,
    onToggleLearned,
    onEdit,
  }: {
    shortcut: ShortcutItem;
    isFavorite: boolean;
    isLearned: boolean;
    onToggleFavorite: (id: string) => void;
    onToggleLearned: (id: string) => void;
    onEdit?: (shortcut: ShortcutItem) => void;
  } = $props();

  let copied = $state(false);

  let keyTokens = $derived(
    shortcut.keys
      .split(/\s*(\+|\s)\s*/)
      .filter((t) => t.trim().length > 0)
  );

  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(`${shortcut.command} — ${shortcut.keys}`);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 1500);
    } catch {}
  }
</script>

<div
  class="group relative flex items-center justify-between gap-3 px-3 py-2 rounded-md bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border-subtle)] hover:border-[var(--border-strong)] transition-all duration-150 {isLearned ? 'opacity-50 hover:opacity-100' : ''}"
>
  <div class="flex items-center gap-2 min-w-0 flex-1">
    <button
      type="button"
      class="p-1 rounded text-[var(--text-muted)] hover:text-amber-400 hover:bg-[var(--bg-card-active)] transition-colors {isFavorite ? 'text-amber-400' : ''}"
      title={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
      onclick={() => onToggleFavorite(shortcut.id)}
    >
      <Star class="w-3.5 h-3.5 {isFavorite ? 'fill-amber-400' : ''}" />
    </button>

    <div class="flex flex-col min-w-0 flex-1">
      <span class="text-sm font-medium text-[var(--text-primary)] truncate select-text leading-tight">
        {shortcut.command}
      </span>
      <span class="text-[11px] text-[var(--text-muted)] truncate select-none">
        {shortcut.category}
      </span>
    </div>
  </div>

  <div class="flex items-center gap-2 shrink-0">
    <div class="flex items-center gap-1">
      {#each keyTokens as token}
        {#if token === '+'}
          <span class="text-[11px] text-[var(--text-muted)] font-mono">+</span>
        {:else}
          <kbd>{token}</kbd>
        {/if}
      {/each}
    </div>

    <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
      {#if onEdit}
        <button
          type="button"
          class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--accent)] hover:bg-[var(--bg-card-active)] transition-colors"
          title="Edit key action"
          onclick={() => onEdit(shortcut)}
        >
          <Pencil class="w-3.5 h-3.5" />
        </button>
      {/if}

      <button
        type="button"
        class="p-1 rounded text-[var(--text-muted)] hover:text-[var(--accent)] hover:bg-[var(--bg-card-active)] transition-colors"
        title={copied ? 'Copied!' : 'Copy shortcut'}
        onclick={handleCopy}
      >
        {#if copied}
          <CheckCheck class="w-3.5 h-3.5 text-emerald-400" />
        {:else}
          <Copy class="w-3.5 h-3.5" />
        {/if}
      </button>

      <button
        type="button"
        class="p-1 rounded text-[var(--text-muted)] hover:text-emerald-400 hover:bg-[var(--bg-card-active)] transition-colors {isLearned ? 'text-emerald-400' : ''}"
        title={isLearned ? 'Mark as unlearned' : 'Mark as learned'}
        onclick={() => onToggleLearned(shortcut.id)}
      >
        <Check class="w-3.5 h-3.5" />
      </button>
    </div>
  </div>
</div>
