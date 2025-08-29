import { client } from '$lib/api/api';

export async function PATCH({ request, fetch }) {
	const { round, activity_id } = await request.json();

	const { data, response, error } = await client.PATCH('/forum/activities/set', {
		fetch,
		body: { round, activity_id }
	});

	if (error) {
		return new Response(`Failed to fetch: ${error}`, { status: response.status });
	}

	return new Response(JSON.stringify(data), { status: response.status });
}
