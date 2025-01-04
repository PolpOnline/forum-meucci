<script lang="ts">
	import { type AvailableEvent } from '$lib/utils';
	import LucideUsers from '~icons/lucide/users';

	const { event }: { event: AvailableEvent } = $props();
</script>

<div class="relative w-full rounded-xl py-2">
	<div class="text-lg">{event.name}</div>
	<div class="text-sm text-muted-foreground">{event.description}</div>
	<div class="text-sm text-muted-foreground">{event.room}</div>
	<div class="absolute right-2 top-2 flex items-center text-sm text-muted-foreground">
		{event.available_seats} / {event.total_seats}
		<LucideUsers class="ml-1 inline" />
	</div>
</div>
