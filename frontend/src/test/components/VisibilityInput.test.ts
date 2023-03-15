import { Visibility } from '$lib/types';
import { render, screen } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import VisibilityInput from '$lib/components/VisibilityInput.svelte';
import userEvent from '@testing-library/user-event';
import type { ComponentProps } from 'svelte';

describe('VisibilityInput component', () => {
	const props: ComponentProps<VisibilityInput> = {
		visibility: Visibility.Public
	};
	testMatchingSnapshot(VisibilityInput, props);

	it.each([
		{ visibility: Visibility.Public, publicChecked: true, privateChecked: false },
		{ visibility: Visibility.Private, publicChecked: false, privateChecked: true }
	])(
		'$visibility should default with public button checked $publicChecked, private button checked $privateChecked',
		({ visibility, publicChecked, privateChecked }) => {
			render(VisibilityInput, { visibility });
			const publicButton: HTMLInputElement = screen.getByLabelText('Public');
			const privateButton: HTMLInputElement = screen.getByLabelText('Private');
			expect(publicButton.checked).toEqual(publicChecked);
			expect(privateButton.checked).toEqual(privateChecked);
		}
	);

	it('should be able to switch between public and private', async () => {
		render(VisibilityInput, props);
		const publicButton: HTMLInputElement = screen.getByLabelText('Public');
		const privateButton: HTMLInputElement = screen.getByLabelText('Private');

		await userEvent.click(privateButton);
		expect(publicButton.checked).toEqual(false);
		expect(privateButton.checked).toEqual(true);

		await userEvent.click(publicButton);
		expect(publicButton.checked).toEqual(true);
		expect(privateButton.checked).toEqual(false);
	});
});
