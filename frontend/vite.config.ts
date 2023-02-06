import { sveltekit } from '@sveltejs/kit/vite';
import Icons from 'unplugin-icons/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit(), Icons({ compiler: 'svelte' })],
	test: {
		environment: 'jsdom',
		globals: true,
		include: ['src/**/*.{test,spec}.{js,ts}'],
		coverage: {
			exclude: ['~icons/**']
		}
	}
};

export default config;
