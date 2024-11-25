<script lang="ts">
	import { slide } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import LineMdLoadingLoop from '~icons/line-md/loading-loop';

	let transitionIn = { easing: cubicOut, duration: 300 };
	let transitionOut = { easing: cubicIn, duration: 300 };
</script>

<div class="flex justify-center" in:slide={transitionIn} out:slide={transitionOut}>
	<LineMdLoadingLoop class="h-10 w-10" />
</div>
