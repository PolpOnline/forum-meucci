<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import LogosGoogleIcon from '~icons/logos/google-icon';
	import { API_URL } from '$lib/api/api';
</script>

<main class="h-screen-not-navbar flex w-full items-center justify-center">
	<Button data-umami-event="Login" href="{API_URL}/auth/login" variant="outline">
		<LogosGoogleIcon class="h-6 w-6" />
		Login with Google
	</Button>
</main>
