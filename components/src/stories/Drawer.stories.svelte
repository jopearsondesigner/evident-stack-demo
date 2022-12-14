<script>
	import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
	import '../lib/drawer/drawer.css';
	import Drawer from '../lib/drawer/Drawer.svelte';
	import Button from '../lib/Button.svelte';
	import Sidebar from '../lib/drawer/Sidebar.svelte';
	import SidebarInner from '../lib/drawer/SidebarInner.svelte';
	import SidebarGroup from '../lib/drawer/SidebarGroup.svelte';
	import SidebarDropdownWrapper from '../lib/drawer/SidebarDropdownWrapper.svelte';
	import SidebarDropdownItem from '../lib/drawer/SidebarDropdownItem.svelte';
	import SidebarItem from '../lib/drawer/SidebarItem.svelte';
	import Icon from '../lib/Icon.svelte';
	import Design from '../lib/icons/Design.svelte';
	import Implement from '../lib/icons/Implement.svelte';
	import Operate from '../lib/icons/Operate.svelte';
	import Collaborations from '../lib/icons/Collaborations.svelte';
	import ManageSchemas from '../lib/icons/ManageSchemas.svelte';
	import ExportJson from '../lib/icons/Json.svelte';
	import GenerateAvro from '../lib/icons/GenerateAvro.svelte';
	import Powerup from '../lib/icons/Powerup.svelte';
	import AddIcon from '../lib/icons/AddIcon.svelte';
	let aLink = true;
	let headerText = true;
	let featureData = true;
	let hidden = true;
	let sidebarSpanClass = 'flex-1 ml-6 text-left whitespace-nowrap';
</script>

<Meta title="Global/Drawer" component={Drawer} argTypes={{}} />

<Template let:args>
	<Drawer {...args} />
</Template>

<Story name="Default">
	<div class="text-center">
		<Button primary size="small" on:click={() => (hidden = false)} class="">Show Sidebar</Button><br
		/>
		<button on:click={() => (hidden = true)} class="mt-4">Close</button>
	</div>

	<Drawer {hidden}>
		<Sidebar>
			<SidebarInner slot="eventModel">
				<SidebarGroup>
					<SidebarDropdownWrapper horizontal label="Design">
						<Icon slot="icon" name="design" iconColor="fill-current text-brand-4"><Design /></Icon>
						<SidebarDropdownItem label="Start a Collaboration">
							<Icon slot="icon" name="collaborations" iconColor="fill-current text-brand-4"
								><Collaborations /></Icon
							>
						</SidebarDropdownItem>
						<SidebarDropdownItem label="Manage Schemas">
							<Icon slot="icon" name="manage-schemas" iconColor="fill-current text-brand-4"
								><ManageSchemas /></Icon
							>
						</SidebarDropdownItem>
						<SidebarDropdownItem label="Get Design PowerUps" {aLink}>
							<Icon slot="icon" name="powerup"><Powerup /></Icon>
						</SidebarDropdownItem>
					</SidebarDropdownWrapper>
					<SidebarDropdownWrapper horizontal label="Implement">
						<Icon slot="icon" name="implement" iconColor="fill-current text-brand-4"
							><Implement /></Icon
						>
						<SidebarDropdownItem label="Export JSON">
							<Icon slot="icon" name="export-jason" iconColor="fill-current text-brand-4"
								><ExportJson /></Icon
							>
						</SidebarDropdownItem>
						<SidebarDropdownItem label="Generate Avro">
							<Icon slot="icon" name="generate-avro" iconColor="fill-current text-brand-4"
								><GenerateAvro /></Icon
							>
						</SidebarDropdownItem>
						<SidebarDropdownItem label="Get Implement PowerUps" {aLink}>
							<Icon slot="icon" name="powerup"><Powerup /></Icon>
						</SidebarDropdownItem>
					</SidebarDropdownWrapper>
					<SidebarDropdownWrapper horizontal label="Operate">
						<Icon slot="icon" name="operate" iconColor="fill-current text-brand-4"><Operate /></Icon
						>
						<SidebarDropdownItem label="Get Operate PowerUps" {aLink}>
							<Icon slot="icon" name="powerup"><Powerup /></Icon>
						</SidebarDropdownItem>
					</SidebarDropdownWrapper>
				</SidebarGroup>
				<SidebarGroup>
					<SidebarDropdownWrapper label="Slices">
						<SidebarDropdownWrapper label="State Change" {sidebarSpanClass}>
							<SidebarItem label="<State Change Data>" {featureData} />
						</SidebarDropdownWrapper>
						<SidebarDropdownWrapper label="State View" {sidebarSpanClass}>
							<SidebarItem label="<State View Data>" {featureData} />
						</SidebarDropdownWrapper>
						<SidebarDropdownWrapper label="External State Import" {sidebarSpanClass}>
							<SidebarItem label="<External State Import Data>" {featureData} />
						</SidebarDropdownWrapper>
						<SidebarDropdownWrapper label="Internal State Import" {sidebarSpanClass}>
							<SidebarItem label="<Internal State Import Data>" {featureData} />
						</SidebarDropdownWrapper>
					</SidebarDropdownWrapper>
				</SidebarGroup>
				<SidebarGroup>
					<SidebarDropdownWrapper label="Repository">
						<SidebarDropdownItem label="Repo Feature Item" />
					</SidebarDropdownWrapper>
				</SidebarGroup>
			</SidebarInner>
			<SidebarInner slot="domainModels">
				<SidebarGroup>
					<SidebarItem label="Workflows" {headerText}>
						<Icon slot="icon" name="add-icon" size={20} iconColor="fill-current text-body"
							><AddIcon /></Icon
						>
					</SidebarItem>
					<SidebarItem label="<Workflow 1>" />
					<SidebarItem label="<Workflow 2>" />
					<SidebarItem label="<Workflow 3>" />
				</SidebarGroup>
			</SidebarInner>
		</Sidebar>
	</Drawer>
</Story>
