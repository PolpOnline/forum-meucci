<script lang="ts">
	import LucideMoon from '~icons/lucide/moon';
	import LucideSun from '~icons/lucide/sun';
	import LucideLogOut from '~icons/lucide/log-out';
	// noinspection ES6UnusedImports
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import LucideSettings from '~icons/lucide/settings';
	import { mode, toggleMode } from 'mode-watcher';
	import LucideGithub from '~icons/lucide/github';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import DropdownMenuLinkItem from '$lib/components/DropdownMenuLinkItem.svelte';
	import type { LoginStatus } from '../../app';
	import ItisMeucciLogo from '$lib/images/ItisMeucciLogo.svelte';

	const { loginStatus }: { loginStatus: LoginStatus } = $props();

	const loggedIn = $derived(loginStatus === 'logged_in');
</script>

<nav class="grid h-20 grid-cols-3">
	<div class="flex items-center">
		<ItisMeucciLogo class="ml-3 h-20 w-20" />
	</div>
	<div class="flex items-center justify-center">
		<a href="/" class="text-3xl"> Forum Meucci </a>
	</div>
	<div class="mr-3 flex items-center gap-1 justify-self-end">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger class={buttonVariants({ size: 'icon' })} aria-label="Navbar Menu">
				<LucideSettings />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content>
				<DropdownMenu.Group>
					<DropdownMenu.Item onclick={toggleMode}>
						{#if $mode === 'dark'}
							<LucideSun class="mr-2 h-4 w-4" />
						{:else}
							<LucideMoon class="mr-2 h-4 w-4" />
						{/if}
						<span> Cambia tema </span>
					</DropdownMenu.Item>
					{#if loggedIn}
						<div data-sveltekit-preload-data="off">
							<DropdownMenuLinkItem class="text-red-600" href="/logout">
								<LucideLogOut class="mr-2 h-4 w-4" />
								Logout
							</DropdownMenuLinkItem>
						</div>
					{/if}
				</DropdownMenu.Group>
				<DropdownMenu.Separator />
				<DropdownMenuLinkItem href="https://github.com/PolpOnline/forum-meucci" target="_blank">
					<LucideGithub class="mr-2 h-4 w-4" />
					Vedi su GitHub
				</DropdownMenuLinkItem>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</nav>
