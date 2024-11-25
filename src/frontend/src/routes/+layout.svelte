<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import Navbar from '$lib/components/Navbar.svelte';
	import type { LayoutData } from './$types';
	import type { Snippet } from 'svelte';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import { afterNavigate, beforeNavigate } from '$app/navigation';
	import { fly } from 'svelte/transition';
	import Loader from '$lib/components/Loader.svelte';

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

<Navbar loginStatus={data.loginStatus} />
<ModeWatcher defaultMode="dark" />

{#if isLoading}
	<Loader />
{/if}
{#key data.pathname}
	<div in:fly={transitionIn} out:fly={transitionOut}>
		{@render children()}
	</div>
{/key}
