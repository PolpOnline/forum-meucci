import 'unplugin-icons/types/svelte';

// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			loginStatus: LoginStatus;
		}

		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};

export type LoginStatus = 'logged_in' | 'logged_out';
