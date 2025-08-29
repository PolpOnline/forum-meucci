<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import LucideMapPin from '~icons/lucide/map-pin';
	import LucideArrowLeft from '~icons/lucide/arrow-left';
	import AdminRound from './AdminRound.svelte';
	import { Button } from '$lib/components/ui/button';
	import { title } from '$lib/stores/title.store';

	let { data } = $props();

	const { activity_id, adminRounds } = $derived(data.data);
	const { name, description, room, rounds } = $derived(adminRounds);

	$effect(() => {
		title.set(`${name} - Turni - Forum Meucci`);
	});
</script>

<main>
	<div class="grid grid-cols-12">
		<div class="col-span-2 flex items-center justify-center">
			<Button class="text-primary" href="/forum/admin" size="icon" variant="outline">
				<LucideArrowLeft />
			</Button>
		</div>
		<div class="col-span-8 mt-5 text-center font-bold">
			<h1 class="text-3xl">
				{name}
			</h1>
			<div class="mt-1 text-sm text-gray-500">
				{description}
			</div>
			<Badge class="pointer-events-none mt-2" variant="secondary">
				<LucideMapPin class="mr-1 h-4 w-4" />
				<div class="text-sm">{room}</div>
			</Badge>
		</div>
		<div class="col-span-2"></div>
	</div>
	<div class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col space-y-4">
		{#each rounds as round (round.round)}
			<AdminRound {round} {activity_id} />
		{/each}
	</div>
</main>
