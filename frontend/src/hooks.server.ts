import type { Handle } from '@sveltejs/kit';

export const handle = (async ({ event, resolve }) => {
	event.locals.isLoggedIn = !!event.cookies.get('id');

	return await resolve(event);
}) satisfies Handle;
