<script lang="ts">
	import classNames from 'classnames';
	import Navbar from './Navbar.svelte';
	import NavHamburger from './NavHamburger.svelte';
	import NavBrand from './NavBrand.svelte';
	import Nav from './Nav.svelte';
	import NavInner from './NavInner.svelte';
	import NavToolbar from './NavToolbar.svelte';
	import Icon from '../Icon.svelte';
	import Button from '../Button.svelte';
	import IconButton from '../IconButton.svelte';
	import Avatar from '../Avatar.svelte';
	import Home from '../icons/Home.svelte';
	import Docs from '../icons/Docs.svelte';
	import Support from '../icons/Support.svelte';
	import AdminPortal from '../icons/AdminPortal.svelte';
	import Logo from '../assets/images/product/design/evidentDesignLogo.svg';
	import A from '../typography/A.svelte';
	let website: boolean = false;
	export let user: boolean = true;
</script>

<Navbar website>
	<NavInner navDivClass="flex justify-between items-center">
		<NavToolbar navClass="px-3 h-9 inline-flex space-x-4 mx-3 items-center">
			<NavHamburger {website} />
			<NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3" />
			<div class="px-3 h-9 border-x border-gray-secondary flex items-center">
				<IconButton iconBtnClass="hover:bg-gray-brand-1/0">
					<Icon
						size={16}
						name="home"
						iconColor="text-body-light hover:text-brand-primary transition duration-200 ease-in"
					>
						<Home />
					</Icon>
				</IconButton>
			</div>
		</NavToolbar>
		<NavToolbar
			navClass="px-3 h-9 inline-flex space-x-2 mx-3 items-center border-l border-gray-secondary"
		>
			{#if user}
				<div class="tooltip tooltip-bottom" data-tip="Docs">
					<IconButton><Icon name="docs" size={18}><Docs /></Icon></IconButton>
				</div>
				<div class="tooltip tooltip-bottom" data-tip="Support">
					<IconButton><Icon name="support" size={18}><Support /></Icon></IconButton>
				</div>
				<div class="tooltip tooltip-bottom" data-tip="Admin Portal">
					<IconButton><Icon name="admin-portal" size={18}><AdminPortal /></Icon></IconButton>
				</div>
				<span class="flex justify-center px-3">
					<Avatar />
				</span>
			{:else}
				<A class="text-sm mr-3 transition duration-300 ease-in">Log in</A>
				<Button gradient color="brandStackPrimary" size="sm" on:click label="Sign up" />
			{/if}
		</NavToolbar>
	</NavInner>
</Navbar>

<style>
	.tooltip:before {
		font-size: 12px;
	}
</style>
