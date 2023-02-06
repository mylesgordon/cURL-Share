import { render, screen } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import IconLink from '$lib/components/IconLink.svelte';
import type { ComponentProps } from 'svelte';

describe('IconLink component', () => {
	const props: ComponentProps<IconLink> = {
		description: 'Settings button',
		href: 'https://example.com'
	};
	testMatchingSnapshot(IconLink, props);

	it('should set href and description correctly', () => {
		render(IconLink, props);
		const link = screen.getByLabelText(props.description);
		expect(link.getAttribute('href')).toBe(props.href);
	});
});
