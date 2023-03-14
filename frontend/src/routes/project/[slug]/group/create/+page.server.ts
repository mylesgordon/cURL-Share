import type { PageServerLoad } from './$types';

export const load = (async ({ params }) => {
	const projectId = parseInt(params.slug) ?? -1;
	return { projectId };
}) satisfies PageServerLoad;
