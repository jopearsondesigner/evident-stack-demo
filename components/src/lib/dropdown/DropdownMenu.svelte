<script lang="ts">
	import type { NavbarType } from '../types';
	import { clickOutside } from '../utils/clickOutside';
	import Icon from '../Icon.svelte';
	import NavArrowDark from '../icons/NavArrowDark.svelte';
	import NavArrowLight from '../icons/NavArrowLight.svelte';
	import ArrowDown from '../icons/ArrowDown.svelte';
	export let liButtonClass: string =
		'whitespace-nowrap inline-flex justify-center items-center py-4 text-sm text-body-light dark:text-body-dark hover:text-body-dark dark:hover:text-body-light focus:outline-none transition duration-300 ease-in-out';
	export let name: string;
	export const child: NavbarType[] = [];
	export let dropdownDiv: string =
		'absolute whitespace-nowrap z-40 -ml-4 -mt-1 transform w-auto rounded-lg shadow-xl border border-light dark:border-border-dark overflow-hidden bg-white dark:bg-dark-2 px-4 py-4 sm:gap-8 sm:p-8';
	export const dropdownLinkClassWithChild: string | undefined = undefined;
	export const rel: string | undefined = undefined;

	let hidden = true;
	let block = false;
	let visible = false;
	export let website = false;
	export let product = false;

	const handleDropdown = () => {
		hidden = !hidden;
		block = !block;
		visible = !visible;
	};

	let liClass = '';
</script>

{#if website}
	<li use:clickOutside={() => !hidden && handleDropdown()} class={liClass}>
		<button on:click={() => handleDropdown()} class={liButtonClass}
			>{name}
			<Icon name="arrow-down" class="ml-1 stroke-2" iconColor="fill-current" size={12}
				><ArrowDown /></Icon
			>
			{#if visible}
				<Icon name="nav-arrow-light" class="block dark:hidden z-50 w-6 absolute mt-[37px] transform"
					><NavArrowLight /></Icon
				>
				<Icon name="nav-arrow-dark" class="hidden dark:block z-50 w-6 absolute mt-[37px] transform"
					><NavArrowDark /></Icon
				>
			{/if}
		</button>

		<!-- Dropdown menu -->
		<div class:hidden class={dropdownDiv}>
			<slot />
		</div>
	</li>
{:else if product}
	<div class="w-full flex justify-center" use:clickOutside={() => !hidden && handleDropdown()}>
		<ul class:block class={dropdownDiv}>
			<slot />
		</ul>
	</div>
{/if}
