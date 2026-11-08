<script lang="ts">
  import {
    Folder,
    Pin,
    RotateCw,
    History,
    Plus,
    Redo,
    Code,
    BookOpen,
    Printer,
    Download,
    Bookmark,
    Search,
    KeyRound,
    Clock,
    ExternalLink,
    Undo2,
    Scissors,
    Copy,
    Clipboard,
    VenetianMask,
    VolumeX,
    ZoomIn,
    ZoomOut,
    Maximize2,
    File,
    Terminal,
    Columns2,
    Save,
    CheckSquare,
    X,
    Layers,
    SquareCode,
  } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';

  let {
    iconName = '',
    size = 20,
    color = 'currentColor',
    gradientId = '',
  }: {
    iconName?: string;
    size?: number;
    color?: string;
    gradientId?: string;
  } = $props();

  let customImageData = $state<string | null>(null);

  $effect(() => {
    if (!iconName) {
      customImageData = null;
      return;
    }
    if (iconName.startsWith('data:image')) {
      customImageData = iconName;
      return;
    }
    // Check if it's a custom user icon file from library
    if (iconName.includes('.') || iconName.startsWith('icon_')) {
      invoke<string>('read_icon_base64', { name: iconName })
        .then((data) => {
          customImageData = data;
        })
        .catch(() => {
          customImageData = null;
        });
    } else {
      customImageData = null;
    }
  });

  const strokeAttr = $derived(gradientId ? `url(#${gradientId})` : color);
</script>

{#if customImageData}
  <img
    src={customImageData}
    alt="Custom Icon"
    class="object-contain pointer-events-none"
    style="width: {size}px; height: {size}px;"
  />
{:else if iconName.startsWith('folder-')}
  {@const tabNum = iconName.replace('folder-', '')}
  <svg
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke={strokeAttr}
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class="pointer-events-none shrink-0"
  >
    <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
    {#if tabNum === 'x'}
      <line x1="9.5" y1="11.5" x2="14.5" y2="16.5" />
      <line x1="14.5" y1="11.5" x2="9.5" y2="16.5" />
    {:else}
      <text
        x="12"
        y="15.5"
        text-anchor="middle"
        font-size="7.5"
        font-weight="bold"
        fill={strokeAttr}
        stroke="none"
        font-family="sans-serif"
      >
        {tabNum}
      </text>
    {/if}
  </svg>
{:else if iconName === 'folder'}
  <Folder size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'pin'}
  <Pin size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'rotate-cw'}
  <RotateCw size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'history'}
  <History size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'plus'}
  <Plus size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'redo'}
  <Redo size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'code'}
  <Code size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'book-open'}
  <BookOpen size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'printer'}
  <Printer size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'download' || iconName === 'download-cloud'}
  <Download size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'bookmark'}
  <Bookmark size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'search'}
  <Search size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'key-round'}
  <KeyRound size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'clock'}
  <Clock size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'external-link'}
  <ExternalLink size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'undo-2' || iconName === 'undo'}
  <Undo2 size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'scissors'}
  <Scissors size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'copy'}
  <Copy size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'clipboard'}
  <Clipboard size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'venetian-mask'}
  <VenetianMask size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'volume-x'}
  <VolumeX size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'zoom-in'}
  <ZoomIn size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'zoom-out'}
  <ZoomOut size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'maximize-2'}
  <Maximize2 size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'file'}
  <File size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'terminal'}
  <Terminal size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'columns-2'}
  <Columns2 size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'save'}
  <Save size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'check-square'}
  <CheckSquare size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName === 'x'}
  <X size={size} stroke={strokeAttr} strokeWidth={2} />
{:else if iconName}
  <KeyRound size={size} stroke={strokeAttr} strokeWidth={2} />
{/if}
