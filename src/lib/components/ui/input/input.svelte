<script lang="ts">
	import { cn } from "$lib/utils.js";
	import type { HTMLInputAttributes } from "svelte/elements";

	type Props = HTMLInputAttributes & {
		ref?: HTMLInputElement | null;
	};

	let {
		ref = $bindable<HTMLInputElement | null>(null),
		value = $bindable(""),
		type = "text",
		class: className,
		...restProps
	}: Props = $props();
</script>

<input
	bind:this={ref}
	bind:value
	{type}
	class={cn(
		"flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50",
		className
	)}
	{...restProps}
/>
