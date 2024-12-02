<script lang="ts">
	import LucidePencil from '~icons/lucide/pencil';
	import EventSelectorDrawer from '$lib/components/EventSelectorDrawer.svelte';

	const { round, name, formattedDate }: { round: number; name: string; formattedDate: string } =
		$props();
</script>

<div
	class="w-100 relative flex items-center justify-center rounded-xl border p-5"
	class:border-dashed={!name}
	class:custom-absent={name === 'absent'}
>
	{#if name === 'absent'}
		<div class="text-lg">Assente</div>
	{:else if name}
		<div class="text-lg">{name}</div>
	{:else}
		<div class="text-lg text-muted-foreground">+ Seleziona un evento</div>
	{/if}

	{#if name}
		<EventSelectorDrawer {formattedDate} {round} class="absolute right-5">
			{#snippet trigger()}
				<LucidePencil />
			{/snippet}
		</EventSelectorDrawer>
	{/if}
</div>

<style lang="postcss">
	.custom-absent {
		@apply border-destructive;

		background: repeating-linear-gradient(
			45deg,
			hsl(var(--destructive) / 30%),
			hsl(var(--destructive) / 30%) 10px,
			hsl(var(--destructive) / 50%) 10px,
			hsl(var(--destructive) / 50%) 20px
		);
	}
</style>
