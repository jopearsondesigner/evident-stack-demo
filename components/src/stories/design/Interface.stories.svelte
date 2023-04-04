<script>
	// @ts-nocheck

	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

	import Interface from '../../lib/design/grid/Interface.svelte';
</script>

<Meta title="Evident Design/Placement/Interface" component={Interface} />

<Template let:args>
	<Interface id="a-uuid" row={1} column={3} {...args} />
</Template>

<Story name="blank" args={{ type: 'blank', name: 'A nice Interface' }} />
