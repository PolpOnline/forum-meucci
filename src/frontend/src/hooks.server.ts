import { API_URL } from '$lib/api/api';
import type { Handle, HandleFetch, ResolveOptions } from '@sveltejs/kit';
import type { LoginStatus } from './app';
import { default as setCookieParser } from 'set-cookie-parser';

// Cookie max age in seconds (400 days)
export const COOKIE_ABSOLUTE_MAX_AGE = 34560000;

// Forwards all cookies to the API, see https://kit.svelte.dev/docs/hooks#server-hooks-handlefetch
export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
	const isApiRequest = request.url.startsWith(API_URL);

	if (!isApiRequest) {
		return fetch(request);
	}

	// Forward all cookies to the API
	const cookies = event.request.headers.get('cookie');
	if (cookies) {
		request.headers.set('cookie', cookies);
	}

	const res = await fetch(request);

	// Check if the response contains a set-cookie header and set the cookie in to the client if it does
	const setCookieHeader = res.headers.getSetCookie();

	if (!setCookieHeader) {
		return res;
	}

	const setCookies = setCookieParser.parse(setCookieHeader);

	// Response did not contain a set-cookie header
	if (!setCookies) {
		return res;
	}

	// Forward the set-cookie header to the client
	setCookies.forEach((cookie) => {
		event.cookies.set(cookie.name, cookie.value, {
			sameSite: 'strict',
			path: '/',
			maxAge: cookie.maxAge || COOKIE_ABSOLUTE_MAX_AGE,
			httpOnly: true,
			secure: true
		});
	});

	return res;
};

// noinspection JSUnusedGlobalSymbols
export const resolveOptions: ResolveOptions = {
	preload: ({ type }) => type === 'font' || type === 'js' || type === 'css' || type === 'asset'
};

export const handle: Handle = async ({ event, resolve }) => {
	const requestedPath = event.url.pathname;
	// Auth check
	event.locals.loginStatus = event.cookies.get('meucci_forum_id')
		? 'logged_in'
		: ('logged_out' as LoginStatus);

	if (requestedPath.startsWith('/auth') || requestedPath.startsWith('/system/')) {
		return resolve(event, resolveOptions);
	}

	if (event.locals.loginStatus === 'logged_out') {
		return new Response(null, {
			status: 302,
			headers: { location: '/auth/login' }
		});
	}

	return resolve(event, resolveOptions);
};
