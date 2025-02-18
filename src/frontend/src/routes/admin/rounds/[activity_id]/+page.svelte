<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import LucideMapPin from '~icons/lucide/map-pin';
	import LucideArrowLeft from '~icons/lucide/arrow-left';
	import AdminRound from './AdminRound.svelte';
	import { Button } from '$lib/components/ui/button';

	let { data } = $props();

	const { activity_id, adminRounds } = $derived(data.data);
	const { name, room, rounds } = $derived(adminRounds);
</script>

<main>
	<div class="grid grid-cols-12">
		<div class="col-span-2 flex items-center justify-center">
			<Button class="text-primary" href="/admin" size="icon" variant="outline">
				<LucideArrowLeft />
			</Button>
		</div>
		<h1 class="col-span-8 mt-5 text-center text-3xl font-bold">
			{name}
			<br />
			<Badge class="pointer-events-none mt-2" variant="secondary">
				<LucideMapPin class="mr-1 h-4 w-4" />
				<div class="text-sm">{room}</div>
			</Badge>
		</h1>
		<div class="col-span-2"></div>
	</div>
	<div class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col space-y-4">
		{#each rounds as round (round.round)}
			<AdminRound {round} {activity_id} />
		{/each}
	</div>
</main>
