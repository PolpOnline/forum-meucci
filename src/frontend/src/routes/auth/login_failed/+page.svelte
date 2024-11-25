<script lang="ts">
	import { Button } from '$lib/components/ui/button';

	const { data } = $props();
</script>

<div class="h-screen-not-navbar flex w-full flex-col items-center justify-center">
	<h1 class="text-6xl">Errore nel login</h1>
	<h3 class="mt-6 text-xl text-zinc-500">{data.message}</h3>

	<Button class="mt-16" variant="outline" href="/auth/login">Ritorna alla pagina di login</Button>
</div>
