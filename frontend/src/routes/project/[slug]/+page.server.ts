import { backendUrl } from '$lib/stores';
import { get } from 'svelte/store';
import type { PageServerLoad } from './$types';
import type { Project } from '$lib/types';

export const load = (async ({ params }) => {
	try {
		const url = get(backendUrl);
		const projectResponse = await fetch(`${url}/api/v1/project/${params.slug}`, {
			method: 'GET',
			mode: 'cors',
			headers: { 'Access-Control-Allow-Origin': url },
			credentials: 'include'
		});		

		const project: Project = await projectResponse.json();
		return { project };
	} catch (e) {
		console.error(e);
	}
}) satisfies PageServerLoad;