<script lang="ts">
	import LucideLogOut from '~icons/lucide/log-out';
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideSettings from '~icons/lucide/settings';
	import LucideGithub from '~icons/lucide/github';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import DropdownMenuLinkItem from '$lib/components/DropdownMenuLinkItem.svelte';
	import type { LoginStatus } from '../../app';
	import ItisMeucciLogo from '$lib/images/ItisMeucciLogo.svelte';

	const { loginStatus }: { loginStatus: LoginStatus } = $props();

	const loggedIn = $derived(loginStatus === 'logged_in');
</script>

<nav class="grid h-20 grid-cols-12">
	<div class="grid-span-2 col-span-2 flex items-center">
		<a href="/">
			<ItisMeucciLogo class="ml-3 h-20 w-20 max-w-full" />
		</a>
	</div>
	<div class="col-span-8 flex items-center justify-center font-black">
		<a href="/" class="text-3xl"> Forum Meucci </a>
	</div>
	<div class="col-span-2 mr-3 flex items-center gap-1 justify-self-end">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger class={buttonVariants({ size: 'icon' })} aria-label="Navbar Menu">
				<LucideSettings />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content>
				{#if loggedIn}
					<DropdownMenu.Group>
						<div data-sveltekit-preload-data="off">
							<DropdownMenuLinkItem class="text-red-600" href="/auth/logout">
								<LucideLogOut class="mr-2 h-4 w-4" />
								Logout
							</DropdownMenuLinkItem>
						</div>
					</DropdownMenu.Group>
					<DropdownMenu.Separator />
				{/if}
				<DropdownMenuLinkItem href="https://github.com/PolpOnline/forum-meucci" target="_blank">
					<LucideGithub class="mr-2 h-4 w-4" />
					GitHub
				</DropdownMenuLinkItem>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</nav>
