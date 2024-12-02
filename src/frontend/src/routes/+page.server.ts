import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data } = await client.GET('/me', { fetch });

	if (!data) {
		return {
			status: 401,
			redirect: '/login'
		};
	}

	return {
		availableEvents: [],
		email: data.email,
		name: data.name
	};
};
