<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Command from '$lib/components/ui/command';
	// noinspection ES6UnusedImports
	import { type AvailableEvent, cn } from '$lib/utils.js';
	import AvailableEventCard from '$lib/components/available_drawer/AvailableEventCard.svelte';
	import { LucideCheck } from 'lucide-svelte';

	let {
		availableEvents,
		selectedId = $bindable()
	}: { availableEvents: AvailableEvent[]; selectedId?: number } = $props();
</script>

<Command.Root>
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
					class="my-2"
				>
					<LucideCheck class={cn('mr-2 size-4', selectedId !== event.id && 'text-transparent')} />
					<AvailableEventCard {event} />
				</Command.Item>
			{/each}
		</Command.Group>
	</Command.List>
</Command.Root>
