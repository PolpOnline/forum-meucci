import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const event_id = Number(params.event_id);
	const round = Number(params.round);

	const { data, response } = await client.GET('/admin/presences/{event_id}/{round}', {
		fetch,
		params: { path: { event_id, round } }
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

	return {
		data: {
			adminPresences: data,
			event_id,
			round
		}
	};
};
