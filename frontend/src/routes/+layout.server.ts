import { fetchIsLoggedIn } from '$lib/api';
import type { LayoutServerLoad } from './$types';

export const load = (async ({ fetch }) => {
	try {
		const isLoggedIn = await fetchIsLoggedIn(fetch);
		return { isLoggedIn };
	} catch (e) {
		console.error(e);
	}
}) satisfies LayoutServerLoad;
