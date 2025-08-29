<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { fromEmailEstimateName } from '$lib/utils/text';

	let { data } = $props();

	let user = fromEmailEstimateName(data.loggedInEmail!);
</script>

<h1 class="mt-3 text-center text-3xl font-bold">Welcome {user}</h1>

<Button class="mx-auto mt-20 font-bold" href="/forum" variant="outline">Forum Meucci</Button>
