<script lang="ts">
	import ActivityItem from '$lib/components/ActivityItem.svelte';
	import { title } from '$lib/stores/title.store';
	import type { PageData } from './$types';
	import { flip } from 'svelte/animate';
	import { formatItalianDate } from '$lib/utils/dates';

	let { data }: { data: PageData } = $props();

	let selectedActivities = $derived(data.selectedActivities!);

	title.set('Forum Meucci');
</script>

<main>
	<div class="mx-auto mt-5 flex w-[95%] max-w-[800px] flex-col space-y-6">
		{#each selectedActivities as activity (activity.round)}
			<div animate:flip>
				<div class="ml-1 font-bold">
					{formatItalianDate(activity.date)}
				</div>
				<div class="mt-2">
					<ActivityItem {activity} formattedDate={formatItalianDate(activity.date)} />
				</div>
			</div>
		{/each}
	</div>
</main>
