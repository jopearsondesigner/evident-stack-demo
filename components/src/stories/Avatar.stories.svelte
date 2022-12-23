<script>
	import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
	import Avatar from '../lib/Avatar.svelte';
</script>

<Meta title="Global/Avatar" component={Avatar} argTypes={{}} />

<Template let:args>
	<Avatar {...args} />
</Template>

<Story name="Avatar with presence indicator" args={{}} />
