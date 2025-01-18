<script lang="ts">
	import '../app.css';
	import 'unfonts.css';
	import Navbar from '$lib/components/Navbar.svelte';
	import type { LayoutData } from './$types';
	import type { Snippet } from 'svelte';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import { fly } from 'svelte/transition';
	import Loader from '$lib/components/Loader.svelte';
	import { title } from '$lib/stores/title.store';
	import Favicon from '$lib/images/favicon.svg';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { links } from 'unplugin-fonts/head';
	import ActivityFullDialog from '$lib/components/dialogs/ActivityFullDialog.svelte';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();

	// Page transition
	const duration = 300;
	const delay = duration + 100;
	const y = 10;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };

	let isLoading = $state(false);

	// Show loader only when navigating between internal pages
	beforeNavigate(({ to }) => (isLoading = !!to?.route.id));
	afterNavigate(() => (isLoading = false));
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
	<Navbar loginStatus={data.loginStatus} />

	{#if isLoading}
		<Loader />
	{/if}
	{#key data.pathname}
		<div in:fly={transitionIn} out:fly={transitionOut}>
			{@render children()}
		</div>
	{/key}
</div>
