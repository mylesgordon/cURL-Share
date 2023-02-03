import ProjectPage from '../../routes/project/[slug]/+page.svelte';
import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import type { ComponentProps } from 'svelte';

describe('Project page', () => {
	type ProjectPageProps = ComponentProps<ProjectPage>;
	const labels = ['IPB-xxxx', 'Feature', 'User Journey'];
	const commonProps = { labels, projectId: '1', projectName: 'Cool project' };

	const propsWithEntries: ProjectPageProps = {
		data: {
			...commonProps,
			groups: [
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
				}
			]
		}
	};

	const propsWithNoEntries: ProjectPageProps = { data: { ...commonProps, groups: [] } };

	testMatchingSnapshot(ProjectPage, propsWithEntries);
	testMatchingSnapshot(ProjectPage, propsWithNoEntries);

	it('should display the correct title', () => {
		render(ProjectPage, propsWithEntries);
		expect(document.title).toEqual('Cool project | cURL Share');
	});
});
