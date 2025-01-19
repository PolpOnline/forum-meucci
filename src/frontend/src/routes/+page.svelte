<script lang="ts">
	import ActivityItem from '$lib/components/ActivityItem.svelte';
	import { title } from '$lib/stores/title.store';
	import type { PageData } from './$types';
	import { flip } from 'svelte/animate';
	import { formatItalianDate } from '$lib/utils/dates';
	import { fly } from 'svelte/transition';
	import { hide } from '$lib/animations/hide';

	let { data }: { data: PageData } = $props();

	const selectedActivities = $derived(data.selectedActivities!);
	const registrationsEndDate = $derived(new Date(data.registrationsEndDate));

	title.set('Forum Meucci');
</script>

<main>
	<div class="mx-auto mt-5 grid w-[95%] max-w-[800px] space-y-6">
		{#each selectedActivities as activity (`${activity.round}-${activity.id}`)}
			{@const formattedDate = formatItalianDate(activity.date)}
			<div animate:flip class="custom-grid-element" style="--row: {activity.round + 1}">
				<!-- Little hack to make the dates not overlap -->
				<div class="ml-1 font-bold" out:hide={{ duration: 1, delay: 0 }}>
					{formattedDate}
				</div>
				<div
					class="mt-2"
					in:fly={{ x: '-100%', delay: 500, duration: 500 }}
					out:fly={{ x: '100%', duration: 500 }}
				>
					<ActivityItem {activity} {formattedDate} {registrationsEndDate} />
				</div>
			</div>
		{/each}
	</div>
</main>

<style>
	.custom-grid-element {
		grid-row: var(--row);
		grid-column: 1;
	}
</style>
