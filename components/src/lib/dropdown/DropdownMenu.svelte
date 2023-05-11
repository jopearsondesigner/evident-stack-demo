<script lang="ts">
	import classNames from 'classnames';
	import { fade } from 'svelte/transition';
	import type { NavbarType } from '../types';
	import { clickOutside } from '../utils/clickOutside';
	export let activateClickOutside = true;
	import { createEventDispatcher } from 'svelte';

	import Icon from '../Icon.svelte';
	import NavArrowDark from '../icons/NavArrowDark.svelte';
	import NavArrowLight from '../icons/NavArrowLight.svelte';
	import ArrowDown from '../icons/ArrowDown.svelte';
	export let liButtonClass: string =
		'whitespace-nowrap inline-flex justify-center items-center py-4 text-sm text-body-light dark:text-body-dark hover:text-body-dark dark:hover:text-body-light focus:outline-none transition duration-300 ease-in-out';
	export let name: string;
	export const child: NavbarType[] = [];
	export let dropdownClass: string =
		'absolute whitespace-nowrap z-40 transform w-auto rounded-lg border border-light dark:border-border-dark overflow-hidden bg-white dark:bg-dark-2';
	export let divClass: string = '-mt-1 shadow-xl px-4 py-4 sm:gap-8 sm:p-8';
	export let ulClass: string = 'shadow py-3';
	export let marginTop: string = '';
	export const dropdownLinkClassWithChild: string | undefined = undefined;
	export const rel: string | undefined = undefined;

	export let hidden = true;
	let block = false;
	let visible = false;
	export let website = false;
	export let product = false;

	const handleDropdown = () => {
		hidden = !hidden;
		visible = !visible;
	};

	let liClass = 'list-none flex justify-center';
</script>

{#if website && activateClickOutside}
	<li use:clickOutside={() => !hidden && handleDropdown()} class={liClass}>
		<button on:click={() => handleDropdown()} class={liButtonClass}
			>{name}
			<Icon
				name="arrow-down"
				class="ml-1 stroke-2"
				iconColor="fill-current"
				size={12}
				pathName={ArrowDown}
			/>
			{#if visible}
				<Icon
					name="nav-arrow-light"
					class="block dark:hidden z-50 w-6 absolute mt-[37px] transform"
					pathName={NavArrowLight}
				/>
				<Icon
					name="nav-arrow-dark"
					class="hidden dark:block z-50 w-6 absolute mt-[37px] transform"
					pathName={NavArrowDark}
				/>
			{/if}
		</button>

		<!-- Dropdown menu -->
		{#if !hidden}
			<div class:hidden class={classNames(dropdownClass, divClass, marginTop)}>
				<slot />
			</div>
		{/if}
	</li>
{:else if product && activateClickOutside}
	<li use:clickOutside={() => !hidden && handleDropdown()} class={liClass}>
		<button on:click={() => handleDropdown()}>
			<slot name="button" />
		</button>
		{#if !hidden}
			<!-- Dropdown menu -->
			<ul
				class={classNames('flex justify-center', dropdownClass, ulClass, marginTop)}
				transition:fade={{ duration: 100 }}
			>
				<li class="flex flex-col">
					<slot />
				</li>
			</ul>
		{/if}
	</li>
{/if}
