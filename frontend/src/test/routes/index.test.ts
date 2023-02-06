import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import Index from '../../routes/+page.svelte';
import type { ComponentProps } from 'svelte';

describe('Index page', () => {
	type IndexProps = ComponentProps<Index>;
	const propsWithEntries: IndexProps = {
		data: {
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
		}
	};
	const propsWithNoEntries: IndexProps = { data: { entries: [] } };

	testMatchingSnapshot(Index, propsWithEntries);
	testMatchingSnapshot(Index, propsWithNoEntries);

	it('should display the correct title', () => {
		render(Index, propsWithEntries);
		expect(document.title).toEqual('Home | cURL Share');
	});
});
