import { fetchProjects } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ fetch }) => {
	try {
		const data = await fetchProjects(fetch);
		return { data };
	} catch (e) {
		console.error(e);
		return { data: [] };
	}
}) satisfies PageServerLoad;
