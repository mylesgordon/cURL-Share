import { backendUrl } from '$lib/stores';
import { get } from 'svelte/store';
import type { PageServerLoad } from './$types';
import type { ProjectInfo } from '$lib/types';

export const load = (async ({ fetch }) => {
	try {
		const url = get(backendUrl);
		const response = await fetch(`${url}/api/v1/project`, {
			method: 'GET',
			mode: 'cors',
			headers: { 'Access-Control-Allow-Origin': url },
			credentials: 'include'
		});
		const data = (await response.json()) as ProjectInfo[];
		console.log(data);
		return { data };
	} catch (e) {
		console.error(e);
		return { data: [] };
	}
}) satisfies PageServerLoad;
