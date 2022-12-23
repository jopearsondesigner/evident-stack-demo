<script>
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
	import IconButton from '../lib/IconButton.svelte';
	import Docs from '../lib/icons/Docs.svelte';
	import Support from '../lib/icons/Support.svelte';
	import AdminPortal from '../lib/icons/AdminPortal.svelte';
</script>

<Meta
	title="Global/IconButton"
	component={IconButton}
	argTypes={{
		backgroundColor: { control: 'color' },
		buttonSize: {
			control: { type: 'select', options: ['xs', 'sm', 'md', 'lg', 'xl'] }
		},
		onClick: { action: 'onClick' }
	}}
/>

<Template let:args>
	<IconButton {...args} on:click={args.onClick} />
</Template>

<Story
	name="Docs Button"
	args={{
		buttonSize: 'sm',
		name: 'docs',
		size: 20,
		pathName: Docs
	}}
/>

<Story
	name="Support Button"
	args={{
		btnSize: 'sm',
		name: 'support',
		size: 20,
		pathName: Support
	}}
/>

<Story
	name="Admin Portal Button"
	args={{
		btnSize: 'sm',
		name: 'adminPortal',
		size: 20,
		pathName: AdminPortal
	}}
/>
