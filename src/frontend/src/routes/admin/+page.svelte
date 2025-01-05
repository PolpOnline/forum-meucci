<script lang="ts">
	import AdminEvent from './AdminEvent.svelte';

	let { data } = $props();

	let adminEvents = $derived(data.adminEvents!);
</script>

<main>
	<div class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col space-y-4">
		{#each adminEvents as event (event.id)}
			<AdminEvent {event} />
		{/each}
	</div>
</main>
