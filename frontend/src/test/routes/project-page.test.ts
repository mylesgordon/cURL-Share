import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import ProjectPage from '../../routes/project/[slug]/+page.svelte';
import type { ComponentProps } from 'svelte';

describe('Project page', () => {
	// TODO: private project

	type ProjectPageProps = ComponentProps<ProjectPage>;
	const labels = ['IPB-xxxx', 'Feature', 'User Journey'];
	const commonProjectProps = {
		admins: [],
		collaborators: [],
		info: {
			labels: labels.join(','),
			id: 1,
			name: 'Cool project',
			description: 'Really cool',
			environments: '',
			visibility: 'Public'
		}
	};

	const propsWithEntries: ProjectPageProps = {
		data: {
			adminStatus: { isUserAdmin: true },
			isLoggedIn: true,
			project: {
				...commonProjectProps,
				groups: [
					{
						id: 1,
						curls: '',
						project_id: commonProjectProps.info.id,
						name: 'Login Journey',
						description: 'cURL group defining a user journey!',
						labels: `${labels[1]},${labels[2]}`
					},
					{
						id: 2,
						curls: '',
						project_id: commonProjectProps.info.id,
						name: 'IPB-1293',
						description:
							'Lorem ipsum this is long text Lorem ipsum this is long text Lorem ipsum this is long text Lorem ipsum this is long text Lorem ipsum this is long text Lorem ipsum this is long text ',
						labels: `${labels[0]},${labels[1]},${labels[2]}`
					}
				]
			}
		}
	};

	const propsWithNoEntries: ProjectPageProps = {
		data: {
			adminStatus: { isUserAdmin: false },
			isLoggedIn: true,
			project: { ...commonProjectProps, groups: [] }
		}
	};

	testMatchingSnapshot(ProjectPage, propsWithEntries);
	testMatchingSnapshot(ProjectPage, propsWithNoEntries);

	it('should display the correct title', () => {
		render(ProjectPage, propsWithEntries);
		expect(document.title).toEqual('Cool project | cURL Share');
	});
});
