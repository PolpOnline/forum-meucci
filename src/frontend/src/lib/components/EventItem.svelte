<script lang="ts">
	import LucidePencil from '~icons/lucide/pencil';
	import EventSelectorDrawer from '$lib/components/EventSelectorDrawer.svelte';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';

	const { round, name, formattedDate }: { round: number; name: string; formattedDate: string } =
		$props();
</script>

<div
	class="relative flex w-full items-center justify-center rounded-xl border"
	class:p-5={name}
	class:border-dashed={!name}
	class:custom-absent={name === 'absent'}
>
	{#if name === 'absent'}
		<div class="text-lg">Assente</div>
	{:else if name}
		<div class="text-lg">{name}</div>
	{:else}
		<EventSelectorDrawer
			{formattedDate}
			{round}
			class={cn(
				buttonVariants({ variant: 'ghost' }),
				'h-full min-h-[4.25rem] w-full text-lg text-muted-foreground'
			)}
		>
			{#snippet trigger()}
				+ Seleziona un evento
			{/snippet}
		</EventSelectorDrawer>
	{/if}

	{#if name}
		<EventSelectorDrawer
			{formattedDate}
			{round}
			class={cn('absolute right-5', buttonVariants({ variant: 'outline', size: 'icon' }))}
		>
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
