<script lang="ts">
	import type { NavbarType } from '../types';
	import Icon from '../Icon.svelte';
	import ArrowDown from '../icons/ArrowDown.svelte';
	import ArrowUp from '../icons/ArrowUp.svelte';
	export let liButtonClass: string =
		'w-full bg-transparent focus:bg-focus text-body-light dark:text-dark focus:text-white focus:dark:text-white whitespace-nowrap flex justify-between items-center p-4 text-sm focus:text-body-dark focus:outline-none transition duration-300 ease-in-out';
	export let name: string;
	export const child: NavbarType[] = [];
	export let dropdownDiv: string =
		'z-[100] whitespace-nowrap w-auto border-b border-light dark:border-border-dark overflow-hidden bg-white dark:bg-dark-2 px-3';
	export const dropdownLinkClassWithChild: string | undefined = undefined;
	export const rel: string | undefined = undefined;

	let hidden = true;
	let block = false;
	export let website = true;

	const handleDropdown = () => {
		hidden = !hidden;
		block = !block;
		website = !website;
	};

	let liClass = '';
</script>

<li class={liClass}>
	<button on:click={() => handleDropdown()} class={liButtonClass}
		>{name}
		{#if !hidden}
			<Icon name="arrow-down" class="ml-1 stroke-2" iconColor="fill-current" size={16}
				><ArrowDown /></Icon
			>
		{:else}
			<Icon name="arrow-up" class="ml-1 stroke-2" iconColor="fill-current" size={16}
				><ArrowUp /></Icon
			>
		{/if}
	</button>

	<!-- Dropdown menu -->
	<div class:hidden class={dropdownDiv}>
		<slot />
	</div>
</li>
