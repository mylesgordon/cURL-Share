import type { LayoutServerLoad } from './$types';

export const load = (async ({ fetch }) => {
	try {
		const response = await fetch(`${import.meta.env.VITE_BACKEND_BASE_URL}/api/v1/user-status`, {
			method: 'GET',
			mode: 'cors',
			headers: { 'Access-Control-Allow-Origin': 'http://localhost:8080' },
			credentials: 'include'
		});
		const responseJson = await response.json();
		const isLoggedIn = responseJson.is_logged_in ? responseJson.is_logged_in : false;
		return { isLoggedIn };
	} catch (e) {
		console.error(e);
	}
}) satisfies LayoutServerLoad;
