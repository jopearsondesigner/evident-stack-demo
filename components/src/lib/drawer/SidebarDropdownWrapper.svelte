<script>
	import classNames from 'classnames';
	import './drawer.css';
	import { slide } from 'svelte/transition';
	import { sineIn } from 'svelte/easing';
	import { slideRight } from 'svelte-layout-aware-transitions';
	import Icon from '../Icon.svelte';
	import OpenRight from '../icons/OpenRight.svelte';
	import OpenDown from '../icons/OpenDown.svelte';
	import CloseLeft from '../icons/CloseLeft.svelte';
	import CloseUp from '../icons/CloseUp.svelte';
	export let verticalLiCLass='w-full relative inline-flex';
	export let verticalULClass='sidebarDropdownItem w-60 border-t border-r border-b border-gray-primary';
	export let verticalWrapperClass='left-full top-0 bottom-auto absolute';
	export let headerTextClass='font-extrabold text-default text-body';
	export let sidebarItemClass='flex items-center pl-4 pr-1 transition duration-200 ease-in';
	export let sidebarBtnClass='bg-white hover:bg-active/[.20] focus:text-white focus:bg-active/[100] transition duration-200 ease-in w-full space-x-3 h-8 pointer-default';
	export let sidebarSpanClass = 'flex-1 ml-3 text-left whitespace-nowrap';
	export let label = '';
	export let horizontal=false;
	export let isHorizontalOpen=false;
	/**
	 * @type {{ contains: (arg0: any) => boolean; }}
	 */
	let container;
	/**
	 * @param {{ target: any; }} e
	 */
	function onWindowClick(e) {
		if (container.contains(e.target) == false) isHorizontalOpen = false;
	}
	export let isVerticalOpen = false;
	const handleHorizontalDropdown = () => {
		isHorizontalOpen = !isHorizontalOpen;
	};
	const handleVerticalDropdown = () => {
		isVerticalOpen = !isVerticalOpen;
	};
</script>

<svelte:window on:click={onWindowClick} />

{#if horizontal}
	<li class={classNames('sidebarDropdownWrapper', verticalLiCLass)}>
		<button
			bind:this={container}
			class={classNames(sidebarItemClass, sidebarBtnClass)}
			on:click={() => handleHorizontalDropdown()}
		>
			<slot name="icon" />
			<span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
			{#if isHorizontalOpen}
				<Icon name="open-right" size={20} iconColor="text-brand-4"><OpenRight /></Icon>
			{:else}
				<Icon name="close-left" size={20} iconColor="text-brand-4"><CloseLeft /></Icon>
			{/if}
		</button>
		<div class={classNames(verticalWrapperClass)}>
			{#if isHorizontalOpen}
				<ul
					class={classNames(verticalULClass)}
					transition:slideRight={{ x: 240, duration: 200, easing: sineIn }}
				>
					<slot />
				</ul>
			{/if}
		</div>
	</li>
{:else}
	<li class="sidebarDropdownWrapper">
		<button
			class={classNames(sidebarItemClass, sidebarBtnClass)}
			on:click={() => handleVerticalDropdown()}
		>
			<span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
			{#if isVerticalOpen}
				<Icon name="close-down" size={20} iconColor="text-brand-4"><CloseUp /></Icon>
			{:else}
				<Icon name="open-down" size={20} iconColor="text-brand-4"><OpenDown /></Icon>
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
