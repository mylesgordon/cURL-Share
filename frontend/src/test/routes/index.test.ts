import Index from '../../routes/+page.svelte';
import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';

describe('Index page', () => {
    testMatchingSnapshot(Index)

	it('should display the correct title', () => {
		render(Index);
		expect(document.title).toEqual('Home | cURL Share');
	});
});
