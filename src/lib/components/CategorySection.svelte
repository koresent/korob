<script lang="ts">
  import { ChevronDown, ChevronRight } from '@lucide/svelte';
  import ShortcutCard from './ShortcutCard.svelte';
  import type { CategoryGroup, ShortcutItem } from '$lib/types';

  let {
    group,
    favorites = [],
    learned = [],
    isCollapsed = false,
    onToggleCollapse,
    onToggleFavorite,
    onToggleLearned,
    onEdit,
  }: {
    group: CategoryGroup;
    favorites: string[];
    learned: string[];
    isCollapsed: boolean;
    onToggleCollapse: (category: string) => void;
    onToggleFavorite: (id: string) => void;
    onToggleLearned: (id: string) => void;
    onEdit?: (shortcut: ShortcutItem) => void;
  } = $props();
</script>

<div class="flex flex-col gap-1.5">
  <button
    type="button"
    class="flex items-center gap-2 px-2 py-1 -mx-2 rounded hover:bg-[var(--bg-card-hover)] text-left transition-colors select-none group"
    onclick={() => onToggleCollapse(group.name)}
  >
    {#if isCollapsed}
      <ChevronRight class="w-3.5 h-3.5 text-[var(--text-muted)] group-hover:text-[var(--text-primary)] transition-transform" />
    {:else}
      <ChevronDown class="w-3.5 h-3.5 text-[var(--text-muted)] group-hover:text-[var(--text-primary)] transition-transform" />
    {/if}

    <span class="text-xs font-semibold uppercase tracking-wider text-[var(--text-muted)] group-hover:text-[var(--text-primary)] transition-colors">
      {group.name}
    </span>

    <span class="text-[11px] px-1.5 py-0.2 rounded-full bg-[var(--badge-bg)] text-[var(--badge-text)] font-medium">
      {group.items.length}
    </span>
  </button>

  {#if !isCollapsed}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-1.5 pl-2">
      {#each group.items as item (item.id)}
        <ShortcutCard
          shortcut={item}
          isFavorite={favorites.includes(item.id)}
          isLearned={learned.includes(item.id)}
          {onToggleFavorite}
          {onToggleLearned}
          {onEdit}
        />
      {/each}
    </div>
  {/if}
</div>
