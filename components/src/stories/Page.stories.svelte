<script>
	import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
	import Page from '../lib/Page.svelte';
</script>

<Meta title="Example/Page" component={Page} argTypes={{}} />

<Template let:args>
	<Page {...args} />
</Template>

<Story name="Default" />
