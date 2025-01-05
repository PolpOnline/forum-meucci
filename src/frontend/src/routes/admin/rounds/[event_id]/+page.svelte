<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import LucideMapPin from '~icons/lucide/map-pin';
	import AdminRound from './AdminRound.svelte';

	let { data } = $props();

	const { event_id, adminRounds } = $derived(data.data);
	const { name, room, rounds } = $derived(adminRounds);
</script>

<main>
	<h1 class="mt-5 text-center text-3xl font-bold">
		{name}
		<br />
		<Badge class="pointer-events-none mt-2" variant="secondary">
			<LucideMapPin class="mr-1 h-4 w-4" />
			<div class="translate-y-[5%] text-sm">{room}</div>
		</Badge>
	</h1>
	<div class="w-100 mx-auto mt-5 w-[95%] max-w-[800px]">
		{#each rounds as round (round.round)}
			<div class="my-4">
				<div class="mt-3">
					<AdminRound {round} {event_id} />
				</div>
			</div>
		{/each}
	</div>
</main>
