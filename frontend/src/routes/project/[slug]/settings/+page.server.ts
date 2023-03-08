import { fetchProject } from '$lib/api';
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
	let success = false;

	try {
		project = await fetchProject(fetch, params.slug);
		success = true;
	} catch (e) {
		console.error(e);
	}

	return { project, isUnitTest: false, success };
}) satisfies PageServerLoad;
