<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<div class="h-screen w-screen overflow-hidden bg-[var(--bg-base)] text-[var(--text-primary)] antialiased select-none font-sans flex flex-col border border-[var(--border-strong)] rounded-lg shadow-2xl">
  {@render children?.()}
</div>
