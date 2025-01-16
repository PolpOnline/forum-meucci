<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Command from '$lib/components/ui/command';
	// noinspection ES6UnusedImports
	import { type AvailableActivity, cn } from '$lib/utils.js';
	import AvailableActivityCard from '$lib/components/available_drawer/AvailableActivityCard.svelte';
	import { LucideCheck } from 'lucide-svelte';

	let {
		availableActivities,
		selectedId = $bindable()
	}: { availableActivities: AvailableActivity[]; selectedId?: number } = $props();
</script>

<Command.Root>
	<Command.Input placeholder="Cerca un attività..." />
	<Command.List class="max-h-[600px]">
		<Command.Empty>Nessuna attività trovata</Command.Empty>
		<Command.Group>
			{#each availableActivities as activity (activity.id)}
				<Command.Item
					value={String(activity.name)}
					onSelect={() => {
						selectedId = activity.id;
					}}
					class="my-2"
				>
					<LucideCheck
						class={cn('mr-2 size-4', selectedId !== activity.id && 'text-transparent')}
					/>
					<AvailableActivityCard {activity} />
				</Command.Item>
			{/each}
		</Command.Group>
	</Command.List>
</Command.Root>
