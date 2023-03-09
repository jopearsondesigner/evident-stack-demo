<script>
	import classnames from 'classnames';
	import { fly } from 'svelte/transition';
	import { sineOut } from 'svelte/easing';
	export let hidden = true;
	export let divClass = 'z-40';
	let transitionParams = {
		x: -240,
		duration: 200,
		easing: sineOut
	};
</script>

{#if !hidden}
	<div transition:fly={transitionParams} class={classnames('absolute', divClass)}>
		<slot />
	</div>
{/if}
