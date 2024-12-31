<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer/index.js';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import type { Snippet } from 'svelte';
	import { title } from '$lib/stores/title.store';
	import EventSelectorForm from '$lib/components/EventSelectorForm.svelte';
	import type { components } from '$lib/api/schema';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';

	const {
		trigger,
		formattedDate,
		round,
		class: className,
		initialId
	}: {
		trigger: Snippet;
		formattedDate: string;
		round: number;
		class: string;
		initialId?: number;
	} = $props();

	type AvailableEventResponse = components['schemas']['AvailableEventResponse'];

	let open = $state(false);

	$effect(() => {
		if (open) {
			title.set('Selezione evento per il ' + formattedDate);
		} else {
			title.set('Forum Meucci');
		}
	});

	let getAvailableEvents = $state(queryEvents);

	function reloadEvents() {
		console.debug('Hot reloading events for round', round);
		getAvailableEvents = queryEvents;
	}

	$effect(() => {
		if (open) {
			reloadEvents();
		}
	});

	async function queryEvents() {
		const params = new URLSearchParams({ round: String(round) });

		return fetch(`/api/available_events?${params.toString()}`).then(
			(res) => res.json() as Promise<AvailableEventResponse>
		);
	}
</script>

<Drawer.Root bind:open>
	<Drawer.Trigger class={className}>
		{@render trigger()}
	</Drawer.Trigger>
	<Drawer.Content>
		<Drawer.Header>
			<Drawer.Title>{formattedDate}</Drawer.Title>
		</Drawer.Header>
		<div class="mx-auto w-[95%]">
			{#await getAvailableEvents()}
				<div class="flex justify-center">
					<LineMdLoadingLoop />
				</div>
			{:then availableEvents}
				<EventSelectorForm {round} availableEvents={availableEvents.events} {initialId} />
			{:catch error}
				<div class="text-destructive-foreground">
					<p>Errore: {error.message}</p>
				</div>
			{/await}
		</div>
		<Drawer.Footer class="grid grid-cols-2">
			<Button class="col-span-2">Salva</Button>

			<Drawer.Close class={buttonVariants({ variant: 'outline' })}>Annulla</Drawer.Close>

			<Button class={buttonVariants({ variant: 'destructive' })}>Sono assente</Button>
		</Drawer.Footer>
	</Drawer.Content>
</Drawer.Root>
