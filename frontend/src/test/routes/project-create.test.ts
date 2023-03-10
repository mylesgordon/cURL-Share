import { createProjectRequest } from '$lib/api.page';
import { getByText, render } from '@testing-library/svelte';
import { goto } from '$app/navigation';
import { screen } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import ProjectCreatePage from '../../routes/project/create/+page.svelte';
import userEvent from '@testing-library/user-event';
import type { CreateProjectResponse, ProjectInfo } from '$lib/types';

vi.mock('$app/navigation');
vi.mock('$lib/api.page');

describe('Project create page', () => {
	function getPageElements() {
		const nameField = screen.getByLabelText('Name');
		const descriptionField = screen.getByLabelText('Description');
		const createButton = screen.getByRole('button', { name: /Create/i });

		return { nameField, descriptionField, createButton };
	}

	testMatchingSnapshot(ProjectCreatePage);

	it('should display the correct title', () => {
		render(ProjectCreatePage);
		expect(document.title).toEqual('Create Project | cURL Share');
	});

	it('should display an error message when creating a project with no name or description', async () => {
		render(ProjectCreatePage);
		const { nameField, descriptionField, createButton } = getPageElements();

		const table = [
			{ fieldWithText: nameField, fieldToClear: descriptionField },
			{ fieldWithText: descriptionField, fieldToClear: nameField }
		];

		for (const { fieldWithText, fieldToClear } of table) {
			await userEvent.type(fieldWithText, 'cool text');
			await userEvent.clear(fieldToClear);
			await userEvent.click(createButton);

			await screen.getByText('Name or description fields empty - please try again.');
		}
	});

	// TODO
	// it('should display an error message if the server sends a bad response',async () => {
	// 	render(ProjectCreatePage);
	// 	const {nameField, descriptionField, createButton} = getPageElements();
	// 	const error = new Error("Something went wrong!");
	// 	createProjectRequest.mockRejectedValue(error);

	// 	await userEvent.type(nameField, "my project");
	// 	await userEvent.type(descriptionField, "this is a jolly project");
	// 	await userEvent.click(createButton);

	// 	await screen.findByText("Could not create project - please try again.");
	// });

	it('should send the correct data and redirect if project creation is successful', async () => {
		const requestObject: ProjectInfo = {
			id: 0,
			environments: '',
			description: 'Cool project',
			name: 'Project 1',
			visibility: 'Private'
		};
		const mockResolvedResponse: CreateProjectResponse = { id: 1 };

		render(ProjectCreatePage);
		const { nameField, descriptionField, createButton } = getPageElements();
		const privateButton = await screen.getByText('Private');

		createProjectRequest.mockResolvedValue(mockResolvedResponse);

		await userEvent.type(nameField, requestObject.name);
		await userEvent.type(descriptionField, requestObject.description);
		await userEvent.click(privateButton);
		await userEvent.click(createButton);

		expect(createProjectRequest).toHaveBeenCalledWith(window.fetch, requestObject);
		expect(goto).toHaveBeenCalledWith('/project/1');
	});
});
