import '@testing-library/jest-dom';
import { type Project, Visibility } from '$lib/types';
import { deleteProjectRequest, updateProjectRequest } from '$lib/api.page';
import { fireEvent, render, screen } from '@testing-library/svelte';
import { goto } from '$app/navigation';
import { testMatchingSnapshot } from '../common';
import ProjectSettingsPage from '../../routes/project/[slug]/settings/+page.svelte';
import userEvent from '@testing-library/user-event';
import type { ComponentProps } from 'svelte';

vi.mock('$lib/api.page');
vi.mock('$app/navigation');

describe('Project page', () => {
	type ProjectSettingsPageProps = ComponentProps<ProjectSettingsPage>;
	const props: ProjectSettingsPageProps = {
		data: {
			isLoggedIn: true,
			project: {
				admins: ['admin1', 'admin2'],
				collaborators: ['collaborator1', 'dave'],
				groups: [],
				info: {
					id: 1,
					name: 'Placeholder project',
					description: 'This is a placeholder project',
					visibility: Visibility.Public,
					environments: 'https://cool.com, https://dev.cool.com'
				}
			},
			isUnitTest: true,
			success: true,
			userAdminStatus: { isUserAdmin: true }
		}
	};
	const propsNotAdmin = JSON.parse(JSON.stringify(props));
	propsNotAdmin.data.userAdminStatus.isUserAdmin = false;

	const propsSuccessFalse = JSON.parse(JSON.stringify(props));
	propsSuccessFalse.data.success = false;

	testMatchingSnapshot(ProjectSettingsPage, props);
	testMatchingSnapshot(ProjectSettingsPage, propsNotAdmin);
	testMatchingSnapshot(ProjectSettingsPage, propsSuccessFalse);

	it('should display the correct title', () => {
		render(ProjectSettingsPage, props);
		expect(document.title).toEqual('Placeholder project settings | cURL Share');
	});

	it("should display the delete dialog and disappear when 'no' is clicked", async () => {
		render(ProjectSettingsPage, props);
		const dialog = screen.getByRole('dialog', { hidden: true });
		expect(dialog).toHaveAttribute('aria-hidden', 'true');

		const deleteButton: HTMLInputElement = screen.getByText('Delete');
		await fireEvent.click(deleteButton);

		const noButton = screen.getByText('No');
		await fireEvent.click(noButton);
		expect(dialog).toHaveAttribute('aria-hidden', 'true');
	});

	it("should send a delete request and redirect the user when 'yes' is clicked on the delete dialog", async () => {
		render(ProjectSettingsPage, props);
		const deleteButton: HTMLInputElement = screen.getByText('Delete');
		await fireEvent.click(deleteButton);

		const yesButton = screen.getByText('Yes');
		await fireEvent.click(yesButton);

		expect(deleteProjectRequest).toHaveBeenCalledWith(window.fetch, 1);
		expect(deleteProjectRequest).toHaveBeenCalledTimes(1);

		expect(goto).toHaveBeenCalledWith('/');
		expect(goto).toHaveBeenCalledTimes(1);
	});

	it('should send a correct POST request when the user updates project settings', async () => {
		render(ProjectSettingsPage, props);

		const newName = 'New Name';
		const newDescription = 'New description';

		const nameInput = screen.getByLabelText('Name');
		const descriptionInput = screen.getByLabelText('Description');
		const saveButton = screen.getByText('Save');

		await userEvent.clear(nameInput);
		await userEvent.clear(descriptionInput);

		await userEvent.type(nameInput, newName);
		await userEvent.type(descriptionInput, newDescription);
		await userEvent.click(saveButton);

		const expectedProjectData: Project = { ...props.data.project };
		expectedProjectData.info.name = newName;
		expectedProjectData.info.description = newDescription;

		expect(updateProjectRequest).toHaveBeenCalledWith(window.fetch, expectedProjectData);
		expect(updateProjectRequest).toHaveBeenCalledTimes(1);

		expect(goto).toHaveBeenCalledWith('/project/1');
	});
});
