<script lang="ts">
	import { type AvailableActivity } from '$lib/utils';
	import LucideUsers from '~icons/lucide/users';

	const { activity }: { activity: AvailableActivity } = $props();
</script>

<div class="relative w-full rounded-xl py-2">
	<div class="text-lg">{activity.name}</div>
	<div class="text-sm text-muted-foreground">{activity.description}</div>
	<div class="text-sm text-muted-foreground">{activity.room}</div>
	<div class="absolute right-2 top-2 flex items-center text-sm text-muted-foreground">
		{activity.used_seats} / {activity.total_seats}
		<LucideUsers class="ml-1 inline" />
	</div>
</div>
