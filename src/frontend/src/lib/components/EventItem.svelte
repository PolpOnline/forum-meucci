<script lang="ts">
	import LucidePencil from '~icons/lucide/pencil';
	import EventSelectorDrawer from '$lib/components/EventSelectorDrawer.svelte';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import type { components } from '$lib/api/schema';

	const {
		event,
		formattedDate
	}: { event: components['schemas']['SelectedEvent']; formattedDate: string } = $props();

	const { id, name, description, round } = event;
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
		<div class="flex flex-col items-center">
			<div class="text-lg">{name}</div>
			{#if description}
				<div class="text-sm text-muted-foreground">{description}</div>
			{/if}
		</div>
	{:else}
		<EventSelectorDrawer
			{formattedDate}
			{round}
			class={cn(
				buttonVariants({ variant: 'ghost' }),
				'h-full min-h-[4.25rem] w-full rounded-xl text-lg text-muted-foreground'
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
			initialId={id}
		>
			{#snippet trigger()}
				<LucidePencil />
			{/snippet}
		</EventSelectorDrawer>
	{/if}
</div>
