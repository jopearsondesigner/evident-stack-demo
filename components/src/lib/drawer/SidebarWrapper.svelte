<script lang="ts">
	import classNames from 'classnames';
	import DropdownMenu from '$lib/dropdown/DropdownMenu.svelte';
	import DropdownItem from '$lib/dropdown/DropdownItem.svelte';
	export let brandClass =
		'py-6 px-1 bg-white hover:bg-focus/[.04] transition duration-200 ease-in flex justify-center w-full cursor-default';
	export let ulClass =
		'-mt-5 absolute whitespace-nowrap z-40 transform w-auto rounded-lg shadow-xl border border-light dark:border-border-dark overflow-hidden bg-white dark:bg-dark-2 px-1 py-2 cursor-default';
	let liClass =
		'font-extrabold text-center py-2 px-4 text-sm hover:bg-focus/[.2] dark:hover:bg-focus/[.2] cursor-default';
	export let productName = 'Evident Design';
	export let alt = 'Evident Design logo';
	export { brandClass as class };
	export let logoClass = 'inline-flex';
	export let height = 28;
	import Icon from '$lib/Icon.svelte';
	import ArrowDown from '$lib/icons/ArrowDown.svelte';
	import Logo from '$lib/assets/images/product/design/evidentDesignLogo.svg';
	export let design = true;
	export let data = false;
	export let domainFunctions = false;
	export let deploy = false;
	export let database = false;
	let hidden = true;
	let block = false;
	let visible = false;

	const handleDropdown = () => {
		hidden = !hidden;
		block = !block;
		visible = !visible;
	};
	let product = true;

	let container: { contains: (arg0: any) => boolean };
	function onWindowClick(e: { target: any }) {
		if (container.contains(e.target) == false) active = false;
	}

	let active = true;
	const layerActive = () => {
		active = !active;
	};

	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	function handleClick(event: any) {
		dispatch('click', event);
	}
</script>

<svelte:window on:click={onWindowClick} />

<button class={brandClass} on:click={() => handleDropdown()}>
	<span class="sr-only">{productName}</span>
	<div class={logoClass}>
		<img src={Logo} {alt} {height} style="height:{height}px" />
		<Icon name="arrow-down" size={15} class="mt-[13px] ml-1" iconColor="text-body-light"
			><ArrowDown /></Icon
		>
	</div>
</button>
{#if !hidden}
	<DropdownMenu name="Layer" dropdownDiv={ulClass} bind:product>
		{#if active}
			<DropdownItem
				class={liClass}
				btnClass="text-default"
				on:click={() => layerActive()}
				on:click={handleClick}
				bind:active>Evident Design</DropdownItem
			>
		{:else}
			<DropdownItem
				class={liClass}
				btnClass="text-default"
				on:click={() => layerActive()}
				on:click={handleClick}>Evident Design</DropdownItem
			>
		{/if}
		<DropdownItem class={liClass} btnClass="text-default">Evident Data</DropdownItem>
		<DropdownItem class={liClass} btnClass="text-default">Evident Domain Functions</DropdownItem>
		<DropdownItem class={liClass} btnClass="text-default">Evident Deploy</DropdownItem>
		<DropdownItem class={liClass} btnClass="text-default">Evident Database</DropdownItem>
	</DropdownMenu>
{/if}
<div>
	{#if design}
		<slot name="design" />
	{:else if data}
		<slot name="data" />
	{:else if domainFunctions}
		<slot name="domain-functions" />
	{:else if deploy}
		<slot name="deploy" />
	{:else if database}
		<slot name="database" />
	{/if}
</div>
