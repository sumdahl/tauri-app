<script lang="ts">
  import { signinSchema } from '$lib/schemas/auth';
  import { signin } from '$lib/service/auth';
  import { Input } from '$lib/components/ui/input';
  import { Button } from '$lib/components/ui/button';
  import { Typography } from '$lib/components/ui/typography';
  import { LineChart } from '@lucide/svelte';
  import { goto } from '$app/navigation';

  type LoginValues = {
    email: string;
    password: string;
  };

  let serverError = $state<string | null>(null);
  let loading = $state(false);
  let formData = $state<LoginValues>({ email: '', password: '' });
  let fieldErrors = $state<Partial<Record<keyof LoginValues, string[]>>>({});

  async function handleSubmit(e: Event) {
    e.preventDefault();
    fieldErrors = {};
    serverError = null;

    const result = signinSchema.safeParse(formData);
    if (!result.success) {
      fieldErrors = result.error.flatten().fieldErrors;
      return;
    }

    loading = true;
    try {
      await signin(result.data);
    } catch (e) {
      console.error('Signin error:', e);
      serverError = typeof e === 'string' ? e : 'Sign in failed. Please try again.';
    } finally {
      loading = false;
    }
  }
</script>

<div class="w-full max-w-md">
  <div class="flex items-center justify-center gap-2 mb-8">
    <LineChart size={22} class="text-primary" />
    <span class="font-semibold text-foreground tracking-tight">Marketflow</span>
  </div>

  <div class="rounded-xl border border-border bg-card p-8 shadow-sm">
    <div class="mb-6">
      <Typography variant="h3" class="text-foreground">Welcome back</Typography>
      <Typography variant="muted" class="mt-1">Sign in to your account</Typography>
    </div>

    {#if serverError}
      <div class="mb-4 px-3 py-2.5 rounded-lg bg-loss-bg border border-loss/20">
        <Typography variant="small" class="text-loss">{serverError}</Typography>
      </div>
    {/if}

    <form onsubmit={handleSubmit} class="flex flex-col gap-4">
      <div class="space-y-2">
        <label for="email" class="text-sm font-medium text-foreground">Email</label>
        <Input
          id="email"
          name="email"
          type="email"
          placeholder="you@example.com"
          bind:value={formData.email}
          autocomplete="email"
          aria-invalid={fieldErrors.email ? 'true' : undefined}
        />
        {#if fieldErrors.email}
          <Typography variant="small" class="text-loss">{fieldErrors.email[0]}</Typography>
        {/if}
      </div>

      <div class="space-y-2">
        <label for="password" class="text-sm font-medium text-foreground">Password</label>
        <Input
          id="password"
          name="password"
          type="password"
          placeholder="••••••••"
          bind:value={formData.password}
          autocomplete="current-password"
          aria-invalid={fieldErrors.password ? 'true' : undefined}
        />
        {#if fieldErrors.password}
          <Typography variant="small" class="text-loss">{fieldErrors.password[0]}</Typography>
        {/if}
      </div>

      <Button type="submit" class="w-full mt-2" disabled={loading}>
        {loading ? 'Signing in...' : 'Sign in'}
      </Button>
    </form>

    <div class="mt-6 text-center">
      <Typography variant="muted">
        Don't have an account?
        <button
          onclick={() => goto('/register')}
          class="text-primary hover:underline font-medium ml-1"
        >
          Create one
        </button>
      </Typography>
    </div>
  </div>
</div>