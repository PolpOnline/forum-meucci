<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import type { components } from '$lib/api/schema';
	import LucideMapPin from '~icons/lucide/map-pin';
	import { Badge } from '$lib/components/ui/badge';
	import LucideClock from '~icons/lucide/clock';

	const { activity }: { activity: components['schemas']['AdminActivity'] } = $props();

	const { id, name, description, room } = $derived(activity);
</script>

<div class="relative flex w-full items-center rounded-xl border p-5">
	<div>
		<div class="text-lg">{name}</div>
		{#if description}
			<div class="text-muted-foreground text-sm">{description}</div>
		{/if}
		{#if room}
			<Badge class="pointer-events-none mt-2" variant="secondary">
				<LucideMapPin class="mr-1 h-4 w-4" />
				<div class="text-sm">{room}</div>
			</Badge>
		{/if}
	</div>

	<Button
		class={cn(
			'text-primary absolute right-5',
			buttonVariants({ variant: 'outline', size: 'icon' })
		)}
		href="/admin/rounds/{id}"
	>
		<LucideClock />
	</Button>
</div>
