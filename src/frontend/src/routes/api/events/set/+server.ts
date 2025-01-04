import { client } from '$lib/api/api';

export async function PATCH({ request, fetch }) {
	const { round, event_id } = await request.json();

	const { data, response, error } = await client.PATCH('/events/set', {
		fetch,
		body: { round, event_id }
	});

	if (error) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
	}

	return new Response(JSON.stringify(data), { status: response.status });
}
