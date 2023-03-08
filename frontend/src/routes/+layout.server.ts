import { backendUrl } from '$lib/stores';
import { get } from 'svelte/store';
import type { LayoutServerLoad } from './$types';

export const load = (async ({ fetch }) => {
	try {
		const url = get(backendUrl);
		const response = await fetch(`${url}/api/v1/user-status`, {
			method: 'GET',
			mode: 'cors',
			headers: { 'Access-Control-Allow-Origin': url },
			credentials: 'include'
		});
		const responseJson = await response.json();
		const isLoggedIn = responseJson.is_logged_in ? responseJson.is_logged_in : false;
		return { isLoggedIn };
	} catch (e) {
		console.error(e);
	}
}) satisfies LayoutServerLoad;
