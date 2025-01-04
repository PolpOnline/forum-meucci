<script lang="ts">
	import AdminEvent from './AdminEvent.svelte';

	let { data } = $props();

	let adminEvents = $derived(data.adminEvents!);
</script>

<main>
	<div class="w-100 mx-auto mt-5 w-[95%] max-w-[800px]">
		{#each adminEvents as event (event.id)}
			<div class="my-4">
				<div class="mt-3">
					<AdminEvent {event} />
				</div>
			</div>
		{/each}
	</div>
</main>
