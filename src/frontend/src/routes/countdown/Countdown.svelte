<script lang="ts">
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { onDestroy } from 'svelte';

	const { timeUntil }: { timeUntil: Date } = $props();

	type TimeUntilComponents = {
		days: number;
		hours: number;
		minutes: number;
		seconds: number;
	};

	let reactiveTime = $state(setTime());
	const interval = setInterval(() => {
		reactiveTime = setTime();
	}, 1000);

	function setTime(): TimeUntilComponents {
		let now = new Date().getTime();
		let diff = timeUntil.getTime() - now;

		if (diff <= 0) {
			clearInterval(interval);
			diff = 0;

			if (browser) {
				goto('/');
			}
		}

		let days = Math.floor(diff / (1000 * 60 * 60 * 24));
		let hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
		let minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
		let seconds = Math.floor((diff % (1000 * 60)) / 1000);

		if (days > 99) {
			return { days: 99, hours: 99, minutes: 99, seconds: 99 };
		}

		return { days, hours, minutes, seconds };
	}

	onDestroy(() => {
		clearInterval(interval);
	});
</script>

<div class="flex w-max flex-col flex-wrap rounded-2xl text-center">
	<div class="grid auto-cols-max grid-flow-col gap-5 text-center">
		<div class="flex flex-col">
			<span class="countdown text-4xl">
				<span style="--value: {reactiveTime.days}"></span>
			</span>
			giorni
		</div>
		<div class="flex flex-col">
			<span class="countdown text-4xl">
				<span style="--value: {reactiveTime.hours}"></span>
			</span>
			ore
		</div>
		<div class="flex flex-col">
			<span class="countdown text-4xl">
				<span style="--value: {reactiveTime.minutes}"></span>
			</span>
			min
		</div>
		<div class="flex flex-col">
			<span class="countdown text-4xl">
				<span style="--value: {reactiveTime.seconds}"></span>
			</span>
			sec
		</div>
	</div>
</div>

<style>
	:root .countdown {
		line-height: 1em;
	}

	.countdown {
		display: inline-flex;
	}

	.countdown > * {
		height: 1em;
		display: inline-block;
		overflow-y: hidden;
	}

	.countdown > *:before {
		position: relative;
		content: '00\A 01\A 02\A 03\A 04\A 05\A 06\A 07\A 08\A 09\A 10\A 11\A 12\A 13\A 14\A 15\A 16\A 17\A 18\A 19\A 20\A 21\A 22\A 23\A 24\A 25\A 26\A 27\A 28\A 29\A 30\A 31\A 32\A 33\A 34\A 35\A 36\A 37\A 38\A 39\A 40\A 41\A 42\A 43\A 44\A 45\A 46\A 47\A 48\A 49\A 50\A 51\A 52\A 53\A 54\A 55\A 56\A 57\A 58\A 59\A 60\A 61\A 62\A 63\A 64\A 65\A 66\A 67\A 68\A 69\A 70\A 71\A 72\A 73\A 74\A 75\A 76\A 77\A 78\A 79\A 80\A 81\A 82\A 83\A 84\A 85\A 86\A 87\A 88\A 89\A 90\A 91\A 92\A 93\A 94\A 95\A 96\A 97\A 98\A 99\A';
		white-space: pre;
		top: calc(var(--value) * -1em);
		text-align: center;
		transition: all 1s cubic-bezier(1, 0, 0, 1);
	}
</style>
