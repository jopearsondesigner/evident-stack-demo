<script lang="ts">
	import { onMount } from 'svelte';

	let LottiePlayer;

	onMount(async () => {
		const module = await import('@lottiefiles/svelte-lottie-player');
		LottiePlayer = module.LottiePlayer;
	});

	let controlsLayout = [
		'previousFrame',
		'playpause',
		'stop',
		'nextFrame',
		'progress',
		'frame',
		'loop',
		'spacer',
		'background',
		'snapshot',
		'zoom',
		'info',
		'controls'
	];

	export let src: string;
</script>

{#if LottiePlayer}
	<svelte:component
		this={LottiePlayer}
		{src}
		autoplay={true}
		loop={true}
		controls={false}
		renderer="svg"
		background="transparent"
		height="auto"
		width="auto"
		{controlsLayout}
	/>
{/if}
