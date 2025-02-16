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
	import { StatusCodes } from 'http-status-codes';
	import { activityFullDialogOpen } from '$lib/stores/dialogs.store';
	import { slide, type SlideParams } from 'svelte/transition';
	import { trackEvent } from '@lukulent/svelte-umami';

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
		} else if (res.status === StatusCodes.CONFLICT) {
			toast.error('Attività non più disponibile', {
				duration: 5000,
				action: {
					label: 'Info',
					onClick: () => activityFullDialogOpen.set(true)
				}
			});
		} else {
			toast.error("Errore durante il salvataggio dell'attività");
		}

		open = false;

		await invalidateAll();

		await trackEvent('Set Activity', { round, activity_id: activity_id || 'absent' });
	}

	let isSaving = $state(false);
	let isAbsentSaving = $state(false);

	let isOpening = $state(false);

	const slideParams: SlideParams = $derived({
		duration: isOpening ? 0 : 300
	});
</script>

<Drawer.Root
	bind:open
	onAnimationEnd={() => (isOpening = false)}
	onOpenChange={() => (isOpening = true)}
>
	<Drawer.Trigger class={className}>
		{@render trigger()}
	</Drawer.Trigger>
	<Drawer.Content>
		<Drawer.Header>
			<Drawer.Title>{formattedDate}</Drawer.Title>
		</Drawer.Header>
		<div class="mx-auto w-[95%]">
			{#await getAvailableActivities()}
				<div class="flex justify-center text-4xl">
					<LineMdLoadingLoop />
				</div>
			{:then availableActivities}
				<div transition:slide={slideParams} class="min-h-16">
					{#if availableActivities.activities.length !== 0}
						<div>
							<ActivitySelectorForm
								availableActivities={availableActivities.activities}
								bind:selectedId
							/>
						</div>
					{:else}
						<div
							class="text-muted-foreground flex h-full w-full flex-col items-center justify-center"
						>
							<p>Non sono disponibili più attività</p>
						</div>
					{/if}
				</div>
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
