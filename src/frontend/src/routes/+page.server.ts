import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data } = await client.GET('/events/selected', { fetch });

	if (!data) {
		return {
			status: 401,
			redirect: '/login'
		};
	}

	return {
		selectedEvents: data.events
	};
};
