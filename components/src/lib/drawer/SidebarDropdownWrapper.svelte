<script type="ts">
	import classNames from 'classnames';
	import { slide } from 'svelte/transition';
	import { sineIn } from 'svelte/easing';
	import { slideRight } from 'svelte-layout-aware-transitions';
	import Icon from '../Icon.svelte';
	import OpenRight from '../icons/OpenRight.svelte';
	import OpenDown from '../icons/OpenDown.svelte';
	import CloseLeft from '../icons/CloseLeft.svelte';
	import CloseUp from '../icons/CloseUp.svelte';
	export let horizontalLiCLass = 'relative inline-flex';
	export let horizontalULClass =
		'sidebarDropdownItem w-60 border-t border-r border-b border-gray-primary dark:border-gray-brand-1';
	export let horizontalWrapperClass = 'left-full top-0 bottom-auto absolute';
	export let headerTextClass = 'font-extrabold text-default text-body dark:text-white';
	export let sidebarItemClass = 'flex items-center pl-4 pr-1 transition duration-200 ease-in';
	export let sidebarBtnClass =
		'w-full bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] focus:text-body focus:bg-focus/[.20] dark:focus:bg-focus/[.20] transition duration-200 ease-in space-x-3 h-8 pointer-default';
	export let sidebarSpanClass = 'flex-1 ml-3 text-left whitespace-nowrap';
	export let label = '';
	export let horizontal = false;
	export let isHorizontalOpen = false;
	let container: HTMLButtonElement;
	function onWindowClick(e: { target: any }) {
		if (container.contains(e.target) == false) isHorizontalOpen = false;
	}
	export let isVerticalOpen = false;
	const handleHorizontalDropdown = () => {
		isHorizontalOpen = !isHorizontalOpen;
	};
	const handleVerticalDropdown = () => {
		isVerticalOpen = !isVerticalOpen;
	};
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();
	function handleClick(event: any) {
		dispatch('click', event);
	}
</script>

<svelte:window on:click={onWindowClick} />

{#if horizontal}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<li class={classNames('sidebarDropdownWrapper', horizontalLiCLass)}>
		<button
			bind:this={container}
			class={classNames(sidebarItemClass, sidebarBtnClass)}
			on:click={() => handleHorizontalDropdown()}
		>
			<slot name="icon" />
			<span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
			{#if isHorizontalOpen}
				<Icon name="open-right" size={16} iconColor="text-brand-4 dark:text-white/[.31]"
					><OpenRight /></Icon
				>
			{:else}
				<Icon name="close-left" size={16} iconColor="text-brand-4 dark:text-white/[.31]"
					><CloseLeft /></Icon
				>
			{/if}
		</button>
		<div class={classNames(horizontalWrapperClass)}>
			{#if isHorizontalOpen}
				<ul
					class={classNames(horizontalULClass)}
					transition:slideRight={{ x: 240, duration: 200, easing: sineIn }}
				>
					<slot />
				</ul>
			{/if}
		</div>
	</li>
{:else}
	<li class={classNames('sidebarDropdownWrapper')}>
		<button
			class={classNames(sidebarItemClass, sidebarBtnClass)}
			on:click={() => handleVerticalDropdown()}
			on:click={handleClick}
		>
			<slot name="icon" />
			<span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
			{#if isVerticalOpen}
				<Icon name="close-down" size={16} iconColor="text-brand-4 dark:text-white/[.31]"
					><CloseUp /></Icon
				>
			{:else}
				<Icon name="open-down" size={16} iconColor="text-brand-4 dark:text-white/[.31]"
					><OpenDown /></Icon
				>
			{/if}
		</button>
		{#if isVerticalOpen}
			<ul
				class="sidebarDropdownItem"
				transition:slide={{ delay: 0, duration: 200, easing: sineIn }}
			>
				<slot />
			</ul>
		{/if}
	</li>
{/if}
