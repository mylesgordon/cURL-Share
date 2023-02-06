import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import LogIn from '../../routes/log-in/+page.svelte';

describe('Log In page', () => {
	testMatchingSnapshot(LogIn);

	it('should display the correct title', () => {
		render(LogIn);
		expect(document.title).toEqual('Log In | cURL Share');
	});
});
