<script>
	// @ts-nocheck

	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

	import SchemaView from '$lib/form/SchemaViewExample.svelte';
</script>

<Meta title="product/Schema View" component={SchemaView} />

<Template let:args>
	<SchemaView {...args} id="a-uuid" title="A nice Interface" row={1} column={3} cursor={false} />
</Template>

<Story name="blank" args={{ type: 'blank' }} />
