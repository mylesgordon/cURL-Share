import { fetchCurlGroup } from '$lib/api.server';
import type { CurlGroup } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load = (async ({ fetch, params }) => {
	try {
		const curlGroup = await fetchCurlGroup(fetch, params.slug);
		return { success: true, curlGroup };
	} catch (e) {
		console.error(e);
		const failedCurlGroup: CurlGroup = {
			id: -1,
			curls: '[]',
			description: 'Failed to load',
			name: 'Failed',
			labels: '',
			project_id: -1
		};
		return { success: false, curlGroup: failedCurlGroup };
	}
}) satisfies PageServerLoad;
