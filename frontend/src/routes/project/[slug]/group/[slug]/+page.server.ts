import { fetchCurlGroup, fetchProject, fetchProjectAdminStatus } from '$lib/api.server';
import type { CurlGroup, ProjectAdminStatus } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load = (async ({ fetch, params }) => {
	try {
		const curlGroup = await fetchCurlGroup(fetch, params.slug);
		const project = await fetchProject(fetch, curlGroup.project_id.toString());
		const projectAdminStatus = await fetchProjectAdminStatus(
			fetch,
			curlGroup.project_id.toString()
		);
		return { success: true, curlGroup, projectAdminStatus, project };
	} catch (e) {
		console.error(e);
		const projectAdminStatus: ProjectAdminStatus = { isUserAdmin: false };
		const failedCurlGroup: CurlGroup = {
			id: -1,
			curls: '[]',
			description: 'Failed to load',
			name: 'Failed',
			labels: '',
			project_id: -1
		};
		return { success: false, projectAdminStatus, curlGroup: failedCurlGroup };
	}
}) satisfies PageServerLoad;
