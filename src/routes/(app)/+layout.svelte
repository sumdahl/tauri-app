<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/store/auth.svelte';

  let { children } = $props();

  onMount(() => {
    if (!authStore.isAuthenticated) {
      goto('/login');
    }
  });
</script>

{#if authStore.isAuthenticated}
  {@render children()}
{/if}