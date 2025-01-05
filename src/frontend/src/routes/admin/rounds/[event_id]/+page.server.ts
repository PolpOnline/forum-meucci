import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, params, url }) => {
	const event_id = Number(params.event_id);

	const { data, response } = await client.GET('/admin/rounds/{event_id}', {
		fetch,
		params: { path: { event_id } }
	});

	if (response.status === StatusCodes.FORBIDDEN) {
		error(StatusCodes.FORBIDDEN, getReasonPhrase(StatusCodes.FORBIDDEN));
	}

	if (response.status === StatusCodes.UNAUTHORIZED) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/auth/login');
	}

	if (!data) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	// Used to hide the back button if there is only one event
	const one = url.searchParams.get('one')! === 'true';

	return {
		data: {
			adminRounds: data,
			event_id,
			one
		}
	};
};
