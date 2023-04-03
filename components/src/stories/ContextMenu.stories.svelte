<script>
	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

	import ContextMenu from '$lib/context/ContextMenuExample.svelte';
</script>

<Meta title="product/Context Menu" component={ContextMenu} />

<Template let:args>
	<ContextMenu {...args} />
</Template>

<Story name="blank" args={{ type: 'blank' }} />
