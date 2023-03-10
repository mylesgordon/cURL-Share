import { fetchProject, fetchProjectAdminStatus } from '$lib/api.server';
import type { PageServerLoad } from './$types';
import type { Project } from '$lib/types';

export const load = (async ({ fetch, params }) => {
	// dummy project so project isn't undefined
	let project: Project = {
		info: { id: -1, environments: '', description: '', name: '', visibility: '' },
		admins: [],
		collaborators: [],
		groups: []
	};
	let userAdminStatus = { isUserAdmin: false };
	let success = false;

	try {
		project = await fetchProject(fetch, params.slug);
		userAdminStatus = await fetchProjectAdminStatus(fetch, params.slug);
		success = true;
	} catch (e) {
		console.error(e);
	}

	return { project, isUnitTest: false, success, userAdminStatus };
}) satisfies PageServerLoad;
