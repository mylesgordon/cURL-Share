import type { PageServerLoad } from './$types';
import type { projectInfo } from '$lib/types';

export const load = (async ({ fetch }) => {
	try {
		const response = await fetch(`${import.meta.env.VITE_BACKEND_BASE_URL}/api/v1/project`, {
			method: 'GET',
			mode: 'cors',
			headers: { 'Access-Control-Allow-Origin': 'http://localhost:8080' },
			credentials: 'include'
		});
		const data = (await response.json()) as projectInfo[];
		console.log(data);
		return { data };
	} catch (e) {
		console.error(e);
		return { data: [] };
	}
}) satisfies PageServerLoad;
