<script lang="ts">
  import { theme, type Theme } from '$lib/store/theme.svelte'
  import { Button } from '$lib/components/ui/button'
  import { Sun, Moon, Monitor, Check } from '@lucide/svelte'

  const options: { value: Theme; label: string }[] = [
    { value: 'light',  label: 'Light'  },
    { value: 'dark',   label: 'Dark'   },
    { value: 'system', label: 'System' },
  ]

  let open = $state(false)

  function select(t: Theme) {
    theme.set(t)
    open = false
  }
</script>

<svelte:window
  onkeydown={(e) => { if (e.key === 'Escape') open = false }}
  onclick={(e) => {
    if (!(e.target as HTMLElement).closest('.theme-toggle-root')) open = false
  }}
/>

<div class="theme-toggle-root relative">
  <Button
    variant="ghost"
    size="icon"
    onclick={() => open = !open}
    aria-label="Toggle theme"
    aria-expanded={open}
  >
    {#if theme.isDark}
      <Moon size={16} />
    {:else}
      <Sun size={16} />
    {/if}
  </Button>

  {#if open}
    <div class="absolute right-0 top-full mt-1 z-50 min-w-[8rem] rounded-md border border-border bg-popover p-1 shadow-md">
      {#each options as opt}
        <button
          onclick={() => select(opt.value)}
          class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm text-popover-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
        >
          {#if opt.value === 'light'}
            <Sun size={14} />
          {:else if opt.value === 'dark'}
            <Moon size={14} />
          {:else}
            <Monitor size={14} />
          {/if}

          {opt.label}

          {#if theme.current === opt.value}
            <Check size={14} class="ml-auto" />
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>
