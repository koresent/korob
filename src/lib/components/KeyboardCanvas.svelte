<script lang="ts">
  import type { KeyboardLayout, LayoutKey, ShortcutItem, ModifierDef } from '$lib/types';
  import KeyIcon from './KeyIcon.svelte';
  import { Check } from '@lucide/svelte';

  let {
    layout,
    shortcuts = [],
    modifiers = {},
    searchQuery = '',
    onSelectKey,
  }: {
    layout: KeyboardLayout | null;
    shortcuts: ShortcutItem[];
    modifiers: Record<string, ModifierDef>;
    searchQuery: string;
    onSelectKey?: (key: LayoutKey, matchedShortcut?: ShortcutItem) => void;
  } = $props();

  let hoveredKey = $state<LayoutKey | null>(null);
  let hoveredShortcut = $state<ShortcutItem | null>(null);
  let tooltipX = $state(0);
  let tooltipY = $state(0);
  let copiedId = $state<string | null>(null);

  // Map shortcuts by base_key for O(1) lookup
  const shortcutMap = $derived.by(() => {
    const map = new Map<string, ShortcutItem>();
    for (const s of shortcuts) {
      const bk = s.base_key.toLowerCase();
      if (!map.has(bk)) {
        map.set(bk, s);
      }
    }
    return map;
  });

  function getModifierColor(mod: string): string {
    const clean = mod.toLowerCase();
    if (modifiers[clean]) {
      return modifiers[clean].color;
    }
    if (clean === 'win' || clean === 'cmd' || clean === 'meta' || clean === 'gui') {
      return modifiers['super']?.color || '#ef4444';
    }
    return '#a3a3a3';
  }

  function getGradientId(item: ShortcutItem): string {
    return `grad_${item.id.replace(/[^a-zA-Z0-9]/g, '_')}`;
  }

  function getKeyStyling(key: LayoutKey) {
    const bk = key.base_key.toLowerCase();
    const lbl = key.label.toLowerCase();

    // Check if key itself is a physical modifier
    for (const [modId, def] of Object.entries(modifiers)) {
      if (lbl === modId || bk === modId || lbl === def.name.toLowerCase()) {
        return { isModifierKey: true, color: def.color, gradientId: '' };
      }
    }
    if (lbl === 'ctrl' || bk === 'ctrl') {
      return { isModifierKey: true, color: modifiers['ctrl']?.color || '#facc15', gradientId: '' };
    }
    if (lbl === 'alt' || bk === 'alt') {
      return { isModifierKey: true, color: modifiers['alt']?.color || '#c084fc', gradientId: '' };
    }
    if (lbl === 'shift' || bk === 'shift') {
      return { isModifierKey: true, color: modifiers['shift']?.color || '#38bdf8', gradientId: '' };
    }
    if (lbl === 'super' || bk === 'super') {
      return { isModifierKey: true, color: modifiers['super']?.color || '#ef4444', gradientId: '' };
    }

    // Check mapped shortcut
    const shortcut = shortcutMap.get(bk);
    if (!shortcut) {
      return { isModifierKey: false, color: '', gradientId: '' };
    }

    if (shortcut.modifiers.length > 1) {
      return {
        isModifierKey: false,
        color: '',
        gradientId: getGradientId(shortcut),
      };
    } else if (shortcut.modifiers.length === 1) {
      return {
        isModifierKey: false,
        color: getModifierColor(shortcut.modifiers[0]),
        gradientId: '',
      };
    }

    return { isModifierKey: false, color: modifiers['ctrl']?.color || '#facc15', gradientId: '' };
  }

  function isKeyFiltered(key: LayoutKey): boolean {
    if (!searchQuery.trim()) return false;
    const q = searchQuery.toLowerCase();
    const shortcut = shortcutMap.get(key.base_key.toLowerCase());
    if (!shortcut) return true;
    return !(
      shortcut.command.toLowerCase().includes(q) ||
      shortcut.keys.toLowerCase().includes(q) ||
      shortcut.category.toLowerCase().includes(q) ||
      key.label.toLowerCase().includes(q)
    );
  }

  function handleMouseEnter(key: LayoutKey, e: MouseEvent) {
    hoveredKey = key;
    hoveredShortcut = shortcutMap.get(key.base_key.toLowerCase()) || null;
    const rect = (e.currentTarget as SVGElement).getBoundingClientRect();
    tooltipX = rect.left + rect.width / 2;
    tooltipY = rect.top - 12;
  }

  function handleMouseLeave() {
    hoveredKey = null;
    hoveredShortcut = null;
  }

  async function handleKeyClick(key: LayoutKey) {
    const sc = shortcutMap.get(key.base_key.toLowerCase());
    if (sc) {
      try {
        await navigator.clipboard.writeText(`${sc.command} — ${sc.keys}`);
        copiedId = sc.id;
        setTimeout(() => {
          copiedId = null;
        }, 1500);
      } catch {}
    }
    onSelectKey?.(key, sc);
  }
</script>

