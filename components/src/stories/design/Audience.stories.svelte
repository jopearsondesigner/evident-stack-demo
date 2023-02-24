<script>
  import { Meta, Story, Template } from '@storybook/addon-svelte-csf';

  import Audience from '$lib/design/grid/Audience.svelte';
  import Interface from '$lib/design/grid/Interface.svelte';
  import EmptyCell from '$lib/design/grid/EmptyCell.svelte';
</script>

<Meta title="Evident Design/Lane/Audience" component={Audience} />

<Template let:args>
  <Audience {...args}>
    <Interface id="a-uuid" type="blank" title="Blank interface" row={0} column={0} />
    <EmptyCell row={0} column={1} />
    <EmptyCell row={0} column={2} />
    <EmptyCell row={0} column={3} />
  </Audience>
</Template>

<Story name="Empty Default">
  <Audience row={0} />
</Story>

<Story name="Empty">
  <Audience row={0} title="A nice Audience lane" />
</Story>

<Story name="Non-empty Default" args={{row: 0}} />
<Story name="Non-empty" args={{row: 0, title: "A non-empty named Audience"}} />
