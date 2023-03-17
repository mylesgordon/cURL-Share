import '@testing-library/jest-dom/extend-expect';
import { createCurlGroupRequest, updateCurlGroupRequest } from '$lib/api.page';
import { goto } from '$app/navigation';
import { render, screen } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import CurlGroupForm from '$lib/components/CurlGroupForm.svelte';
import userEvent from '@testing-library/user-event';
import type { ComponentProps } from 'svelte';
import type { CreateProjectResponse, Curl, CurlGroup } from '$lib/types';

vi.mock('$lib/api.page');
vi.mock('$app/navigation');

describe('CurlGroupForm component', () => {
	const testCreatingCurl: Array<Curl> = [
		{
			id: 1,
			name: 'cURL #1',
			description: 'Insert description here',
			rawQuery: 'curl -v google.com'
		}
	];

	const testEditingCurls: Array<Curl> = [
		{
			id: 1,
			name: 'cURL #1',
			description: 'Insert description here',
			rawQuery: 'curl -v google.com'
		},
		{
			id: 2,
			name: 'cURL #2',
			description: 'Insert another description here',
			rawQuery: 'curl -v google.com'
		}
	];

	const testCurlGroup: CurlGroup = {
		id: -1,
		curls: JSON.stringify(testCreatingCurl),
		description: 'Insert description',
		labels: '',
		name: 'New cURL Group',
		project_id: 0
	};

	const creatingProps: ComponentProps<CurlGroupForm> = {
		curlGroup: {
			id: -1,
			curls: '[]',
			description: 'Insert description',
			labels: '',
			name: 'New cURL Group',
			project_id: 0
		},
		editing: undefined,
		projectId: 0
	};

	const editingProps: ComponentProps<CurlGroupForm> = {
		curlGroup: {
			id: 1,
			curls: JSON.stringify(testEditingCurls),
			description: '',
			labels: '',
			name: '',
			project_id: 0
		},
		editing: true,
		projectId: 0
	};

	testMatchingSnapshot(CurlGroupForm, creatingProps);
	testMatchingSnapshot(CurlGroupForm, editingProps);

	it('calls the correct endpoint for creating a cURL group', async () => {
		render(CurlGroupForm, creatingProps);

		const response: CreateProjectResponse = { id: 1 };
		createCurlGroupRequest.mockResolvedValue(response);

		const curlInput = await screen.findByLabelText('Add cURL query');
		const addButton = await screen.findByText('Add');
		const createButton = await screen.findByText('Create');

		await userEvent.type(curlInput, 'curl -v google.com');
		await userEvent.click(addButton);
		await userEvent.click(createButton);

		expect(createCurlGroupRequest).toHaveBeenCalledWith(window.fetch, testCurlGroup);
		expect(goto).toHaveBeenCalledWith('/project/0/group/1');
	});

	it('calls the correct endpoint for editing a cURL group', async () => {
		render(CurlGroupForm, editingProps);

		updateCurlGroupRequest.mockResolvedValue();

		const finishButton = await screen.findByText('Finish');
		await userEvent.click(finishButton);

		expect(updateCurlGroupRequest).toHaveBeenCalledWith(window.fetch, editingProps.curlGroup);
		expect(goto).toHaveBeenCalledWith('/project/0/group/1');
	});
});
