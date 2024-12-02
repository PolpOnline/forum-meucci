<script lang="ts">
	import { type AvailableEventItem } from '$lib/utils';
	import LucideUser from '~icons/lucide/user';

	const { event }: { event: AvailableEventItem } = $props();
</script>

<div class="relative w-full rounded-xl py-2">
	<div class="text-lg">{event.name}</div>
	<div class="text-sm text-muted-foreground">{event.description}</div>
	<div class="text-sm text-muted-foreground">{event.room}</div>
	<div class="absolute right-2 top-2 flex items-center text-sm text-muted-foreground">
		{event.availableSeats} / {event.totalSeats}
		<LucideUser class="inline" />
	</div>
</div>
