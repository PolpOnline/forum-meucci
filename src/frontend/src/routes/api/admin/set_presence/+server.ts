import { client } from '$lib/api/api';

export async function PATCH({ fetch, request }) {
	const { event_id, round, user_id, present } = await request.json();

	const { data, response, error } = await client.PATCH('/admin/set_presence', {
		fetch,
		body: { event_id, round, user_id, present }
	});

	if (error) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
	}

	return new Response(JSON.stringify(data), { status: response.status });
}
