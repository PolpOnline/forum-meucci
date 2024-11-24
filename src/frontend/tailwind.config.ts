// @ts-expect-error EsModuleInterop is working
import forms from '@tailwindcss/forms';
// @ts-expect-error EsModuleInterop is working
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {}
	},

	plugins: [typography, forms]
} satisfies Config;
