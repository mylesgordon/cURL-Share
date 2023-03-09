import { fetchIsLoggedIn } from '$lib/api';
import type { LayoutServerLoad } from './$types';

export const load = (async ({ fetch }) => {
	let isLoggedIn = false;

	try {
		isLoggedIn = await fetchIsLoggedIn(fetch);
	} catch (e) {
		console.error(e);
	}

	return { isLoggedIn };
}) satisfies LayoutServerLoad;
