<!-- src/lib/components/ui/typography/typography.svelte -->
<script lang="ts">
  type Variant =
    | "h1"
    | "h2"
    | "h3"
    | "h4"
    | "p"
    | "blockquote"
    | "lead"
    | "large"
    | "small"
    | "muted"
    | "inline-code";

  let {
    variant = "p",
    class: className = "",
    children,
  }: {
    variant?: Variant;
    class?: string;
    children?: any;
  } = $props();

  const variants: Record<Variant, { tag: string; classes: string }> = {
    h1: {
      tag: "h1",
      classes:
        "scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl",
    },
    h2: {
      tag: "h2",
      classes:
        "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0",
    },
    h3: {
      tag: "h3",
      classes: "scroll-m-20 text-2xl font-semibold tracking-tight",
    },
    h4: {
      tag: "h4",
      classes: "scroll-m-20 text-xl font-semibold tracking-tight",
    },
    p: {
      tag: "p",
      classes: "leading-7 [&:not(:first-child)]:mt-6",
    },
    blockquote: {
      tag: "blockquote",
      classes: "mt-6 border-s-2 ps-6 italic",
    },
    lead: {
      tag: "p",
      classes: "text-muted-foreground text-xl",
    },
    large: {
      tag: "div",
      classes: "text-lg font-semibold",
    },
    small: {
      tag: "small",
      classes: "text-sm leading-none font-medium",
    },
    muted: {
      tag: "p",
      classes: "text-muted-foreground text-sm",
    },
    "inline-code": {
      tag: "code",
      classes:
        "bg-muted relative rounded px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold",
    },
  };

const tag = $derived(variants[variant].tag);
const classes = $derived(variants[variant].classes);

</script>

<svelte:element this={tag} class="{classes} {className}">
  {@render children?.()}
</svelte:element>