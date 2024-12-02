<script lang="ts">
	import EventItem from '$lib/components/EventItem.svelte';
	import { capitalizeFirstLetter } from '$lib/utils/text';
	import { title } from '$lib/stores/title.store';

	// let { data }: { data: PageData } = $props();

	type MockEvent = {
		round: number;
		name: string;
		date: string;
	};

	const mockSelectedEvents: MockEvent[] = [
		{ round: 1, name: '', date: '2022-01-01T12:00:00Z' },
		{ round: 2, name: 'absent', date: '2022-02-02T12:00:00Z' },
		{ round: 3, name: 'Evento 3', date: '2022-03-03T12:00:00Z' },
		{ round: 4, name: 'Evento 4', date: '2022-04-04T12:00:00Z' }
	];

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
		{#each mockSelectedEvents as event}
			<div class="my-4">
				<div class="font-semibold">
					{formatItalianDate(event.date)}
				</div>
				<div class="mt-3">
					<EventItem
						round={event.round}
						name={event.name}
						formattedDate={formatItalianDate(event.date)}
					/>
				</div>
			</div>
		{/each}
	</div>
</main>
