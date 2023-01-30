import Index from './+page.svelte';
import { render, screen } from '@testing-library/svelte';

it('should display the correct title', () => {
	render(Index);
	expect(document.title).toEqual("Home | cURL Share");
})
