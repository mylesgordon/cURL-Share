import { render } from '@testing-library/svelte';
import type { ComponentProps, SvelteComponent } from 'svelte';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Constructor<T> = new (...args: any[]) => T;

export const testMatchingSnapshot = <C extends SvelteComponent>(
	component: Constructor<C>,
	props?: ComponentProps<C>
) =>
	it(`should match snapshot`, () => {
		const { container } = render(component, props);
		expect(container).toMatchSnapshot();
	});
