import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { StatusCodes } from 'http-status-codes';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data, response, error: errorMessage } = await client.GET('/events/selected', { fetch });

	if (response.status === StatusCodes.UNAUTHORIZED) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/login');
	}

	if (errorMessage) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, `Failed to fetch: ${errorMessage}`);
	}

	if (!data) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, 'Failed to fetch');
	}

	return {
		selectedEvents: data.events
	};
};
