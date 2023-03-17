import { type CurlGroup, type ProjectInfo, Visibility } from '$lib/types';
import { fireEvent, render, screen } from '@testing-library/svelte';
import { get, writable } from 'svelte/store';
import { testMatchingSnapshot } from '../common';
import Search from '$lib/components/Search.svelte';
import html from 'svelte-htm';
import userEvent from '@testing-library/user-event';
import type { ComponentProps } from 'svelte';

describe('Search component', () => {
	const curlGroupArray: Array<CurlGroup> = [
		{
			id: 1,
			project_id: 1,
			name: 'Login Journey',
			description: 'cURL group defining a user journey!',
			labels: 'label 1,label 2',
			curls: ''
		},
		{
			id: 2,
			project_id: 1,
			name: 'Another Journey',
			description: 'cURL group defining another journey!',
			labels: 'label 3,label 4',
			curls: ''
		}
	];
	const projectInfoArray: Array<ProjectInfo> = [
		{
			id: 1,
			name: 'Placeholder project',
			description: 'This is a placeholder project',
			visibility: Visibility.Public,
			environments: 'https://cool.com, https://dev.cool.com'
		},
		{
			id: 2,
			name: 'Another project',
			description: 'This is a placeholder project',
			visibility: Visibility.Public,
			environments: 'https://cool.com, https://dev.cool.com'
		},
		{
			id: 3,
			name: 'Final project',
			description: 'This is a placeholder project',
			visibility: Visibility.Public,
			environments: 'https://cool.com, https://dev.cool.com'
		}
	];

	const curlGroupProps: ComponentProps<Search> = { curlGroupArray };
	const projectInfoProps: ComponentProps<Search> = { projectInfoArray };

	testMatchingSnapshot(Search, curlGroupProps);
	testMatchingSnapshot(Search, projectInfoProps);

	it('searches project info correctly', async () => {
		const projectInfo = writable([...projectInfoArray]);
		render(html`<${Search} bind:projectInfoArray=${projectInfo} />`);

		const input = await screen.findByTestId('search-input');
		await userEvent.type(input, 'Another project');

		expect(get(projectInfo)).toStrictEqual([projectInfoArray[1]]);

		await userEvent.clear(input);
		expect(get(projectInfo)).toStrictEqual(projectInfoArray);
	});

	it('searches curl groups correctly', async () => {
		const curlGroups = writable([...curlGroupArray]);
		render(html`<${Search} bind:curlGroupArray=${curlGroups} />`);

		const input = await screen.findByTestId('search-input');
		await userEvent.type(input, 'Another journey');

		expect(get(curlGroups)).toStrictEqual([curlGroupArray[1]]);

		await userEvent.clear(input);
		expect(get(curlGroups)).toStrictEqual(curlGroupArray);

		const labelsDropdown = await screen.getByTestId('labels-select');
		await fireEvent.change(labelsDropdown, { target: { value: 'label 2' } });

		expect(get(curlGroups)).toStrictEqual([curlGroupArray[0]]);
	});
});
