<script lang='ts'>
	import classNames from 'classnames';
	import {createEventDispatcher} from 'svelte';
	import Icon from "../Icon.svelte";
	import CloseSidebar from "../icons/CloseSidebar.svelte";
  	import OpenSidebar from "../icons/OpenSidebar.svelte";
	export let navHamburgerClass = 'cursor-default';
	const dispatch = createEventDispatcher();
	function handleClick(event: any) {
		dispatch('click', event);
	}
	let hidden = true;
	let toggle = () => {
		hidden = !hidden;
	};
</script>

<button class={classNames(navHamburgerClass)} on:click={() => toggle()} on:click={handleClick}>
	{#if !hidden}
	<Icon
	  name="open-sidebar"
	  size={32}
	  viewBox="0 0 28 28"
	  iconColor="text-gray-brand-1"><OpenSidebar /></Icon
	>
  {:else}
	<Icon
	  name="close-sidebar"
	  size={32}
	  viewBox="0 0 28 28"
	  iconColor="text-gray-brand-1"><CloseSidebar /></Icon
	>
  {/if}
</button>
