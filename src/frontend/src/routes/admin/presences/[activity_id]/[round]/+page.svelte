<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import LucideMapPin from '~icons/lucide/map-pin';
	import LucideArrowLeft from '~icons/lucide/arrow-left';
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	// noinspection ES6UnusedImports
	import * as Table from '$lib/components/ui/table/index.js';
	import { toast } from 'svelte-sonner';
	import { invalidateAll } from '$app/navigation';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';
	import LineMdConfirm from '~icons/line-md/confirm';
	import LineMdClose from '~icons/line-md/close';
	import { Label } from '$lib/components/ui/label';
	import LucideUsers from '~icons/lucide/users';
	import { formatItalianDate } from '$lib/utils/dates.js';
	import LucideDice5 from '~icons/lucide/dice-5';
	import LucidePen from '~icons/lucide/pen';
	import { slide } from 'svelte/transition';

	let { data } = $props();

	// Use a state to be able to update the data reactively
	const { activity_id, round, adminPresences } = $state(data.data);
	const { name, description, room, date, presences, total_seats } = $state(adminPresences);
	const used_seats = adminPresences.presences.length;

	type SpinnerState = 'loading' | 'success' | 'error' | 'idle';

	const spinnerStates = $state(Array(presences.length).fill('idle') as SpinnerState[]);

	async function setPresence(i: number) {
		const { present, id: user_id } = presences[i]!;

		spinnerStates[i] = 'idle';

		const loadingTimeout = setTimeout(() => {
			spinnerStates[i] = 'loading';
		}, 200);

		const res = await fetch(`/api/admin/set_presence`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ activity_id, round, user_id, present })
		});

		if (res.ok) {
			spinnerStates[i] = 'success';
		} else {
			spinnerStates[i] = 'error';
			toast.error('Errore durante il salvataggio della presenza');
		}

		clearTimeout(loadingTimeout);
		setTimeout(() => {
			spinnerStates[i] = 'idle';
		}, 5000);

		await invalidateAll();
	}

	async function callRegister() {
		const res = await fetch(`/api/admin/call_register`, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ activity_id, round })
		});

		if (!res.ok) {
			toast.error('Errore durante la richiesta di modifica delle presenze');
		}
	}

	let editMode = $state(false);
</script>

<main>
	<div class="grid grid-cols-12">
		<div class="col-span-2 flex items-center justify-center">
			<Button class="text-primary" href="/admin/rounds/{activity_id}" size="icon" variant="outline">
				<LucideArrowLeft />
			</Button>
		</div>
		<div class="col-span-8 mt-5 text-center text-3xl font-bold">
			<h1>
				{name}
			</h1>
			<h2 class="text-muted-foreground text-lg">
				{formatItalianDate(date)}
			</h2>
			<div class="mt-1 text-sm text-gray-500">
				{description}
			</div>
			<Badge class="pointer-events-none mt-2" variant="secondary">
				<LucideMapPin class="mr-1 h-4 w-4" />
				<div class="text-sm">{room}</div>
			</Badge>
			<Badge class="pointer-events-none mt-2" variant="secondary">
				<LucideUsers class="mr-1 h-4 w-4" />
				<div class="text-sm">{used_seats} / {total_seats}</div>
			</Badge>
		</div>
		<div class="col-span-2"></div>
	</div>
	<div class="mx-auto mt-5 w-[95%] max-w-[800px] pb-1">
		{#if !editMode}
			<div out:slide class="flex items-center justify-center">
				<Button
					class="mb-5 w-full"
					onclick={async () => {
						await callRegister();
						editMode = true;
					}}
					variant="secondary"
				>
					<LucidePen class="mr-1 h-4 w-4" />
					Modifica presenze
				</Button>
			</div>
		{/if}

		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>Nome</Table.Head>
					<Table.Head class="text-center">Classe</Table.Head>
					<Table.Head class="text-right">Presente?</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each presences as presence, i (presence.name)}
					<Table.Row class="rounded-xl">
						<Table.Cell>
							<Label for={'p' + i}>
								{presence.name}
							</Label>
							{#if presence.randomized}
								<LucideDice5 class="text-muted-foreground ml-1 inline h-4 w-4" />
							{/if}
						</Table.Cell>
						<Table.Cell class="text-center">
							<Label for={'p' + i}>
								{presence.class}{presence.section}
							</Label>
						</Table.Cell>
						<Table.Cell class="flex items-center justify-end">
							<div class="mr-2 h-6 w-6">
								{#if spinnerStates[i] === 'idle'}
									<div class="h-full w-full"></div>
								{:else if spinnerStates[i] === 'loading'}
									<LineMdLoadingLoop class="h-full w-full" />
								{:else if spinnerStates[i] === 'success'}
									<LineMdConfirm class="h-full w-full text-green-500" />
								{:else if spinnerStates[i] === 'error'}
									<LineMdClose class="h-full w-full text-red-500" />
								{/if}
							</div>

							<Switch
								id={'p' + i}
								bind:checked={presence.present}
								onCheckedChange={() => setPresence(i)}
								disabled={!editMode}
								class="data-[state=checked]:bg-green-500 data-[state=unchecked]:bg-red-500"
							/>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</main>
