<script>
	import './drawer.css';
	import { fly } from 'svelte/transition';
	import { sineIn } from 'svelte/easing';
	export let hidden = true;
	let transitionParams = {
		x: -240,
		duration: 200,
		easing: sineIn
	};
</script>

{#if !hidden}
	<div transition:fly={transitionParams}>
		<slot />
	</div>
{/if}
