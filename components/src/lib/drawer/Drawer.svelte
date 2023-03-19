<script>
	import classnames from 'classnames';
	import { fly } from 'svelte/transition';
	import { sineOut } from 'svelte/easing';
	// import { clickOutside } from '$lib/utils/clickOutside';
	export let activateClickOutside = true;
	export let hidden = true;
	export let divClass = 'z-40';
	export let leftOffset = 'left-0';
	export let rightOffset = 'right-0';
	export let placement = 'left';
	let transitionParams = {
		x: -240,
		duration: 200,
		easing: sineOut
	};
	let transitionParamsRight = {
		x: 480,
		duration: 200,
		easing: sineOut
	};
	export let drawerRight = false;

	const placements = {
		left: leftOffset,
		right: rightOffset
	};
	const handleDrawer = () => {
		hidden = !hidden;
	};
</script>

{#if !hidden}
	{#if !drawerRight}
		<div transition:fly={transitionParams} class={classnames('absolute', divClass)}>
			<slot />
		</div>
	{:else if activateClickOutside}
		<div
			transition:fly={transitionParamsRight}
			class={classnames('absolute', divClass, placements[placement])}
		>
			<slot />
		</div>
	{/if}
{/if}
