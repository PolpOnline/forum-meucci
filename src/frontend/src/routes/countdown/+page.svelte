<script lang="ts">
	import type { PageData } from './$types';
	import Countdown from './Countdown.svelte';

	let { data }: { data: PageData } = $props();
</script>

<div class="h-screen-not-navbar flex flex-col items-center justify-center space-y-10 pb-20">
	<span class="text-center text-2xl font-bold"> Le iscrizioni apriranno tra: </span>

	<Countdown timeUntil={new Date(data.startDate)} />
</div>
