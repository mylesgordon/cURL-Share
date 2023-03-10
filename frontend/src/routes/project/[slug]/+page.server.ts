import { fetchProject, fetchProjectAdminStatus } from '$lib/api.server';
import type { PageServerLoad } from './$types';

export const load = (async ({ fetch, params }) => {
	try {
		const project = await fetchProject(fetch, params.slug);
		const adminStatus = await fetchProjectAdminStatus(fetch, params.slug);
		return { adminStatus, project };
	} catch (e) {
		console.error(e);
		return { adminStatus: { isUserAdmin: false } };
	}
}) satisfies PageServerLoad;
