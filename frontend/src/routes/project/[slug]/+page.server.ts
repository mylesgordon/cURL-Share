import type { PageServerLoad } from './$types';

const labels = ['IPB-1293', 'Feature', 'User Journey'];

const groups = [
	{
		groupId: 1,
		title: 'Login Journey',
		description: 'cURL group defining a user journey!',
		labels: [labels[1], labels[2]]
	},
	{
		groupId: 2,
		title: 'IPB-1293',
		description:
			'Lorem ipsum this is long text Lorem ipsum this is long text Lorem ipsum this is long text Lorem ipsum this is long text Lorem ipsum this is long text Lorem ipsum this is long text ',
		labels: [labels[0], labels[1], labels[2]]
	},
	{
		groupId: 3,
		title: 'Cool journey',
		description: '',
		labels: [labels[2]]
	},
	{
		groupId: 4,
		title: 'Feature 1',
		description: '',
		labels: [labels[1]]
	}
];

export const load = (({ params }) => {
	return {
		groups,
		labels,
		projectId: params.slug,
		projectName: 'Placeholder Project'
	};
}) satisfies PageServerLoad;
