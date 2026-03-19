<script lang="ts">
  import { signupSchema } from '$lib/schemas/auth';
  import { signup } from '$lib/service/auth';
  import { Input } from '$lib/components/ui/input';
  import { Button } from '$lib/components/ui/button';
  import { Typography } from '$lib/components/ui/typography';
  import { LineChart } from '@lucide/svelte';
  import { goto } from '$app/navigation';

  type RegisterValues = {
    fullname: string;
    email: string;
    password: string;
    confirmPassword: string;
  };

  let serverError = $state<string | null>(null);
  let loading = $state(false);
  let formData = $state<RegisterValues>({
    fullname: '',
    email: '',
    password: '',
    confirmPassword: '',
  });
  let fieldErrors = $state<Partial<Record<keyof RegisterValues, string[]>>>({});

  async function handleSubmit(e: Event) {
    e.preventDefault();
    fieldErrors = {};
    serverError = null;

    const result = signupSchema.safeParse(formData);
    if (!result.success) {
      fieldErrors = result.error.flatten().fieldErrors;
      return;
    }

    loading = true;
    try {
      await signup({
        fullname: result.data.fullname,
        email: result.data.email,
        password: result.data.password,
      });
    } catch (e) {
      console.error('Signup error:', e);
      serverError = typeof e === 'string' ? e : 'Registration failed. Please try again.';
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
      <Typography variant="h3" class="text-foreground">Create an account</Typography>
      <Typography variant="muted" class="mt-1">Start tracking markets today</Typography>
    </div>

    {#if serverError}
      <div class="mb-4 px-3 py-2.5 rounded-lg bg-loss-bg border border-loss/20">
        <Typography variant="small" class="text-loss">{serverError}</Typography>
      </div>
    {/if}

    <form onsubmit={handleSubmit} class="flex flex-col gap-4">
      <div class="space-y-2">
        <label for="fullname" class="text-sm font-medium text-foreground">Full name</label>
        <Input
          id="fullname"
          name="fullname"
          type="text"
          placeholder="John Doe"
          bind:value={formData.fullname}
          autocomplete="name"
          aria-invalid={fieldErrors.fullname ? 'true' : undefined}
        />
        {#if fieldErrors.fullname}
          <Typography variant="small" class="text-loss">{fieldErrors.fullname[0]}</Typography>
        {/if}
      </div>

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
          autocomplete="new-password"
          aria-invalid={fieldErrors.password ? 'true' : undefined}
        />
        {#if fieldErrors.password}
          <Typography variant="small" class="text-loss">{fieldErrors.password[0]}</Typography>
        {/if}
      </div>

      <div class="space-y-2">
        <label for="confirmPassword" class="text-sm font-medium text-foreground">
          Confirm password
        </label>
        <Input
          id="confirmPassword"
          name="confirmPassword"
          type="password"
          placeholder="••••••••"
          bind:value={formData.confirmPassword}
          autocomplete="new-password"
          aria-invalid={fieldErrors.confirmPassword ? 'true' : undefined}
        />
        {#if fieldErrors.confirmPassword}
          <Typography variant="small" class="text-loss">{fieldErrors.confirmPassword[0]}</Typography>
        {/if}
      </div>

      <Button type="submit" class="w-full mt-2" disabled={loading}>
        {loading ? 'Creating account...' : 'Create account'}
      </Button>
    </form>

    <div class="mt-6 text-center">
      <Typography variant="muted">
        Already have an account?
        <button
          onclick={() => goto('/login')}
          class="text-primary hover:underline font-medium ml-1"
        >
          Sign in
        </button>
      </Typography>
    </div>
  </div>
</div>