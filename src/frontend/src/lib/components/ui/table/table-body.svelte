<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import type { WithElementRef } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLTableSectionElement>> = $props();
</script>

<tbody {...restProps} bind:this={ref} class={cn('[&_tr:last-child]:border-0', className)}>
	{@render children?.()}
</tbody>
