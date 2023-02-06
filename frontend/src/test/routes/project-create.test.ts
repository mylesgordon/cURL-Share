import ProjectCreatePage from '../../routes/project/create/+page.svelte';
import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';

describe('Project create page', () => {
	testMatchingSnapshot(ProjectCreatePage);

	it('should display the correct title', () => {
		render(ProjectCreatePage);
		expect(document.title).toEqual('Create Project | cURL Share');
	});

	// TODO: once api.ts has been added (or similar), mock it here and intercept the call to make sure that the correct calls are being made.
});
