import '@testing-library/jest-dom/extend-expect';
import { get, writable } from 'svelte/store';
import { render, screen } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import AgnosticInput from '$lib/components/AgnosticInput.svelte';
import html from 'svelte-htm';
import userEvent from '@testing-library/user-event';
import type { ComponentProps } from 'svelte';

describe('AgnosticInput component', () => {
	const props: ComponentProps<AgnosticInput> = {
		id: 'unit-test',
		label: 'Unit Test',
		value: 'Input'
	};
	testMatchingSnapshot(AgnosticInput, props);

	it('updates state correctly', async () => {
		const value = writable('Input');
		render(html`<${AgnosticInput} id="unit-test" label="Unit Test" bind:value=${value} />`);

		const input = await screen.findByLabelText('Unit Test');
		await userEvent.type(input, ' - now updated!');

		expect(get(value)).toBe('Input - now updated!');
	});
});
