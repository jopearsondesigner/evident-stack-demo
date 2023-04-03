<script>
	// @ts-nocheck

	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

	import SchemaView from '$lib/form/SchemaViewExample.svelte';
</script>

<Meta title="product/Schema View" component={SchemaView} />

<Template let:args>
	<SchemaView />
</Template>

<Story name="blank" args={{ type: 'blank' }} />
