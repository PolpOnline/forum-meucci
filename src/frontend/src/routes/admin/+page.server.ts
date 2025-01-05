import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data, response } = await client.GET('/admin/events', { fetch });

	if (response.status === StatusCodes.FORBIDDEN) {
		error(StatusCodes.FORBIDDEN, getReasonPhrase(StatusCodes.FORBIDDEN));
	}

	if (response.status === StatusCodes.UNAUTHORIZED) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/auth/login');
	}

	if (!data) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	// Auto redirect to the first event if the length is 1
	if (data.events.length === 1) {
		redirect(StatusCodes.MOVED_TEMPORARILY, `/admin/rounds/${data.events[0]!.id}?one=true`);
	}

	return {
		adminEvents: data.events
	};
};