<div class="relative w-full h-full flex flex-col items-center justify-center p-3 select-none overflow-hidden">
  {#if !layout}
    <div class="text-sm text-[var(--text-muted)] animate-pulse">Loading keyboard layout...</div>
  {:else}
    <svg
      viewBox="0 0 {layout.canvas.width} {layout.canvas.height}"
      class="w-full max-h-[500px] h-auto object-contain filter drop-shadow-xl"
      preserveAspectRatio="xMidYMid meet"
    >
      <defs>
        {#each shortcuts as item (item.id)}
          {#if item.modifiers.length > 1}
            <linearGradient id={getGradientId(item)} x1="0%" y1="0%" x2="100%" y2="100%">
              {#each item.modifiers as mod, idx}
                <stop
                  offset="{(idx / (item.modifiers.length - 1)) * 100}%"
                  stop-color={getModifierColor(mod)}
                />
              {/each}
            </linearGradient>
          {/if}
        {/each}
      </defs>

      {#each layout.keys as key (key.id)}
        {@const matched = shortcutMap.get(key.base_key.toLowerCase())}
        {@const styling = getKeyStyling(key)}
        {@const isFiltered = isKeyFiltered(key)}
        {@const isHovered = hoveredKey?.id === key.id}
        {@const hasStroke = styling.color || styling.gradientId}
        {@const strokeVal = styling.gradientId ? `url(#${styling.gradientId})` : styling.color || 'var(--border-subtle)'}

        <g
          transform="translate({key.x}, {key.y}) rotate({key.rotation})"
          class="cursor-pointer transition-all duration-150 {isFiltered ? 'opacity-20' : 'opacity-100'}"
          onmouseenter={(e) => handleMouseEnter(key, e)}
          onmouseleave={handleMouseLeave}
          onclick={() => handleKeyClick(key)}
          onkeydown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              handleKeyClick(key);
            }
          }}
          role="button"
          tabindex="0"
          aria-label="{key.label} {matched ? matched.command : ''}"
        >
          <rect
            x={-key.width / 2}
            y={-key.height / 2}
            width={key.width}
            height={key.height}
            rx="10"
            ry="10"
            fill="var(--bg-card)"
            stroke={hasStroke ? strokeVal : 'var(--border-subtle)'}
            stroke-width={hasStroke ? (isHovered ? '2.5' : '1.8') : '1'}
            class="transition-colors duration-150 {isHovered ? 'brightness-125' : ''}"
          />

          {#if matched}
            <g transform="translate({-11}, {-11})">
              <KeyIcon
                iconName={matched.icon || 'key-round'}
                size={22}
                color={styling.color || 'currentColor'}
                gradientId={styling.gradientId}
              />
            </g>

            <text
              x={key.width / 2 - 7}
              y={key.height / 2 - 6}
              font-size="8.5"
              font-family="monospace"
              font-weight="bold"
              text-anchor="end"
              fill="var(--text-muted)"
              opacity="0.6"
              class="pointer-events-none"
            >
              {key.label}
            </text>
          {:else}
            <text
              x="0"
              y="4.5"
              font-size={key.label.length > 4 ? '9' : '12'}
              font-family="sans-serif"
              font-weight={styling.isModifierKey ? 'bold' : '500'}
              text-anchor="middle"
              fill={styling.isModifierKey ? styling.color : 'var(--text-primary)'}
              class="pointer-events-none"
            >
              {key.label}
            </text>
          {/if}
        </g>
      {/each}
    </svg>

    {#if hoveredKey && hoveredShortcut}
      {@const styling = getKeyStyling(hoveredKey)}
      <div
        class="fixed z-50 pointer-events-none -translate-x-1/2 -translate-y-full mb-3 px-3.5 py-2.5 rounded-lg bg-[var(--bg-sidebar)] border border-[var(--border-strong)] shadow-2xl backdrop-blur-md flex flex-col gap-1 min-w-[200px] max-w-xs transition-opacity duration-150"
        style="left: {tooltipX}px; top: {tooltipY}px;"
      >
        <div class="flex items-center justify-between gap-2">
          <div class="flex items-center gap-1.5">
            <KeyIcon
              iconName={hoveredShortcut.icon || 'key-round'}
              size={16}
              color={styling.color || 'currentColor'}
              gradientId={styling.gradientId}
            />
            <span class="text-xs font-semibold text-[var(--text-primary)] leading-tight">
              {hoveredShortcut.command}
            </span>
          </div>

          <span class="text-[9px] px-1.5 py-0.5 rounded bg-[var(--badge-bg)] text-[var(--badge-text)] uppercase font-semibold">
            {hoveredShortcut.category}
          </span>
        </div>

        {#if hoveredShortcut.description}
          <p class="text-[11px] text-[var(--text-muted)] leading-relaxed">
            {hoveredShortcut.description}
          </p>
        {/if}

        <div class="flex items-center justify-between gap-2 pt-1 border-t border-[var(--border-subtle)] text-[10px] text-[var(--text-muted)]">
          <div class="flex items-center gap-1">
            {#each hoveredShortcut.keys.split('+') as part}
              <kbd class="px-1.5 py-0.5 rounded bg-[var(--kbd-bg)] border border-[var(--kbd-border)] text-[var(--kbd-text)] font-mono font-semibold">
                {part.trim()}
              </kbd>
            {/each}
          </div>

          {#if copiedId === hoveredShortcut.id}
            <span class="flex items-center gap-1 text-emerald-400 font-medium">
              <Check class="w-3 h-3" /> Copied!
            </span>
          {:else}
            <span class="text-[9px] opacity-75">Click to copy</span>
          {/if}
        </div>
      </div>
    {/if}
  {/if}
</div>
