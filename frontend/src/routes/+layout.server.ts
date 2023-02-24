import type { LayoutServerLoad } from './$types';

export const load = (async ({ fetch }) => {
	const response = await fetch('http://localhost:8080/api/v1/user-status', {
		method: 'GET',
		mode: 'cors',
		headers: { 'Access-Control-Allow-Origin': 'http://localhost:8080' },
		credentials: 'include'
	});
	const responseJson = await response.json();
	const isLoggedIn = responseJson.is_logged_in ? responseJson.is_logged_in : false;

	return { isLoggedIn };
}) satisfies LayoutServerLoad;
