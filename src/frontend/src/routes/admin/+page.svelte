<script lang="ts">
	import AdminActivity from './AdminActivity.svelte';
	import { Input } from '$lib/components/ui/input/index.js';

	let { data } = $props();

	let adminActivities = $derived(data.adminActivities!);

	let filterKey = $state('');

	const filteredActivities = $derived.by(() => {
		if (!filterKey) return adminActivities;

		return adminActivities.filter((activity) =>
			activity.name.toLowerCase().includes(filterKey.toLowerCase())
		);
	});
</script>

<main>
	<h1 class="mt-5 text-center text-3xl font-bold">Le tue attività</h1>

	<div class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col space-y-4">
		{#if adminActivities.length > 1}
			<Input placeholder="Cerca un'attività..." class="w-full" bind:value={filterKey} />
		{/if}

		{#each filteredActivities as activity (activity.id)}
			<AdminActivity {activity} />
		{/each}
	</div>
</main>
