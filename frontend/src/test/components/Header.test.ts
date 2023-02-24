import { isLoggedIn } from '$lib/stores';
import { render } from '@testing-library/svelte';
import { testMatchingSnapshot } from '../common';
import Header from '$lib/components/Header.svelte';

describe('Header component', () => {
	let isLoggedInValue: boolean;
	isLoggedIn.subscribe((value) => {
		isLoggedInValue = value;
	});

	testMatchingSnapshot(Header, { isLoggedInResponse: false });
	testMatchingSnapshot(Header, { isLoggedInResponse: true });

	it('should update isLoggedIn store with correct value', () => {
		const testValues = [false, true];
		testValues.forEach((testValue) => {
			render(Header, { isLoggedInResponse: testValue });
			expect(isLoggedInValue).toBe(testValue);
		});
	});
});
