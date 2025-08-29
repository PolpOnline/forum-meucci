import { client } from '$lib/api/api';

export async function GET({ fetch, params }) {
	const round = Number(params.round);

	const { data, response, error } = await client.GET('/forum/activities/available/{round}', {
		fetch,
		params: {
			path: {
				round
			}
		}
	});

	if (error) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
	}

	return new Response(JSON.stringify(data), { status: response.status });
}
