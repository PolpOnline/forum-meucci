import { client } from '$lib/api/api';

export async function PATCH({ fetch, request }) {
	const { activity_id, round } = await request.json();

	const { data, response, error } = await client.PATCH('/forum/admin/call_register', {
		fetch,
		body: { activity_id, round }
	});

	if (error) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
	}

	return new Response(JSON.stringify(data), { status: response.status });
}
