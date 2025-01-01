import { writable } from 'svelte/store';
import { dev } from '$app/environment';

function createTitle() {
	const { subscribe, set } = writable(getTitle());

	return {
		subscribe,
		set: (value: string) => {
			set(getTitle(value));
		}
	};
}

function getTitle(value: string = 'Forum Meucci') {
	return `${dev ? '(DEV) ' : ''}${value}`;
}

export const title = createTitle();
