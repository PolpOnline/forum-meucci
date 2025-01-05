import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { redirect } from '@sveltejs/kit';
import { StatusCodes } from 'http-status-codes';

export const load: PageServerLoad = async (event) => {
	await client.GET('/auth/logout', {
		fetch: event.fetch
	});

	redirect(StatusCodes.MOVED_TEMPORARILY, '/auth/login');
};
