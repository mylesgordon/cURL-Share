import Meta from '../../lib/components/Meta.svelte';
import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';

describe('Meta component', () => {
	testMatchingSnapshot(Meta);

	it('should default to Unknown without an argument', () => {
		render(Meta);
		expect(document.title).toBe("Unknown | cURL Share");
	} )

	it('should use the title argument', () => {
		render(Meta, { title: "Wow, what a page!"});
		expect(document.title).toBe("Wow, what a page! | cURL Share");
	} )
});
