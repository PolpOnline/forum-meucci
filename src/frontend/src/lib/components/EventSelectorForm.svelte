<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Command from '$lib/components/ui/command/index.js';
	// noinspection ES6UnusedImports
	import { type AvailableEvent, cn } from '$lib/utils.js';
	import AvailableEventCard from '$lib/components/AvailableEventCard.svelte';
	import { LucideCheck } from 'lucide-svelte';

	const {
		availableEvents,
		initialId
	}: { round: number; availableEvents: AvailableEvent[]; initialId?: number } = $props();

	let selectedId = $state(initialId);
</script>

<Command.Root
	filter={(value, search) => {
		if (value === 'Assente') return 1;
		if (value.toLowerCase().startsWith(search.toLowerCase())) return 0.95;
		return 0;
	}}
>
	<Command.Input placeholder="Cerca un evento..." />
	<Command.List class="max-h-[600px]">
		<Command.Empty>Nessun evento trovato</Command.Empty>
		<Command.Group>
			{#each availableEvents as event (event.id)}
				<Command.Item
					value={String(event.name)}
					onSelect={() => {
						selectedId = event.id;
					}}
					class={cn('my-2', event.name === 'Assente' ? '!custom-absent-saturated' : '')}
				>
					<LucideCheck class={cn('mr-2 size-4', selectedId !== event.id && 'text-transparent')} />
					<AvailableEventCard {event} />
				</Command.Item>
			{/each}
		</Command.Group>
	</Command.List>
</Command.Root>
