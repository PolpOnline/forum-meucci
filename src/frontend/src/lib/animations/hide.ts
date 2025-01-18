import type { TransitionConfig } from 'svelte/transition';

export function hide(
	node: Element,
	{ duration, delay }: { duration: number; delay: number }
): TransitionConfig {
	// noinspection JSUnusedLocalSymbols
	return {
		duration: duration + delay,
		// eslint-disable-next-line @typescript-eslint/no-unused-vars
		css: (t) => `opacity: 0`
	};
}
