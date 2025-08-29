<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import type { components } from '$lib/api/schema';
	import { Badge } from '$lib/components/ui/badge';
	import LucideUsers from '~icons/lucide/users';
	import { formatItalianDate } from '$lib/utils/dates';

	const {
		round: roundProp,
		activity_id
	}: { round: components['schemas']['AdminRound']; activity_id: number } = $props();

	const { round, date, present_seats, used_seats, total_seats } = $derived(roundProp);

	const badgeGreen = $derived(present_seats! > used_seats! / 2);
</script>

<div class="relative flex w-full items-center rounded-xl border p-5">
	<div>
		<div class="text-lg">{formatItalianDate(date)}</div>
		<Badge class="pointer-events-none mt-2" variant="secondary">
			<LucideUsers class="mr-1 h-4 w-4" />
			<div class="text-sm">{used_seats} / {total_seats}</div>
		</Badge>
		{#if new Date(date) < new Date()}
			<Badge
				class="pointer-events-none mt-2 ml-1
				{badgeGreen ? 'border-green-500 text-green-500' : 'border-red-500 text-red-500'}"
				variant="outline"
			>
				<LucideUsers class="mr-1 h-4 w-4" />
				<div class="text-sm">{present_seats} / {used_seats}</div>
			</Badge>
		{/if}
	</div>

	<Button
		class={cn(
			'text-primary absolute right-5',
			buttonVariants({ variant: 'outline', size: 'icon' })
		)}
		href="/forum/admin/presences/{activity_id}/{round}"
	>
		<LucideUsers />
	</Button>
</div>
