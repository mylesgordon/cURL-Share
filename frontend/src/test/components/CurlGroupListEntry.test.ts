import CurlGroupListEntry from '$lib/components/CurlGroupListEntry.svelte';
import { screen } from '@testing-library/dom';
import '@testing-library/jest-dom/extend-expect';
import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import type { ComponentProps } from 'svelte';

describe('ProjectListEntry component', () => {
	const props: ComponentProps<CurlGroupListEntry> = {
		groupId: 1,
		projectId: '1',
		title: 'Login Journey',
		description: 'cURL group defining a user journey!',
		labels: ['label 1', 'label 2']
	};

	testMatchingSnapshot(CurlGroupListEntry, props);

	it('should render title, description and labels', () => {
		render(CurlGroupListEntry, props);
		const description = screen.getByTestId('curl-group-list-entry-1-description');
		const title = screen.getByTestId('curl-group-list-entry-1-title');
		const label1 = screen.getByTestId('curl-group-list-entry-1-label-label 1');
		const label2 = screen.getByTestId('curl-group-list-entry-1-label-label 2');
		expect(description).toHaveTextContent(props.description);
		expect(label1).toHaveTextContent(props.labels[0]);
		expect(label2).toHaveTextContent(props.labels[1]);
		expect(title).toHaveTextContent(props.title);
	});

	it('should link to the correct place', () => {
		render(CurlGroupListEntry, props);
		const title = screen.getByTestId('curl-group-list-entry-1-title');
		expect(title.getAttribute('href')).toBe('/project/1/group/1');
	});
});
