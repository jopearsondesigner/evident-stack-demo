<script>
	import 'components/src/app.css';
	import { Accordion, AccordionItem } from 'svelte-accessible-accordion';
	import Grid from '$lib/design/Grid.svelte';
	import Drawer from '$lib/drawer/Drawer.svelte';
	import Sidebar from '$lib/drawer/Sidebar.svelte';
	import SidebarWrapper from '$lib/drawer/SidebarWrapper.svelte';
	import SidebarContainer from '$lib/drawer/SidebarContainer.svelte';
	import SidebarGroup from '$lib/drawer/SidebarGroup.svelte';
	import SidebarDropdownWrapper from '$lib/drawer/SidebarDropdownWrapper.svelte';
	import SidebarDropdownItem from '$lib/drawer/SidebarDropdownItem.svelte';
	import SidebarItem from '$lib/drawer/SidebarItem.svelte';
	import GenerateAvro from '$lib/icons/GenerateAvro.svelte';
	import ThemeSwitch from '$lib/utils/ThemeSwitch.svelte';
	import Navbar from '$lib/navbar/Navbar.svelte';
	import NavHamburger from '$lib/navbar/NavHamburger.svelte';
	import NavBrand from '$lib/navbar/NavBrand.svelte';
	import NavInner from '$lib/navbar/NavInner.svelte';
	import NavToolbar from '$lib/navbar/NavToolbar.svelte';
	import Icon from '$lib/Icon.svelte';
	import Button from '$lib/Button.svelte';
	import Download from '$lib/icons/Download.svelte';
	import IconButton from '$lib/IconButton.svelte';
	import Avatar from '$lib/Avatar.svelte';
	import Docs from '$lib/icons/Docs.svelte';
	import Support from '$lib/icons/Support.svelte';
	import A from '$lib/typography/A.svelte';
	import MaybeTooltip from '$lib/utils/MaybeTooltip.svelte';
	import Logo from '$lib/assets/images/global/evidentStackLogo.svg';
	import DesignLogo from '$lib/assets/images/product/design/evidentDesignLogo.svg';
	import DataLogo from '$lib/assets/images/product/data/evidentDataLogo.svg';
	import DomainFunctionsLogo from '$lib/assets/images/product/DomainFunctions/evidentDomainFunctionsLogo.svg';
	import DeployLogo from '$lib/assets/images/product/deploy/evidentDeployLogo.svg';
	import DatabaseLogo from '$lib/assets/images/product/database/evidentDatabaseLogo.svg';
	let feature = true;
	let isClosed = true;
	export let alt = 'Brand Logo';
	export let user = true;
	let hidden = true;
	let hiddenRight = true;
	let handleDrawer = () => {
		hidden = !hidden;
	};
	let handleClick = () => {
		isClosed = !isClosed;
	};
	let website = false;

	/**
	 * @type {boolean}
	 */
	let expanded = true;
</script>

<span class="lg:block hidden right-0 z-40 fixed pt-4 pr-10 mt-16"><ThemeSwitch /></span>
<Drawer {hidden}>
	<Sidebar class={!isClosed ? 'w-[416px]' : 'w-[239px]'} {isClosed}>
		<SidebarWrapper>
			<Accordion>
				<SidebarContainer src={DesignLogo} title="Design" id="design" bind:expanded>
					<SidebarGroup>
						<SidebarDropdownWrapper label="Event Model Schema" on:click={() => handleClick()}>
							<Icon
								slot="icon"
								name="generate-avro"
								size={16}
								iconColor="fill-current text-gray-brand-4 dark:text-gray-brand-2"
								><GenerateAvro /></Icon
							>
							<SidebarDropdownItem label="Hello, please develop me!" {feature} />
						</SidebarDropdownWrapper>
						<SidebarItem blank>
							<Button
								label="Export"
								gradient
								color="ghost"
								size="sm"
								className="my-4"
								on:click
								class=""
								><Icon
									slot="icon"
									name="download"
									size={12}
									iconColor="text-body-light dark:text-body-dark"
									class="inline-flex mb-1"><Download /></Icon
								></Button
							>
						</SidebarItem>
					</SidebarGroup>
				</SidebarContainer>
				<SidebarContainer src={DataLogo} id="data" title="Data" on:click={() => (isClosed = true)}>
					<SidebarGroup>
						<SidebarItem label="Hello, please develop me!" blank />
					</SidebarGroup>
				</SidebarContainer>
				<SidebarContainer
					src={DomainFunctionsLogo}
					id="domain-functions"
					title="Domain Functions"
					on:click={() => (isClosed = true)}
				>
					<SidebarGroup>
						<SidebarItem label="Hello, please develop me!" blank />
					</SidebarGroup>
				</SidebarContainer>
				<SidebarContainer
					src={DeployLogo}
					id="deploy"
					title="Deploy"
					on:click={() => (isClosed = true)}
				>
					<SidebarGroup>
						<SidebarItem label="Hello, please develop me!" blank />
					</SidebarGroup>
				</SidebarContainer>
				<SidebarContainer
					src={DatabaseLogo}
					id="database"
					title="Database"
					on:click={() => (isClosed = true)}
				>
					<SidebarGroup>
						<SidebarItem label="Hello, please develop me!" blank />
					</SidebarGroup>
				</SidebarContainer>
			</Accordion>
		</SidebarWrapper>
	</Sidebar>
