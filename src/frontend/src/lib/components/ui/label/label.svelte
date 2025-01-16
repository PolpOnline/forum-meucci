<script lang="ts">
	import { Label as LabelPrimitive } from 'bits-ui';
	import { cn } from '$lib/utils.js';

	type $$Props = LabelPrimitive.Props;
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	type $$Events = LabelPrimitive.Events;

	let className: $$Props['class'] = undefined;
	export { className as class };
</script>

<LabelPrimitive.Root
	{...$$restProps}
	class={cn(
		'text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70',
		className
	)}
	on:mousedown
>
	<slot />
</LabelPrimitive.Root>
