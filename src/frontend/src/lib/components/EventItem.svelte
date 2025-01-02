<script lang="ts">
	import LucidePencil from '~icons/lucide/pencil';
	import EventSelectorDrawer from '$lib/components/EventSelectorDrawer.svelte';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import type { components } from '$lib/api/schema';
	import LucideMapPin from '~icons/lucide/map-pin';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import LucideUser from '~icons/lucide/user';

	const {
		event,
		formattedDate
	}: { event: components['schemas']['SelectedEvent']; formattedDate: string } = $props();

	const { id, name, description, round, room, available_seats, total_seats } = $derived(event);
</script>

<div
	class="relative flex w-full items-center rounded-xl border"
	class:p-5={name}
	class:border-dashed={!name}
	class:custom-absent={name === 'absent'}
>
	{#if name === 'absent'}
		<div class="text-lg">Assente</div>
	{:else if name}
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
			{#if available_seats && total_seats}
				<Badge class="pointer-events-none mt-2" variant="secondary">
					<LucideUser class="mr-1 h-4 w-4" />
					<div class="translate-y-[5%] text-sm">{available_seats} / {total_seats}</div>
				</Badge>
			{/if}
		</div>
	{:else}
		<EventSelectorDrawer
			{formattedDate}
			{round}
			class={cn(
				buttonVariants({ variant: 'ghost' }),
				'h-full min-h-[4.25rem] w-full rounded-xl text-lg text-muted-foreground'
			)}
		>
			{#snippet trigger()}
				+ Seleziona un evento
			{/snippet}
		</EventSelectorDrawer>
	{/if}

	{#if name}
		<EventSelectorDrawer
			{formattedDate}
			{round}
			class={cn('absolute right-5', buttonVariants({ variant: 'outline', size: 'icon' }))}
			initialId={id}
		>
			{#snippet trigger()}
				<LucidePencil />
			{/snippet}
		</EventSelectorDrawer>
	{/if}
</div>
