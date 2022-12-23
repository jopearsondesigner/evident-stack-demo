<script>
	import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
	import Icon from '../lib/Icon.svelte';
	import Home from '../lib/icons/Home.svelte';
	import Docs from '../lib/icons/Docs.svelte';
	import Support from '../lib/icons/Support.svelte';
	import AdminPortal from '../lib/icons/AdminPortal.svelte';
	import CloseSidebar from '../lib/icons/CloseSidebar.svelte';
	import OpenSidebar from '../lib/icons/OpenSidebar.svelte';
</script>

<Meta
	title="Global/Icon"
	component={Icon}
	argTypes={{
		size: { control: 'text' },
		viewBox: { control: 'text' },
		name: {
			control: {
				type: 'select',
				options: [
					'add',
					'adminPortal',
					'arrowRight',
					'close',
					'closeSidebar',
					'closeUp',
					'collaborations',
					'design',
					'details',
					'docs',
					'eventModels',
					'file',
					'generateAvro',
					'hamburger',
					'hamburgerContextMenu',
					'home-solid',
					'home',
					'implement',
					'importEventModel',
					'jobGears',
					'json',
					'manageSchemas',
					'openRight',
					'openSidebar',
					'operate',
					'profile',
					'sliceThumbArrow',
					'support'
				]
			}
		},
		btnColor: { control: 'color' }
	}}
/>

<Template let:args>
	<Icon {...args} />
</Template>

<Story
	name="Home"
	args={{
		name: 'home',
		size: 20,
		iconColor: 'fill-color text-white',
		pathName: Home
	}}
/>

<Story
	name="Docs"
	args={{
		name: 'home',
		size: 20,
		iconColor: 'fill-color text-white',
		pathName: Docs
	}}
/>

<Story
	name="Support"
	args={{
		name: 'home',
		size: 20,
		iconColor: 'fill-color text-white',
		pathName: Support
	}}
/>

<Story
	name="Admin Portal"
	args={{
		name: 'home',
		size: 20,
		iconColor: 'fill-color text-white',
		pathName: AdminPortal
	}}
/>

<Story
	name="Close Sidebar"
	args={{
		name: 'home',
		size: 28,
		iconColor: 'fill-color text-white',
		pathName: CloseSidebar
	}}
/>

<Story
	name="Open Sidebar"
	args={{
		name: 'home',
		size: 28,
		iconColor: 'fill-color text-white',
		pathName: OpenSidebar
	}}
/>
