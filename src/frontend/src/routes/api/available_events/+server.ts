import { client } from '$lib/api/api';

export async function GET({ request, fetch }) {
	// extract round from request query params
	const url = new URL(request.url);
	const roundStr = url.searchParams.get('round');

	if (!roundStr) {
		return new Response('Missing round', { status: 400 });
	}

	const round = Number(roundStr);

	const { data, response, error } = await client.GET('/available_events', {
		fetch,
		params: {
			query: {
				round
			}
		}
	});

	if (error) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
	}

	return new Response(JSON.stringify(data), { status: response.status });
}
