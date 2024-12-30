<script lang="ts">
	import EventItem from '$lib/components/EventItem.svelte';
	import { capitalizeFirstLetter } from '$lib/utils/text';
	import { title } from '$lib/stores/title.store';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let selectedEvents = data.selectedEvents!;

	function formatItalianDate(isoTimestamp: string): string {
		const date = new Date(isoTimestamp);

		const options: (typeof Intl.DateTimeFormat.arguments)[1] = {
			weekday: 'long',
			day: 'numeric',
			month: 'numeric',
			hour: 'numeric',
			minute: 'numeric'
		};

		const formattedDate = new Intl.DateTimeFormat('it-IT', options)
			.format(date)
			.replace(',', ' alle');

		return capitalizeFirstLetter(formattedDate);
	}

	title.set('Forum Meucci');
</script>

<main>
	<div class="w-100 mx-auto mt-5 w-[95%] max-w-[800px]">
		{#each selectedEvents as event (event.round)}
			<div class="my-4">
				<div class="font-semibold">
					{formatItalianDate(event.date)}
				</div>
				<div class="mt-3">
					<EventItem
						round={event.round}
						name={event.name}
						description={event.description}
						formattedDate={formatItalianDate(event.date)}
					/>
				</div>
			</div>
		{/each}
	</div>
</main>
