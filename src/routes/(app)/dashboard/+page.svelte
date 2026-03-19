<script lang="ts">
  import { Typography } from '$lib/components/ui/typography';
  import { Button } from '$lib/components/ui/button';
  import { ThemeToggle } from '$lib/components/ui/theme-toggle';
  import { authStore } from '$lib/store/auth.svelte';
  import { signout } from '$lib/service/auth';
  import { LineChart } from '@lucide/svelte';
</script>

<div class="min-h-screen bg-background">
  <header class="border-b border-border px-6 py-4 flex items-center justify-between">
    <div class="flex items-center gap-2">
      <LineChart size={22} class="text-primary" />
      <span class="font-semibold text-foreground tracking-tight">Marketflow</span>
    </div>
    <div class="flex items-center gap-3">
      <Typography variant="small" class="text-muted-foreground">
        {authStore.user?.email}
      </Typography>
      <ThemeToggle />
      <Button variant="outline" size="sm" onclick={signout}>Sign out</Button>
    </div>
  </header>

  <main class="p-6">
    <Typography variant="h2" class="text-foreground">Dashboard</Typography>
    <Typography variant="muted" class="mt-1">Market data coming soon.</Typography>
  </main>
</div>