<script>
	import classNames from 'classnames';
	export let brandClass =
		'py-6 px-1 bg-white hover:bg-focus/[.04] transition duration-200 ease-in flex justify-center w-full cursor-default';
	export let product = 'Evident Design';
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
</script>

<button class={brandClass}>
	<span class="sr-only">{product}</span>
	<div class={logoClass}>
		<img src={Logo} {alt} {height} style="height:{height}px" />
		<Icon name="arrow-down" size={15} class="mt-[13px] ml-1" iconColor="text-body-light"
			><ArrowDown /></Icon
		>
	</div>
</button>
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