</Drawer>

<div class="text-center z-30 absolute inset-x-0 mt-20">
	<!--For testing-->
	<Button gradient color="primary" size="sm" on:click={() => (hiddenRight = false)} class=""
		>Show Sidebar</Button
	><br />
	<button on:click={() => (hiddenRight = true)} class="mt-4">Close</button>
</div>

<Drawer placement="right" bind:hidden={hiddenRight} drawerRight>
	<div class="bg-white dark:bg-dark-2 w-[480px] p-4 h-screen pt-20">Hello please develop me!</div>
</Drawer>

<Navbar>
	<NavInner navDivClass="flex justify-between items-center">
		<NavToolbar navClass="px-3 h-9 inline-flex space-x-4 mx-3 items-center">
			<NavHamburger {website} on:click={() => handleDrawer()} on:click={() => (isClosed = true)} />
			<NavBrand src={Logo} height={28} logoClass="flex no-underline mx-3 cursor-default" />
		</NavToolbar>
		<NavToolbar
			navClass="px-3 h-9 inline-flex space-x-2 mx-3 items-center border-l border-gray-secondary dark:border-border-dark"
		>
			{#if user}
				<MaybeTooltip tip="Docs">
					<IconButton
						><Icon name="docs" size={18} iconColor="text-body-light dark:text-body-dark"
							><Docs /></Icon
						></IconButton
					>
				</MaybeTooltip>
				<MaybeTooltip tip="Support">
					<IconButton
						><Icon name="support" size={18} iconColor="text-body-light dark:text-body-dark"
							><Support /></Icon
						></IconButton
					>
				</MaybeTooltip>
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

<main
	class="left-0 relative transition-all duration-[200ms] ml-0"
	class:left-60={!hidden}
	class:ml-[177px]={!isClosed}
	class:ease-in={!isClosed}
>
	<Grid
		default_audience_placements={[
			null,
			null,
			{ id: '1', title: 'An interface!', description: 'blah *blah* **blah**' },
			null,
			null,
			null,
			null,
			null,
			null,
			null,
			null,
			{ id: '2', title: 'Another interface!', description: 'blah *blah* **blah**' }
		]}
		audiences={[
			{
				title: 'A named audience',
				placements: [
					null,
					null,
					null,
					{ id: '3', title: 'An interface!', description: 'blah *blah* **blah**' },
					null,
					{ id: '4', title: 'Another interface!', description: 'blah *blah* **blah**' }
				]
			},
			{
				title: 'Another named audience',
				placements: [
					null,
					{ id: '5', title: 'An interface!', description: 'blah *blah* **blah**' },
					null,
					null,
					null,
					{ id: '6', title: 'Another interface!', description: 'blah *blah* **blah**' }
				]
			}
		]}
		timeline_placements={[
			null,
			{ id: '5', title: 'A command!', type: 'command', description: 'blah *blah* **blah**' },
			null,
			{ id: '6', title: 'A read model!', type: 'readModel', description: 'blah *blah* **blah**' },
			null,
			{ id: '7', title: 'Another command!', type: 'command', description: 'blah *blah* **blah**' },
			{
				id: '8',
				title: 'Another read model!',
				type: 'readModel',
				description: 'blah *blah* **blah**'
			}
		]}
		streams={[
			{
				title: 'A named stream',
				placements: [
					null,
					null,
					null,
					{ id: '9', title: 'An event!', description: 'blah *blah* **blah**' },
					null,
					null,
					null,
					null,
					null,
					null,
					null,
					null,
					null,
					{ id: '10', title: 'Another event!', description: 'blah *blah* **blah**' }
				]
			}
		]}
		default_stream_placements={[
			null,
			null,
			{ id: '11', title: 'An event!', description: 'blah *blah* **blah**' },
			null,
			null,
			{ id: '12', title: 'Another event!', description: 'blah *blah* **blah**' }
		]}
	/>
</main>

<style>
	:global([data-accordion]) {
		height: 100%;
	}
</style>
