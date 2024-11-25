import type { PageServerLoad } from './$types';

const loginFailedReasons = {
	invalid_email: 'Indirizzo email non valido'
};

export const load: PageServerLoad = async ({ url }) => {
	console.log('url', url.searchParams.get('reason'));

	const message =
		loginFailedReasons[url.searchParams.get('reason') as keyof typeof loginFailedReasons] ||
		'Unknown error';

	return {
		message
	};
};