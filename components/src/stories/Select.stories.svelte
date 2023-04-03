<script>
	// @ts-nocheck

	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

	import Select from '$lib/form/SelectView.svelte';
</script>

<Meta title="product/Select" component={Select} />

<Template let:args>
	<Select {...args} />
</Template>

<Story name="blank" args={{ type: 'blank' }} />
