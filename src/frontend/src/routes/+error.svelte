<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import { page } from '$app/state';
</script>

<div class="h-screen-not-navbar flex w-full flex-col items-center justify-center">
	<h1 class="text-bold text-9xl">{page.status}</h1>
	<h2 class="text-light text-muted mt-4 text-5xl">{page.error?.message}</h2>

	<Button class={cn('text-primary mt-8', buttonVariants({ variant: 'outline' }))} href="/">
		Go back home
	</Button>
</div>
