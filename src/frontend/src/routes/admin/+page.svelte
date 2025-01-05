<script lang="ts">
	import AdminEvent from './AdminEvent.svelte';

	let { data } = $props();

	let adminEvents = $derived(data.adminEvents!);
</script>

<main>
	<h1 class="mt-5 text-center text-3xl font-bold">I tuoi eventi</h1>

	<div class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col space-y-4">
		{#each adminEvents as event (event.id)}
			<AdminEvent {event} />
		{/each}
	</div>
</main>
