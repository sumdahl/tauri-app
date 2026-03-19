<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components/ui/button';
  import { Typography } from '$lib/components/ui/typography';
  import { ThemeToggle } from '$lib/components/ui/theme-toggle';
  import { authStore } from '$lib/store/auth.svelte';
  import { TrendingUp, BarChart2, Zap, Shield, ArrowRight, LineChart } from '@lucide/svelte';
  import { onMount } from 'svelte';

  // Redirect if already authenticated
  onMount(() => {
    if (authStore.isAuthenticated) {
      goto('/dashboard');
    }
  });

  const features = [
    {
      icon: TrendingUp,
      title: 'Real-time Market Data',
      description: 'Live stock, crypto, and forex feeds via WebSocket — zero latency.',
    },
    {
      icon: BarChart2,
      title: 'Multi-asset Coverage',
      description: 'Stocks, ETFs, crypto pairs, and 140+ forex tickers in one place.',
    },
    {
      icon: Zap,
      title: 'Instant Execution',
      description: 'Native desktop performance. No browser overhead. Pure speed.',
    },
    {
      icon: Shield,
      title: 'Secure by Design',
      description: 'Your API keys and credentials never leave your machine.',
    },
  ];

  // Animated ticker data for the visual side
  const tickers = [
    { symbol: 'AAPL', price: '191.24', change: '+1.23', gain: true },
    { symbol: 'BTC', price: '42,850.00', change: '+2.41', gain: true },
    { symbol: 'TSLA', price: '238.45', change: '-0.87', gain: false },
    { symbol: 'ETH', price: '2,284.10', change: '+3.12', gain: true },
    { symbol: 'EUR/USD', price: '1.0851', change: '-0.02', gain: false },
    { symbol: 'MSFT', price: '415.32', change: '+0.94', gain: true },
  ];
</script>

<div class="min-h-screen bg-background flex flex-col">
  <!-- Navbar -->
  <header class="border-b border-border px-6 py-4 flex items-center justify-between">
    <div class="flex items-center gap-2">
      <LineChart size={22} class="text-primary" />
      <span class="font-semibold text-foreground tracking-tight">Marketflow</span>
    </div>
    <div class="flex items-center gap-3">
      <ThemeToggle />
      <Button variant="ghost" onclick={() => goto('/login')}>Sign in</Button>
      <Button onclick={() => goto('/register')}>Get started</Button>
    </div>
  </header>

  <!-- Hero -->
  <main class="flex-1 flex items-center px-6 py-16 max-w-7xl mx-auto w-full gap-16">
    <!-- Left: Text -->
    <div class="flex-1 flex flex-col gap-6 max-w-xl">
      <div class="badge-info w-fit">
        Live market data · Stocks · Crypto · Forex
      </div>

      <Typography variant="h1" class="text-foreground">
        Markets at your fingertips,<br />
        <span class="text-primary">in real time.</span>
      </Typography>

      <Typography variant="lead">
        A native desktop terminal for traders and developers.
        Stream live prices, track portfolios, and analyze fundamentals —
        all from one fast, secure app.
      </Typography>

      <div class="flex items-center gap-3 pt-2">
        <Button size="lg" onclick={() => goto('/register')} class="gap-2">
          Start for free
          <ArrowRight size={16} />
        </Button>
        <Button size="lg" variant="outline" onclick={() => goto('/login')}>
          Sign in
        </Button>
      </div>

      <!-- Feature grid -->
      <div class="grid grid-cols-2 gap-4 pt-4">
        {#each features as feature}
          <div class="flex items-start gap-3 p-3 rounded-lg border border-border bg-card">
            <div class="mt-0.5 text-primary shrink-0">
              <feature.icon size={16} />
            </div>
            <div>
              <Typography variant="small" class="text-foreground">{feature.title}</Typography>
              <Typography variant="muted" class="mt-0.5">{feature.description}</Typography>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Right: Live ticker visual -->
    <div class="flex-1 max-w-md hidden lg:flex flex-col gap-3">
      <div class="rounded-xl border border-border bg-card p-5 shadow-sm">
        <div class="flex items-center justify-between mb-4">
          <Typography variant="small" class="text-foreground">Live Feed</Typography>
          <span class="flex items-center gap-1.5 text-xs text-gain">
            <span class="w-1.5 h-1.5 rounded-full bg-gain animate-pulse"></span>
            Live
          </span>
        </div>

        <div class="flex flex-col gap-2">
          {#each tickers as ticker}
            <div
              class="flex items-center justify-between px-3 py-2.5 rounded-lg bg-background border border-border"
            >
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-md bg-muted flex items-center justify-center">
                  <Typography variant="small" class="text-foreground tabular-nums text-[10px]">
                    {ticker.symbol.slice(0, 3)}
                  </Typography>
                </div>
                <Typography variant="small" class="text-foreground">{ticker.symbol}</Typography>
              </div>
              <div class="flex items-center gap-3">
                <span class="tabular-nums text-sm font-medium text-foreground" data-price>
                  ${ticker.price}
                </span>
                <span class={ticker.gain ? 'badge-gain' : 'badge-loss'}>
                  {ticker.gain ? '+' : ''}{ticker.change}%
                </span>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <!-- Stats row -->
      <div class="grid grid-cols-3 gap-3">
        <div class="rounded-lg border border-border bg-card p-3 text-center">
          <Typography variant="large" class="text-primary">8k+</Typography>
          <Typography variant="muted">Crypto pairs</Typography>
        </div>
        <div class="rounded-lg border border-border bg-card p-3 text-center">
          <Typography variant="large" class="text-primary">140+</Typography>
          <Typography variant="muted">Forex tickers</Typography>
        </div>
        <div class="rounded-lg border border-border bg-card p-3 text-center">
          <Typography variant="large" class="text-primary">EOD</Typography>
          <Typography variant="muted">Stock data</Typography>
        </div>
      </div>
    </div>
  </main>
</div>