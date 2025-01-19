import type { PageServerLoad } from './$types';
import { client } from '$lib/api/api';
import { getReasonPhrase, StatusCodes } from 'http-status-codes';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	// Check the user type, if the user is a host or admin, redirect to the admin page
	await checkUserType({ fetch });

	const {
		data,
		response,
		error: errorMessage
	} = await client.GET('/activities/selected', { fetch });

	if (response.status === StatusCodes.UNAUTHORIZED) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/auth/login');
	}

	// Too Early
	if (response.status === 425) {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/countdown');
	}

	if (errorMessage) {
		console.error(errorMessage);
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	if (!data) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	return {
		selectedActivities: data.activities
	};
};

async function checkUserType({ fetch }: typeof load.arguments) {
	const { data: typeInfo } = await client.GET('/user/my_type', { fetch });

	// Check the user type, if the user is a host or admin, redirect to the admin page
	if (!typeInfo) {
		error(StatusCodes.INTERNAL_SERVER_ERROR, getReasonPhrase(StatusCodes.INTERNAL_SERVER_ERROR));
	}

	if (typeInfo.user_type === 'host' || typeInfo.user_type === 'admin') {
		redirect(StatusCodes.MOVED_TEMPORARILY, '/admin');
	}
}
