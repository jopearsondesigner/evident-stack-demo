<script lang="ts">
	import classNames from 'classnames';
	import Navbar from './Navbar.svelte';
	import NavInner from './NavInner.svelte';
	import NavLeft from './NavLeft.svelte';
	import NavHamburger from './NavHamburger.svelte';
	import NavBrand from './NavBrand.svelte';
	import NavToolbar from './NavToolbar.svelte';
	import NavRight from './NavRight.svelte';
	import Button from '../Button.svelte';
	import Icon from '../Icon.svelte';
	import IconButton from '../IconButton.svelte';
	import Avatar from '../Avatar.svelte';
	import Docs from '../icons/Docs.svelte';
	import Support from '../icons/Support.svelte';
	import AdminPortal from '../icons/AdminPortal.svelte';
	import Home from '../icons/Home.svelte';
	import Logo from '../assets/images/global/onoteLogo.svg';
	export let alt="Brand Logo";
	export let user=true;
</script>

<Navbar>
	<NavInner>
		<NavLeft>
			<NavHamburger/>
			<NavBrand src={Logo} {alt}/>
			<NavToolbar>
				<IconButton iconBtnClass="hover:bg-gray-brand-1/0">
					<Icon
					name="home"
					iconColor="text-gray-brand-1 hover:text-brand-primary transition duration-200 ease-in">
						<Home />
					</Icon>
				</IconButton>
			</NavToolbar>
		</NavLeft>
		<NavRight>
			{#if user}
			<div class="tooltip tooltip-bottom" data-tip="Docs">
				<IconButton><Icon name="docs"><Docs/></Icon></IconButton>
			</div>
			<div class="tooltip tooltip-bottom" data-tip="Support">
				<IconButton><Icon name="support"><Support/></Icon></IconButton>
			</div>
			<div class="tooltip tooltip-bottom" data-tip="Admin Portal">
				<IconButton><Icon name="admin-portal"><AdminPortal/></Icon></IconButton>
			</div>
			<span class="flex justify-center px-2">
				<Avatar/>
			</span>
			{:else}
			<Button color="secondary" size="sm" on:click label="Log in" />
			<Button gradient color="primary" size="sm" on:click label="Sign up" />
			{/if}
		</NavRight>
	</NavInner>
</Navbar>
