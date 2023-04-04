<script>
	import classNames from 'classnames';
	import CloseButton from '$lib/utils/CloseButton.svelte';
	import { fly } from 'svelte/transition';
	import { sineOut } from 'svelte/easing';
	import { clickOutside } from '$lib/utils/clickOutside';
	export let activateClickOutside = true;
	export let hidden = true;
	export let divClass = 'z-40 fixed h-auto bg-white dark:bg-dark-2';
	export let className = '';
	export let leftOffset = 'left-0';
	export let rightOffset = 'right-0';
	export let placement = 'left';
	export let navbarHeight = 64;
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

	export let backdropClasses = 'bg-gray-900 bg-opacity-50 dark:bg-opacity-80';
</script>

{#if !hidden}
	{#if !drawerRight}
		<div
			transition:fly={transitionParams}
			class={classNames(className, divClass)}
			style="padding-top: {navbarHeight}px;"
		>
			<slot />
		</div>
	{:else if activateClickOutside}
		<div class={classNames('fixed inset-0 z-30', backdropClasses)} />
		<div
			use:clickOutside={() => !hidden && handleDrawer()}
			transition:fly={transitionParamsRight}
			class={classNames(className, divClass, placements[placement])}
			style="padding-top: {navbarHeight}px;"
		>
			<CloseButton
				name="Close Placement Details"
				size={12}
				btnClass="absolute right-0 mt-3 mr-3"
				on:click={() => (hidden = true)}
				color={$$restProps.color}
				style="margin-top: {navbarHeight}px;"
			/>
			<slot />
		</div>
	{/if}
{/if}
