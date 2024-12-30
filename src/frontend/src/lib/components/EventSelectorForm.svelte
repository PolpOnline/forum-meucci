<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Command from '$lib/components/ui/command/index.js';
	// noinspection ES6UnusedImports
	import { type AvailableEventItem, cn } from '$lib/utils.js';
	import AvailableEventCard from '$lib/components/AvailableEventCard.svelte';
	import { LucideCheck } from 'lucide-svelte';

	const { availableEvents }: { round: number; availableEvents: AvailableEventItem[] } = $props();

	let selectedId = $state('');
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
						selectedId = String(event.id);
					}}
				>
					<LucideCheck
						class={cn('mr-2 size-4', selectedId !== String(event.id) && 'text-transparent')}
					/>
					<AvailableEventCard {event} />
				</Command.Item>
			{/each}
		</Command.Group>
	</Command.List>
</Command.Root>
