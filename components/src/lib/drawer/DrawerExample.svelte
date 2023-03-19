<script>
	// @ts-nocheck

	import Drawer from './Drawer.svelte';
	import Sidebar from './Sidebar.svelte';
	import SidebarInner from './SidebarInner.svelte';
	import SidebarGroup from './SidebarGroup.svelte';
	import SidebarDropdownWrapper from './SidebarDropdownWrapper.svelte';
	import SidebarDropdownItem from './SidebarDropdownItem.svelte';
	import SidebarItem from './SidebarItem.svelte';
	import Button from '../Button.svelte';
	import Icon from '../Icon.svelte';
	import Design from '../icons/Design.svelte';
	import Implement from '../icons/Implement.svelte';
	import Operate from '../icons/Operate.svelte';
	import Collaborations from '../icons/Collaborations.svelte';
	import ManageSchemas from '../icons/ManageSchemas.svelte';
	import ExportJson from '../icons/Json.svelte';
	import GenerateAvro from '../icons/GenerateAvro.svelte';
	import Powerup from '../icons/Powerup.svelte';
	import AddIcon from '../icons/AddIcon.svelte';
	let aLink = true;
	let headerText = true;
	let featureData = true;
	let hidden = true;
	let sidebarSpanClass = 'flex-1 ml-6 text-left whitespace-nowrap';
</script>

<div class="text-center">
	<Button gradient color="primary" size="sm" on:click={() => (hidden = false)} class=""
		>Show Sidebar</Button
	><br />
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
					<Icon slot="icon" name="operate" iconColor="fill-current text-brand-4"><Operate /></Icon>
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
