import { sveltekit } from '@sveltejs/kit/vite';
import Icons from 'unplugin-icons/vite';
import basicSsl from '@vitejs/plugin-basic-ssl';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [basicSsl(), sveltekit(), Icons({ compiler: 'svelte' })],
	server: {
		host: '0.0.0.0'
	},
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
