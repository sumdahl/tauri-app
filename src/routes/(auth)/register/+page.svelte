<script lang="ts">
  import { superForm } from 'sveltekit-superforms';
  import { signupSchema } from '$lib/schemas/auth';
  import { typedZodClient } from '$lib/schemas/superforms';
  import { signup } from '$lib/service/auth';
  import * as Form from '$lib/components/ui/form';
  import { Input } from '$lib/components/ui/input';
  import { Button } from '$lib/components/ui/button';
  import { Typography } from '$lib/components/ui/typography';
  import { LineChart } from '@lucide/svelte';
  import { goto } from '$app/navigation';

  let serverError = $state<string | null>(null);
  let loading = $state(false);

  const form = superForm(
    { fullname: '', email: '', password: '', confirmPassword: '' },
    {
      SPA: true,
      validators: typedZodClient(signupSchema),
      onUpdate: async ({ form: formState }) => {
        if (!formState.valid) return;
        loading = true;
        serverError = null;
        try {
          await signup({
            fullname: formState.data.fullname,
            email: formState.data.email,
            password: formState.data.password,
          });
        } catch (e) {
          serverError = typeof e === 'string' ? e : 'Registration failed. Please try again.';
        } finally {
          loading = false;
        }
      },
    }
  );

  const { form: formData, enhance } = form;
</script>

<div class="w-full max-w-md">
  <!-- Logo -->
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

    <form method="POST" use:enhance class="flex flex-col gap-4">
      <Form.Field {form} name="fullname">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Full name</Form.Label>
            <Input
              {...props}
              type="text"
              placeholder="John Doe"
              bind:value={$formData.fullname}
              autocomplete="name"
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field {form} name="email">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Email</Form.Label>
            <Input
              {...props}
              type="email"
              placeholder="you@example.com"
              bind:value={$formData.email}
              autocomplete="email"
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field {form} name="password">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Password</Form.Label>
            <Input
              {...props}
              type="password"
              placeholder="••••••••"
              bind:value={$formData.password}
              autocomplete="new-password"
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Form.Field {form} name="confirmPassword">
        <Form.Control>
          {#snippet children({ props })}
            <Form.Label>Confirm password</Form.Label>
            <Input
              {...props}
              type="password"
              placeholder="••••••••"
              bind:value={$formData.confirmPassword}
              autocomplete="new-password"
            />
          {/snippet}
        </Form.Control>
        <Form.FieldErrors />
      </Form.Field>

      <Button type="submit" class="w-full mt-2" disabled={loading}>
        {loading ? 'Creating account...' : 'Create account'}
      </Button>
    </form>

    <div class="mt-6 text-center">
      <Typography variant="muted">
        Already have an account?{' '}
        <button
          onclick={() => goto('/login')}
          class="text-primary hover:underline font-medium"
        >
          Sign in
        </button>
      </Typography>
    </div>
  </div>
</div>
