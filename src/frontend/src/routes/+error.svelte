<script lang="ts">
	import { page } from '$app/stores';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
</script>

<div class="h-screen-not-navbar flex w-full flex-col items-center justify-center">
	<h1 class="text-bold text-9xl">{$page.status}</h1>
	<h2 class="text-light mt-4 text-5xl text-muted">{$page.error?.message}</h2>

	<Button class={cn('mt-8 text-primary', buttonVariants({ variant: 'outline' }))} href="/">
		Go back home
	</Button>
</div>
