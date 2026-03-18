<Typography variant="h3">Welcome back</Typography>
<Typography variant="muted">Sign in to your account</Typography>
 <script lang="ts">
  import { Button } from '$lib/components/ui/button'
  import { ThemeToggle } from '$lib/components/ui/theme-toggle'
  import { Label } from '$lib/components/ui/label'
  import { Typography } from '$lib/components/ui/typography'
</script>

<!-- Theme toggle fixed to top-right -->
<div class="fixed top-4 right-4 z-50">
  <ThemeToggle />
</div>

<main class="min-h-screen bg-background flex items-center justify-center px-4">
  <div class="w-full max-w-sm space-y-6">

    <!-- Logo + heading -->
    <div class="space-y-2 text-center">
      <div class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-primary mb-2">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none"
          stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="22 7 13.5 15.5 8.5 10.5 2 17"/>
          <polyline points="16 7 22 7 22 13"/>
        </svg>
      </div>
      <h1 class="text-2xl font-semibold text-foreground">Welcome back</h1>
      <p class="text-sm text-muted-foreground">Sign in to your account</p>
    </div>

    <!-- Card -->
    <div class="rounded-lg border border-border bg-card p-6 space-y-4">

      <!-- Email -->
      <div class="space-y-1.5">
        <Label for="email">Email</Label>
        <input
          id="email"
          type="email"
          placeholder="you@example.com"
          class="w-full h-9 rounded-md border border-input bg-transparent px-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring transition-colors"
        />
      </div>

      <!-- Password -->
      <div class="space-y-1.5">
        <div class="flex items-center justify-between">
          <Label for="password">Password</Label>
          <a href="/forgot-password" class="text-xs text-primary hover:underline">
            Forgot password?
          </a>
        </div>
        <input
          id="password"
          type="password"
          placeholder="••••••••"
          class="w-full h-9 rounded-md border border-input bg-transparent px-3 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring transition-colors"
        />
      </div>

      <!-- Submit -->
      <Button class="w-full" onclick={() => {}}>
        Sign in
      </Button>

      <!-- Divider -->
      <div class="relative">
        <div class="absolute inset-0 flex items-center">
          <span class="w-full border-t border-border"></span>
        </div>
        <div class="relative flex justify-center text-xs">
          <span class="bg-card px-2 text-muted-foreground">or</span>
        </div>
      </div>

      <!-- Biometric placeholder -->
      <Button variant="outline" class="w-full gap-2" onclick={() => {}}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none"
          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2z"/>
          <path d="M12 6c-1.5 0-3 .75-3.87 2"/>
          <path d="M8 12a4 4 0 0 0 4 4"/>
          <path d="M12 8a4 4 0 0 1 4 4"/>
          <path d="M9 15.5C9.83 16.42 10.85 17 12 17"/>
        </svg>
        Continue with biometrics
      </Button>
    </div>

    <!-- Register link -->
    <Typography variant="muted" class="text-center">
      Don't have an account?
      <a href="/register" class="text-primary hover:underline font-medium">
        Sign up
      </a>
    </Typography>

  </div>
</main>