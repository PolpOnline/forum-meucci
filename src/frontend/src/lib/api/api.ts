import { dev } from '$app/environment';
import createClient from 'openapi-fetch';
import type { paths } from '$lib/api/schema';

export const API_URL = dev ? 'http://localhost:3000' : 'https://forum-api.meucci.party';

export const client = createClient<paths>({ baseUrl: API_URL });
