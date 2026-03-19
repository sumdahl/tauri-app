<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/store/auth.svelte';

  let { children } = $props();

  onMount(() => {
    if (authStore.isAuthenticated) {
      goto('/dashboard');
    }
  });
</script>

<div class="min-h-screen bg-background flex items-center justify-center px-4">
  {@render children()}
</div>