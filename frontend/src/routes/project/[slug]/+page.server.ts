import { fetchProject } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load = (async ({ fetch, params }) => {
	try {
		const project = await fetchProject(fetch, params.slug);
		return { project };
	} catch (e) {
		console.error(e);
	}
}) satisfies PageServerLoad;
