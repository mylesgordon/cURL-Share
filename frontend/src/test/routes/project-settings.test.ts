import '@testing-library/jest-dom';
import { Visibility } from '$lib/types';
import { fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import ProjectSettingsPage from '../../routes/project/[slug]/settings/+page.svelte';
import type { ComponentProps } from 'svelte';

describe('Project page', () => {
	type ProjectSettingsPageProps = ComponentProps<ProjectSettingsPage>;
	const props: ProjectSettingsPageProps = {
		data: {
            projectSettings: {
    	        id: 1,
    	        name: 'Placeholder project',
    	        description: 'This is a placeholder project',
    	        visibility: Visibility.Public,
    	        collaborators: ['collaborator1', 'dave'],
    	        environments: ['https://cool.com', 'https://dev.cool.com']
            }
		}
	};

	testMatchingSnapshot(ProjectSettingsPage, props);

	it('should display the correct title', () => {
		render(ProjectSettingsPage, props);
		expect(document.title).toEqual('Placeholder project settings | cURL Share');
	});

    // it('should display the delete dialog and disappear when \'no\' is clicked', async () => {
	// 	const page = render(ProjectSettingsPage, props);
    //     const dialog = screen.getByLabelText('deleteProjectDialog');
    //     expect(dialog).toHaveAttribute('aria-hidden', 'true');

	// 	const deleteButton:HTMLInputElement = screen.getByText("Delete");
    //     console.log(deleteButton)
    //     await fireEvent.click(deleteButton);
    //     screen.getByLabelText('deleteProjectDialog');
         
    //     // await waitFor(() =>expect(dialog).not.toHaveAttribute('aria-hidden'));

    //     const noButton = screen.getByTestId('delete-dialog-no-button');
    //     await fireEvent.click(noButton);
    //     expect(dialog).toHaveAttribute('aria-hidden', 'true');
    // });

    // it should send a delete request 
    // it should send a correct post request
});

