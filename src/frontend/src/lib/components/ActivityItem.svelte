<script lang="ts">
	import LucidePencil from '~icons/lucide/pencil';
	import ActivitySelectorDrawer from '$lib/components/available_drawer/ActivitySelectorDrawer.svelte';
	import { buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import type { components } from '$lib/api/schema';
	import LucideMapPin from '~icons/lucide/map-pin';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import LucideUsers from '~icons/lucide/users';
	import LucideUserRoundCheck from '~icons/lucide/user-round-check';
	import LucideUserRoundX from '~icons/lucide/user-round-x';

	const {
		activity,
		formattedDate,
		bookingsEndDate
	}: {
		activity: components['schemas']['Activity'];
		formattedDate: string;
		bookingsEndDate: Date;
	} = $props();

	const canEdit = new Date(bookingsEndDate) > new Date();

	const { id, name, description, round, room, used_seats, total_seats, present, date } =
		$derived(activity);
</script>

<div
	class="relative flex w-full items-center rounded-xl border"
	class:p-5={name}
	class:border-dashed={!name}
	class:custom-absent={name === 'absent'}
>
	{#if name === 'absent'}
		<div class="text-lg">Assente</div>
	{:else if name}
		<div>
			<div class="text-lg">{name}</div>
			{#if description}
				<div class="text-sm text-muted-foreground">{description}</div>
			{/if}
			<div class="space-x-1">
				<Badge class="pointer-events-none mt-2" variant="secondary">
					<LucideMapPin class="mr-1 h-4 w-4" />
					<div class="text-sm">{room}</div>
				</Badge>
				<Badge class="pointer-events-none mt-2" variant="secondary">
					<LucideUsers class="mr-1 h-4 w-4" />
					<div class="text-sm">{used_seats} / {total_seats}</div>
				</Badge>
				{#if present}
					<Badge class="pointer-events-none mt-2 border-green-600 text-green-600" variant="outline">
						<LucideUserRoundCheck class="mr-1 h-4 w-4" />
						<div class="text-sm">Presente</div>
					</Badge>
				{:else if new Date(date) < new Date()}
					<Badge class="pointer-events-none mt-2 border-red-600 text-red-600" variant="outline">
						<LucideUserRoundX class="mr-1 h-4 w-4" />
						<div class="text-sm">Assente</div>
					</Badge>
				{/if}
			</div>
		</div>
	{:else if canEdit}
		<ActivitySelectorDrawer
			{formattedDate}
			{round}
			class={cn(
				buttonVariants({ variant: 'ghost' }),
				'h-full min-h-[4.25rem] w-full rounded-xl text-lg text-muted-foreground'
			)}
		>
			{#snippet trigger()}
				+ Seleziona un'attività
			{/snippet}
		</ActivitySelectorDrawer>
	{:else}
		<div
			class="flex h-full min-h-[4.25rem] w-full items-center justify-center rounded-xl text-lg text-muted-foreground"
		>
			Nessuna attività selezionata
		</div>
	{/if}

	{#if name && canEdit}
		<ActivitySelectorDrawer
			{formattedDate}
			{round}
			class={cn('absolute right-5', buttonVariants({ variant: 'outline', size: 'icon' }))}
			initialId={id}
		>
			{#snippet trigger()}
				<LucidePencil />
			{/snippet}
		</ActivitySelectorDrawer>
	{/if}
</div>
