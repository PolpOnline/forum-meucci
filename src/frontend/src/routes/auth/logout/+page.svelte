<script lang="ts">
	import { onMount } from 'svelte';
	import { trackEvent } from '@lukulent/svelte-umami';

	onMount(() => {
		trackEvent('Logout');
	});
</script>

<div class="h-screen-not-navbar flex w-full flex-col items-center justify-center">
	<h1 class="text-[8rem]">Logging you out...</h1>
</div>
