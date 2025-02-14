import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = ({ locals, url }) => {
	return {
		loginStatus: locals.loginStatus,
		loggedInEmail: locals.email,
		pathname: url.pathname
	};
};
