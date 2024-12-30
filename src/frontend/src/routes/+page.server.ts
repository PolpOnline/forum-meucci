import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data } = await client.GET('/selected_events', { fetch });

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
