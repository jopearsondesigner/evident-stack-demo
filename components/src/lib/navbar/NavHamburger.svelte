<script lang="ts">
	import Menu from '../icons/Menu.svelte';
	import { createEventDispatcher } from 'svelte';
	export let hamburgerClass = 'cursor-default';
	export let hamburgerWebClass =
		'mr-4 lg:hidden bg-none p-2 inline-flex items-center justify-center text-body-1 dark:text-body-4 no-underline';
	import Icon from '../Icon.svelte';
	import CloseSidebar from '../icons/CloseSidebar.svelte';
	import OpenSidebar from '../icons/OpenSidebar.svelte';
	import CloseSidebarDark from '../icons/CloseSidebarDark.svelte';
	import OpenSidebarDark from '../icons/OpenSidebarDark.svelte';
	const dispatch = createEventDispatcher();
	function handleClick(event: any) {
		dispatch('click', event);
	}
	let hidden: boolean = false;
	let toggle = () => {
		hidden = !hidden;
	};
	export let website: boolean;
</script>

{#if !website}
	<button class={hamburgerClass} on:click={() => toggle()} on:click={handleClick}>
		{#if !hidden}
			<Icon
				name="open-sidebar"
				size={32}
				viewBox="0 0 28 28"
				iconColor="text-gray-brand-1"
				class="block dark:hidden"><OpenSidebar /></Icon
			>
			<Icon
				name="open-sidebar-dark"
				size={32}
				viewBox="0 0 28 28"
				iconColor="text-gray-primary"
				class="hidden dark:block"><OpenSidebarDark /></Icon
			>
		{:else}
			<Icon
				name="close-sidebar"
				size={32}
				viewBox="0 0 28 28"
				iconColor="text-gray-brand-1"
				class="block dark:hidden"><CloseSidebar /></Icon
			>
			<Icon
				name="close-sidebar"
				size={32}
				viewBox="0 0 28 28"
				iconColor="text-gray-primary"
				class="hidden dark:block"><CloseSidebarDark /></Icon
			>
		{/if}
	</button>
{:else}
	<button type="button" hidden class={hamburgerWebClass} on:click={handleClick}>
		<span class="sr-only">Open menu</span>
		<Icon name="menu" size={24} viewBox="0 0 24 24" iconColor="dark:text-gray-primary"
			><Menu /></Icon
		>
	</button>
{/if}
