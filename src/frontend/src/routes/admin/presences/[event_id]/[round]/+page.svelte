<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import LucideMapPin from '~icons/lucide/map-pin';
	import LucideArrowLeft from '~icons/lucide/arrow-left';
	import { Button } from '$lib/components/ui/button';
	import { Switch } from '$lib/components/ui/switch';
	// noinspection ES6UnusedImports
	import * as Table from '$lib/components/ui/table/index.js';

	let { data } = $props();

	// Use a state to be able to update the data reactively
	const { event_id, adminPresences } = $state(data.data);
	const { name, room, presences } = $state(adminPresences);
</script>

<main>
	<div class="grid grid-cols-12">
		<div class="col-span-2 flex items-center justify-center">
			<Button class="text-primary" href="/admin/rounds/{event_id}" size="icon" variant="outline">
				<LucideArrowLeft />
			</Button>
		</div>
		<h1 class="col-span-8 mt-5 text-center text-3xl font-bold">
			{name}
			<br />
			<Badge class="pointer-events-none mt-2" variant="secondary">
				<LucideMapPin class="mr-1 h-4 w-4" />
				<div class="translate-y-[5%] text-sm">{room}</div>
			</Badge>
		</h1>
		<div class="col-span-2"></div>
	</div>
	<Table.Root class="mx-auto mt-5 w-[95%] max-w-[800px]">
		<Table.Body>
			{#each presences as presence, i (presence.name)}
				<Table.Row class="rounded-xl">
					<Table.Cell class="">
						{presence.name}
					</Table.Cell>
					<Table.Cell class="flex items-center justify-center">
						<Switch id={'p' + i} bind:checked={presence.present} />
					</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>
</main>
