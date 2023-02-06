import type { PageServerLoad } from './$types';

export const load = (({ params }) => {
	return {
		entries: [
			{
				description: 'this is a project',
				title: 'the cool project',
				link: '/project/1',
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
