import type { PageServerLoad } from './$types';

export const load = (({ params }) => {
	return {
		entries: [
			{
				description: 'this is a project',
				title: 'the cool project',
				link: 'https://google.com',
				projectId: 1
			},
			{
				description: 'this is another project',
				title: 'the other project',
				link: 'https://youtube.com',
				projectId: 0
			}
		]
	};
}) satisfies PageServerLoad;
