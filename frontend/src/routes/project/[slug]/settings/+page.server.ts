import { Visibility } from '$lib/types';
import type { PageServerLoad } from './$types';

const projectSettings = {
	id: 1,
	name: 'Placeholder project',
	description: 'This is a placeholder project',
	visibility: Visibility.Public,
	collaborators: ['collaborator1', 'dave'],
	environments: ['https://cool.com', 'https://dev.cool.com']
};

export const load = (() => {
	return {
		projectSettings,
		isUnitTest: false
	};
}) satisfies PageServerLoad;
