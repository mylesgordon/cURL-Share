import LogIn from '../../routes/log-in/+page.svelte';
import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';

describe('Log In page', () => {
	testMatchingSnapshot(LogIn)

	it('should display the correct title', () => {
		render(LogIn);
		expect(document.title).toEqual('Log In | cURL Share');
	});


});