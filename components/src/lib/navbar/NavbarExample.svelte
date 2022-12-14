<script lang="ts">
	import classNames from 'classnames';
	import './navbar.css';
	import Button from '../Button.svelte';
	import Icon from '../Icon.svelte';
	import IconButton from '../IconButton.svelte';
	import Avatar from '../Avatar.svelte';
	import CloseSidebar from '../icons/CloseSidebar.svelte';
	import OpenSidebar from '../icons/OpenSidebar.svelte';
	import Docs from '../icons/Docs.svelte';
	import Support from '../icons/Support.svelte';
	import AdminPortal from '../icons/AdminPortal.svelte';
	import Home from '../icons/Home.svelte';
	import Logo from '../assets/images/onoteLogo.svg';
	export let href = '/';
	export let user = '';
	export let navbarClass = 'relative shadow-header bg-white w-full max-h-12 px-3 py-1.5 text-body';
	export let navDivCLass = 'flex justify-between items-center';
	export let navClass = 'inline-flex space-x-3 mx-3 items-center';
	export let navHamburgerClass = 'cursor-default';
	export let navBrandClass = 'mx-3 flex justify-center cursor-default h-[24px]';
	export let toolbarClass = 'px-3 h-9 border-x border-gray-secondary flex items-center';
	export let avatarClass = 'flex items-center justify-center px-3';
	export let alt = 'oNote Logo';
	export let iconColor = 'text-gray-brand-1';
	export let size = 20;
	let hidden = true;
	let toggle = () => {
		hidden = !hidden;
	};
</script>

<header id="navbar" class={classNames(navbarClass)}>
	<div id="navbarInner" class={classNames(navDivCLass)}>
		<nav id="navLeft" class={classNames(navClass)}>
			<button id="navHamburger" class={classNames(navHamburgerClass)} on:click={() => toggle()}>
				{#if !hidden}
					<Icon name="open-sidebar" size={32} viewBox="0 0 28 28" iconColor="text-body"
						><OpenSidebar /></Icon
					>
				{:else}
					<Icon name="close-sidebar" size={32} viewBox="0 0 28 28" iconColor="text-body"
						><CloseSidebar /></Icon
					>
				{/if}
			</button>
			<a id="navBrand" {href} class={classNames(navBrandClass)}>
				<img src={Logo} height="22" class={classNames(navBrandClass)} {alt} />
			</a>
			<div id="navToolbar" class={classNames(toolbarClass)}>
				<IconButton iconBtnClass="hover:bg-transparent"
					><Icon name="home" {size} iconColor="text-gray-brand-1 hover:text-onote-brand"
						><Home /></Icon
					></IconButton
				>
			</div>
		</nav>
		<nav id="navRight" class={classNames(navClass)}>
			{#if user}
				<div class="tooltip tooltip-bottom" data-tip="Docs">
					<IconButton><Icon name="docs" {size} {iconColor}><Docs /></Icon></IconButton>
				</div>
				<div class="tooltip tooltip-bottom" data-tip="Support">
					<IconButton><Icon name="support" {size} {iconColor}><Support /></Icon></IconButton>
				</div>
				<div class="tooltip tooltip-bottom" data-tip="Admin Portal">
					<IconButton><Icon name="adminPortal" {size} {iconColor}><AdminPortal /></Icon></IconButton
					>
				</div>
				<div class={classNames(avatarClass)}>
					<Avatar avatarClass="w-8 rounded-full" />
				</div>
			{:else}
				<Button size="small" textColor="light" on:click label="Log in" />
				<Button primary size="small" on:click label="Sign up" />
			{/if}
		</nav>
	</div>
</header>
