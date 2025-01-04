import { capitalizeFirstLetter } from '$lib/utils/text';

export function formatItalianDate(isoTimestamp: string): string {
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
