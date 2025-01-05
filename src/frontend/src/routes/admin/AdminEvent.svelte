<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import type { components } from '$lib/api/schema';
	import LucideMapPin from '~icons/lucide/map-pin';
	import { Badge } from '$lib/components/ui/badge';
	import LucideClock from '~icons/lucide/clock';

	const { event }: { event: components['schemas']['AdminEvent'] } = $props();

	const { id, name, description, room } = $derived(event);
</script>

<div
	class="relative flex w-full items-center rounded-xl border"
	class:border-dashed={!name}
	class:custom-absent={name === 'absent'}
	class:p-5={name}
>
	<div>
		<div class="text-lg">{name}</div>
		{#if description}
			<div class="text-sm text-muted-foreground">{description}</div>
		{/if}
		{#if room}
			<Badge class="pointer-events-none mt-2" variant="secondary">
				<LucideMapPin class="mr-1 h-4 w-4" />
				<div class="translate-y-[5%] text-sm">{room}</div>
			</Badge>
		{/if}
	</div>

	<Button
		class={cn(
			'absolute right-5 text-primary',
			buttonVariants({ variant: 'outline', size: 'icon' })
		)}
		href="/admin/rounds/{id}"
	>
		<LucideClock />
	</Button>
</div>
