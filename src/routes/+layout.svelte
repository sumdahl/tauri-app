<script lang="ts">
import {onMount, onDestroy} from 'svelte'
import {theme} from '$lib/store/theme.svelte'
import "../app.css";

let {children} = $props()

onMount(() => {
    theme.init();
})

onDestroy(() => {
    theme.cleanup()
})
</script>

{@render children()}