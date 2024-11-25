import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async (event) => {
	await client.GET('/auth/logout', {
		fetch: event.fetch
	});

	redirect(302, '/login');
};
