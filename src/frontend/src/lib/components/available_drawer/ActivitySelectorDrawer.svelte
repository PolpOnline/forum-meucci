<script lang="ts">
	// noinspection ES6UnusedImports
	import * as Drawer from '$lib/components/ui/drawer';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import type { Snippet } from 'svelte';
	import { title } from '$lib/stores/title.store';
	import ActivitySelectorForm from '$lib/components/available_drawer/ActivitySelectorForm.svelte';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import { invalidateAll } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import type { AvailableActivityResponse } from '$lib/utils';

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
			title.set('Selezione attività per ' + formattedDate);
		} else {
			title.set('Forum Meucci');
		}
	});

	let getAvailableActivities = $state(queryActivities);

	function reloadActivities() {
		console.debug('Hot reloading activities for round', round);
		getAvailableActivities = queryActivities;
	}

	$effect(() => {
		if (open) {
			reloadActivities();
		}
	});

	async function queryActivities() {
		return fetch(`/api/activities/available/${round}`).then(
			(res) => res.json() as Promise<AvailableActivityResponse>
		);
	}

	let selectedId = $state(initialId);

	async function setActivity(round: number, activity_id?: number) {
		const res = await fetch(`/api/activities/set`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ round, activity_id })
		});

		if (res.ok) {
			toast.success('Attività salvata');
		} else {
			toast.error("Errore durante il salvataggio dell'attività");
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
			{#await getAvailableActivities()}
				<div class="flex justify-center">
					<LineMdLoadingLoop />
				</div>
			{:then availableActivities}
				<ActivitySelectorForm
					availableActivities={availableActivities.activities}
					bind:selectedId
				/>
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
					await setActivity(round, selectedId);
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
					await setActivity(round);
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
