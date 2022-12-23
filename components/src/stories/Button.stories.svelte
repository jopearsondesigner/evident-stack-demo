<script>
	import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
	import Button from '../lib/Button.svelte';
</script>

<Meta
	title="Global/Button"
	component={Button}
	argTypes={{
		label: { control: 'text' },
		primary: { control: 'boolean' },
		textColor: {
			control: { type: 'select', options: ['dark', 'light'] }
		},
		backgroundColor: { control: 'color' },
		size: {
			control: { type: 'select', options: ['small', 'medium', 'large'] }
		},
		onClick: { action: 'onClick' }
	}}
/>

<Template let:args>
	<Button {...args} on:click={args.onClick} />
</Template>

<Story
	name="Primary"
	args={{
		primary: true
	}}
/>

<Story name="Secondary" args={{}} />

<Story
	name="Large"
	args={{
		size: 'large'
	}}
/>

<Story
	name="Small"
	args={{
		size: 'small'
	}}
/>

<Story name="Warning">
	<Button warning />
</Story>

<Story name="Success">
	<Button success />
</Story>
