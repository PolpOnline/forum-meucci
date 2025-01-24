import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import Icons from 'unplugin-icons/vite';
import Unfonts from 'unplugin-fonts/vite';

export default defineConfig({
	plugins: [
		sveltekit(),
		Icons({
			compiler: 'svelte',
			autoInstall: true
		}),
		Unfonts({
			fontsource: {
				families: ['Nunito Sans Variable']
			}
		})
	],
	build: {
		sourcemap: true
	}
});
