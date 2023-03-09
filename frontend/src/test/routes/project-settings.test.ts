import '@testing-library/jest-dom';
import { Visibility } from '$lib/types';
import { fireEvent, render, screen } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import ProjectSettingsPage from '../../routes/project/[slug]/settings/+page.svelte';
import type { ComponentProps } from 'svelte';

describe('Project page', () => {
	type ProjectSettingsPageProps = ComponentProps<ProjectSettingsPage>;
	const props: ProjectSettingsPageProps = {
		data: {
			isLoggedIn: true,
			project: {
				admins: ['admin1, admin2'],
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

	testMatchingSnapshot(ProjectSettingsPage, props);
	testMatchingSnapshot(ProjectSettingsPage, propsNotAdmin);

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

	// TODO success: false
	// it should send a delete request
	// it should send a correct post request
});
