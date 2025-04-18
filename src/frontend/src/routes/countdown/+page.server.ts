import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	const {
		data,
		response,
		error: errorMessage
	} = await client.GET('/registrations_start_date', { fetch });

	if (response.status === StatusCodes.UNAUTHORIZED) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/auth/login');
	}

	if (errorMessage) {
		console.error(errorMessage);
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	if (!data) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	if (new Date() > new Date(data.registrations_start_date)) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/');
	}

	return {
		startDate: data.registrations_start_date
	};
};
