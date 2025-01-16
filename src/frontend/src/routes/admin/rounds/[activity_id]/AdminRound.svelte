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

	const { round, date, used_seats, total_seats } = $derived(roundProp);
</script>

<div class="relative flex w-full items-center rounded-xl border p-5">
	<div>
		<div class="text-lg">{formatItalianDate(date)}</div>
		<Badge class="pointer-events-none mt-2" variant="secondary">
			<LucideUsers class="mr-1 h-4 w-4" />
			<div class="text-sm">{used_seats} / {total_seats}</div>
		</Badge>
	</div>

	<Button
		class={cn(
			'absolute right-5 text-primary',
			buttonVariants({ variant: 'outline', size: 'icon' })
		)}
		href="/admin/presences/{activity_id}/{round}"
	>
		<LucideUsers />
	</Button>
</div>
