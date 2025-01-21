<script lang="ts">
	import '../app.css';
	import 'unfonts.css';
	import Navbar from '$lib/components/Navbar.svelte';
	import type { LayoutData } from './$types';
	import type { Snippet } from 'svelte';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import { fly } from 'svelte/transition';
	import { title } from '$lib/stores/title.store';
	import Favicon from '$lib/images/favicon.svg';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { links } from 'unplugin-fonts/head';
	import ActivityFullDialog from '$lib/components/dialogs/ActivityFullDialog.svelte';
	import { ProgressBar } from '@prgm/sveltekit-progress-bar';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();

	// Page transition
	const duration = 300;
	const delay = duration + 100;
	const y = 10;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };
</script>

<svelte:head>
	<title>{$title}</title>
	<link rel="icon" type="image/svg+xml" href={Favicon} />
	<!-- for Safari -->
	<link rel="mask-icon" href={Favicon} color="#000000" />
	{#each links as link}
		<link {...link?.attrs || {}} />
	{/each}
</svelte:head>

<Toaster position="top-center" richColors theme="dark" />

<ActivityFullDialog />

<div data-vaul-drawer-wrapper>
	<ProgressBar class="text-white" zIndex={100} />

	<Navbar loginStatus={data.loginStatus} />

	{#key data.pathname}
		<div in:fly={transitionIn} out:fly={transitionOut}>
			{@render children()}
		</div>
	{/key}
</div>
