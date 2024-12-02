<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer/index.js';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import type { Snippet } from 'svelte';
	import { title } from '$lib/stores/title.store';
	import EventSelectorForm from '$lib/components/EventSelectorForm.svelte';
	import { type AvailableEventItem, cn } from '$lib/utils';

	const {
		trigger,
		formattedDate,
		round,
		class: className
	}: { trigger: Snippet; formattedDate: string; round: number; class: string } = $props();

	const availableEvents: AvailableEventItem[] = [
		{
			id: 1,
			name: 'Evento 1',
			description: 'Descrizione evento 1',
			room: 'Sala 1',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 2,
			name: 'Evento 2',
			description: 'Descrizione evento 2',
			room: 'Sala 2',
			availableSeats: 5,
			totalSeats: 10
		},
		{
			id: 3,
			name: 'Evento 3',
			description: 'Descrizione evento 3',
			room: 'Sala 3',
			availableSeats: 0,
			totalSeats: 10
		},
		{
			id: 4,
			name: 'Evento 4',
			description: 'Descrizione evento 4',
			room: 'Sala 4',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 5,
			name: 'Evento 5',
			description: 'Descrizione evento 5',
			room: 'Sala 5',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 6,
			name: 'Evento 6',
			description: 'Descrizione evento 6',
			room: 'Sala 6',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 7,
			name: 'Evento 7',
			description: 'Descrizione evento 7',
			room: 'Sala 7',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 8,
			name: 'Evento 8',
			description: 'Descrizione evento 8',
			room: 'Sala 8',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 9,
			name: 'Evento 9',
			description: 'Descrizione evento 9',
			room: 'Sala 9',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 10,
			name: 'Evento 10',
			description: 'Descrizione evento 10',
			room: 'Sala 10',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 11,
			name: 'Evento 11',
			description: 'Descrizione evento 11',
			room: 'Sala 11',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 12,
			name: 'Evento 12',
			description: 'Descrizione evento 12',
			room: 'Sala 12',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 13,
			name: 'Evento 13',
			description: 'Descrizione evento 13',
			room: 'Sala 13',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 14,
			name: 'Evento 14',
			description: 'Descrizione evento 14',
			room: 'Sala 14',
			availableSeats: 10,
			totalSeats: 10
		},
		{
			id: 15,
			name: 'Evento 15',
			description: 'Descrizione evento 15',
			room: 'Sala 15',
			availableSeats: 10,
			totalSeats: 10
		}
	];

	let open = $state(false);

	$effect(() => {
		if (open) {
			title.set('Selezione evento per il ' + formattedDate);
		} else {
			title.set('Forum Meucci');
		}
	});
</script>

<Drawer.Root bind:open>
	<Drawer.Trigger class={cn(className, buttonVariants({ variant: 'outline', size: 'icon' }))}>
		{@render trigger()}
	</Drawer.Trigger>
	<Drawer.Content>
		<Drawer.Header>
			<Drawer.Title>{formattedDate}</Drawer.Title>
		</Drawer.Header>
		<div class="mx-auto w-[95%]">
			<EventSelectorForm {round} {availableEvents} />
		</div>
		<Drawer.Footer class="grid grid-cols-2">
			<Drawer.Close class={buttonVariants({ variant: 'outline' })}>Annulla</Drawer.Close>
			<Button>Salva</Button>
		</Drawer.Footer>
	</Drawer.Content>
</Drawer.Root>
