<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import type { Snippet } from 'svelte';
	import { title } from '$lib/stores/title.store';
	import EventSelectorForm from '$lib/components/available_drawer/EventSelectorForm.svelte';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import { invalidateAll } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import type { AvailableEventResponse } from '$lib/utils';

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
		return fetch(`/api/events/available/${round}`).then(
			(res) => res.json() as Promise<AvailableEventResponse>
		);
	}

	let selectedId = $state(initialId);

	async function setEvent(round: number, event_id?: number) {
		const res = await fetch(`/api/events/set`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ round, event_id })
		});

		if (res.ok) {
			toast.success('Evento salvato');
		} else {
			toast.error("Errore durante il salvataggio dell'evento");
		}

		open = false;

		await invalidateAll();
	}

	let isSaving = $state(false);
	let isAbsentSaving = $state(false);
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
				<EventSelectorForm availableEvents={availableEvents.events} bind:selectedId />
			{:catch error}
				<div class="text-destructive-foreground">
					<p>Errore: {error.message}</p>
				</div>
			{/await}
		</div>
		<Drawer.Footer class="grid grid-cols-2">
			<Button
				class="col-span-2"
				onclick={async () => {
					isSaving = true;
					await setEvent(round, selectedId);
					isSaving = false;
				}}
			>
				{#if isSaving}
					<LineMdLoadingLoop />
				{:else}
					Salva
				{/if}
			</Button>

			<Drawer.Close class={buttonVariants({ variant: 'outline' })}>Annulla</Drawer.Close>

			<Button
				class={buttonVariants({ variant: 'destructive' })}
				onclick={async () => {
					isAbsentSaving = true;
					await setEvent(round);
					isAbsentSaving = false;
				}}
			>
				{#if isAbsentSaving}
					<LineMdLoadingLoop />
				{:else}
					Sono assente
				{/if}
			</Button>
		</Drawer.Footer>
	</Drawer.Content>
</Drawer.Root>
