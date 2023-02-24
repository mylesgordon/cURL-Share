import { testMatchingSnapshot } from '../common';
import Header from '$lib/components/Header.svelte';

describe('Header component', () => {
	testMatchingSnapshot(Header);
});
