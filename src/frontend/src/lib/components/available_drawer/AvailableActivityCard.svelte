<script lang="ts">
	import { type AvailableActivity } from '$lib/utils';
	import LucideUsers from '~icons/lucide/users';

	const { activity }: { activity: AvailableActivity } = $props();
</script>

<div class="relative w-full rounded-xl py-2">
	<div class="text-lg">{activity.name}</div>
	<div class="text-muted-foreground text-sm">{activity.description}</div>
	<div class="text-muted-foreground text-sm">{activity.room}</div>
	<div class="text-muted-foreground absolute top-2 right-2 flex items-center text-sm">
		{activity.used_seats} / {activity.total_seats}
		<LucideUsers class="ml-1 inline" />
	</div>
</div>
