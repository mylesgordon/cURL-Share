import ProjectListEntry from '../../lib/components/ProjectListEntry.svelte';
import '@testing-library/jest-dom/extend-expect';
import { screen } from '@testing-library/dom';
import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import type { ComponentProps } from 'svelte';

describe('ProjectListEntry component', () => {
	const props: ComponentProps<ProjectListEntry> = {
		description: 'Test description',
		link: 'https://google.com',
		projectId: 1,
		title: 'Project title'
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
		expect(title.getAttribute('href')).toBe('https://google.com');
	});
});
