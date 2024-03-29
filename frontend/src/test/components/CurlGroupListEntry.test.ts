import '@testing-library/jest-dom/extend-expect';
import { render } from '@testing-library/svelte';
import { screen } from '@testing-library/dom';
import { testMatchingSnapshot } from '../common';
import CurlGroupListEntry from '$lib/components/CurlGroupListEntry.svelte';
import type { ComponentProps } from 'svelte';

describe('ProjectListEntry component', () => {
	const props: ComponentProps<CurlGroupListEntry> = {
		id: 1,
		project_id: 1,
		name: 'Login Journey',
		description: 'cURL group defining a user journey!',
		labels: 'label 1,label 2',
		index: 0,
		length: 1
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
		expect(title).toHaveTextContent(props.name);
	});

	it('should link to the correct place', () => {
		render(CurlGroupListEntry, props);
		const title = screen.getByTestId('curl-group-list-entry-1-title');
		expect(title.getAttribute('href')).toBe('/project/1/group/1');
	});
});
