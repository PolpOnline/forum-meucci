<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="h-screen-not-navbar flex w-full flex-col items-center justify-center">
	<h1 class="text-bold text-9xl">{$page.status}</h1>
	<h2 class="text-light mt-4 text-5xl text-muted">{$page.error?.message}</h2>
</div>
