import '@testing-library/jest-dom/extend-expect';
import { render } from '@testing-library/svelte';
import { screen } from '@testing-library/dom';
import { testMatchingSnapshot } from '../common';
import ProjectListEntry from '$lib/components/ProjectListEntry.svelte';
import type { ComponentProps } from 'svelte';

describe('ProjectListEntry component', () => {
	const props: ComponentProps<ProjectListEntry> = {
		description: 'Test description',
		id: 1,
		name: 'Project title',
		index: 0,
		length: 1
	};
	testMatchingSnapshot(ProjectListEntry, props);

	it('should render title and description correctly', () => {
		render(ProjectListEntry, props);
		const description = screen.getByTestId('project-list-entry-1-description');
		const title = screen.getByTestId('project-list-entry-1-title');
		expect(description).toHaveTextContent('Test description');
		expect(title).toHaveTextContent('Project title');
	});

	it('should link to the correct place', () => {
		render(ProjectListEntry, props);
		const title = screen.getByTestId('project-list-entry-1-title');
		expect(title.getAttribute('href')).toBe('/project/1');
	});
});
