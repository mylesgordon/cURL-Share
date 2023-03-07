import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import Index from '../../routes/+page.svelte';
import type { ComponentProps } from 'svelte';

describe('Index page', () => {
	type IndexProps = ComponentProps<Index>;
	const propsWithEntries: IndexProps = {
		data: {
			data: [
				{
					description: 'this is a project',
					name: 'the cool project',
					id: 1,
					environments: 'localhost,http://other.com',
					visibility: 'Public'
				},
				{
					description: 'this is another project',
					name: 'the other project',
					id: 0,
					environments: 'http://irrelevant.com',
					visibility: 'Public'
				}
			]
		}
	};
	const propsWithNoEntries: IndexProps = { data: { data: [] } };

	testMatchingSnapshot(Index, propsWithEntries);
	testMatchingSnapshot(Index, propsWithNoEntries);

	it('should display the correct title', () => {
		render(Index, propsWithEntries);
		expect(document.title).toEqual('Home | cURL Share');
	});
});
