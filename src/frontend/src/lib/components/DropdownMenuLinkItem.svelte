<script lang="ts">
	import { DropdownMenuItem } from '$lib/components/ui/dropdown-menu';

	import type { Snippet } from 'svelte';

	const {
		href,
		children,
		target,
		class: className
	}: {
		href: string;
		children?: Snippet | undefined;
		target?: string | undefined;
		class?: string | undefined;
	} = $props();
</script>

<DropdownMenuItem class={className}>
	{#snippet child({ props })}
		<a {...props} {href} {target}>
			{@render children?.()}
		</a>
	{/snippet}
</DropdownMenuItem>
