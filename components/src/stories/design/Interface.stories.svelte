<script>
	// @ts-nocheck

	import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

	import Interface from '../../lib/design/grid/Interface.svelte';
</script>

<Meta title="Evident Design/Placement/Interface" component={Interface} />

<Template let:args>
	<Interface {...args} id="a-uuid" title="A nice Interface" row={1} column={3} cursor={false} />
</Template>

<Story name="blank" args={{ type: 'blank' }} />
