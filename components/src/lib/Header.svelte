<script>
	import './styles//header.css';
	import Button from './Button.svelte';
	import Icon from './Icon.svelte';
	import IconButton from './IconButton.svelte';
	import Avatar from './Avatar.svelte';
	import CloseSidebar from './icons/CloseSidebar.svelte';
	import OpenSidebar from './icons/OpenSidebar.svelte';
	import Docs from './icons/Docs.svelte';
	import Support from './icons/Support.svelte';
	import AdminPortal from './icons/AdminPortal.svelte';
	import Home from './icons/Home.svelte';
	import Logo from './assets/images/onoteLogo.svg';
	import { createEventDispatcher } from 'svelte';
	export let href = '/';

	/**
	 * @type {null}
	 */
	export let user = null;

	const dispatch = createEventDispatcher();

	/**
	 * @param {any} event
	 */
	function onLogin(event) {
		dispatch('login', event);
	}
	/**
	 * @param {any} event
	 */
	/* function onLogout(event) {
		dispatch('logout', event);
	  } */
	/**
	 * @param {any} event
	 */
	function onCreateAccount(event) {
		dispatch('createAccount', event);
	}
</script>

<header>
	<div class="navbar">
		<div class="navbar-start">
			<nav class="flex space-x-6 mx-3 items-center">
				<label class="swap">
					<input type="checkbox" />
					<Icon name="open-sidebar" size={28} class="swap-on" viewBox="0 0 28 28"
						><OpenSidebar /></Icon
					>
					<Icon name="close-sidebar" size={28} class="swap-off" viewBox="0 0 28 28"
						><CloseSidebar /></Icon
					>
				</label>
				<a {href} class="flex justify-cente cursor-default">
					<img src={Logo} height="22" class="h-[22px]" alt="oNote Logo" />
				</a>
				<div class="px-3 border-x border-gray-primary my-2">
					<div class="tooltip tooltip-bottom" data-tip="Back">
						<IconButton><Icon name="home" iconColor="text-body-light"><Home /></Icon></IconButton>
					</div>
				</div>
			</nav>
		</div>
		<div class="navbar-end">
			<nav class="flex space-x-3 mx-3 items-center">
				{#if user}
					<div class="tooltip tooltip-bottom" data-tip="Docs">
						<IconButton><Icon name="docs"><Docs /></Icon></IconButton>
					</div>
					<div class="tooltip tooltip-bottom" data-tip="Support">
						<IconButton><Icon name="support"><Support /></Icon></IconButton>
					</div>
					<div class="tooltip tooltip-bottom" data-tip="Admin Portal">
						<IconButton><Icon name="adminPortal"><AdminPortal /></Icon></IconButton>
					</div>
					<div class="dropdown dropdown-end">
						<label tabindex="0" class="btn btn-ghost btn-circle btn-xs">
							<Avatar avatarClass="w-7 rounded-full" />
						</label>
						<ul
							tabindex="0"
							class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-white rounded-box w-52"
						>
							<li><a>Profile & Billing</a></li>
							<li>
								<a class="justify-between">
									Settings
									<span class="badge text-xxs">New</span>
								</a>
							</li>
							<li><a>Sign Out</a></li>
						</ul>
					</div>
				{/if}
				{#if !user}
					<Button size="small" textColor="light" on:click={onLogin} label="Log in" />
					<Button primary size="small" on:click={onCreateAccount} label="Sign up" />
				{/if}
			</nav>
		</div>
	</div>
</header>

<style>
	.btn-circle:where(.btn-xs) {
		height: 2.25rem;
		width: 2.25rem;
		border-radius: 9999px;
		padding: 0px;
	}
</style>
