import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	// Check the user type, if the user is a host or admin, redirect to the admin page
	const {
		data,
		response,
		error: errorMessage
	} = await client.GET('/activities/selected', { fetch });

	// Too early to set activities
	if (response.status === 425) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/countdown');
	}

	// User is not authenticated
	if (response.status === StatusCodes.UNAUTHORIZED) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/auth/login');
	}

	// User is an admin
	if (response.status === StatusCodes.FORBIDDEN) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/admin');
	}

	if (errorMessage) {
		console.error(errorMessage);
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	if (!data) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	return {
		bookingsEndDate: data.bookings_end_date,
		selectedActivities: data.activities
	};
};
