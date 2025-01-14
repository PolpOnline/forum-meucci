<script lang="ts">
	import AdminEvent from './AdminEvent.svelte';
	import { Input } from '$lib/components/ui/input/index.js';

	let { data } = $props();

	let adminEvents = $derived(data.adminEvents!);

	let filterKey = $state('');

	const filteredEvents = $derived.by(() => {
		if (!filterKey) return adminEvents;

		return adminEvents.filter((event) =>
			event.name.toLowerCase().includes(filterKey.toLowerCase())
		);
	});
</script>

<main>
	<h1 class="mt-5 text-center text-3xl font-bold">I tuoi eventi</h1>

	<div class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col space-y-4">
		<Input placeholder="Cerca un evento..." class="w-full" bind:value={filterKey} />

		{#each filteredEvents as event (event.id)}
			<AdminEvent {event} />
		{/each}
	</div>
</main>
