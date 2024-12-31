<script lang="ts">
	import { type AvailableEvent } from '$lib/utils';
	import LucideUser from '~icons/lucide/user';

	const { event }: { event: AvailableEvent } = $props();

	const isAbsent = event.name === 'Assente';
</script>

<div class="relative w-full rounded-xl py-2">
	<div class="text-lg">{event.name}</div>
	<div class="text-sm text-muted-foreground">{event.description}</div>
	<div class="text-sm text-muted-foreground">{event.room}</div>
	{#if !isAbsent}
		<div class="absolute right-2 top-2 flex items-center text-sm text-muted-foreground">
			{event.available_seats} / {event.total_seats}
			<LucideUser class="inline" />
		</div>
	{/if}
</div>
