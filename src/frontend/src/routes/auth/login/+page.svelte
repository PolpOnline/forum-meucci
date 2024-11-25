<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import LogosGoogleIcon from '~icons/logos/google-icon';
	import { API_URL } from '$lib/api/api';
</script>

<div class="h-screen-not-navbar flex w-full items-center justify-center">
	<Button variant="outline" href={`${API_URL}/auth/login`}>
		<LogosGoogleIcon class="h-6 w-6" />
		Login with Google
	</Button>
</div>
