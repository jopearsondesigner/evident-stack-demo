import Command from '$lib/design/grid/Command.svelte';

export default {
  title: 'Evident Design/Placement/Command',
  component: Command,
};

// eslint-disable-next-line no-unused-vars
const Template = (args) => ({
  Component: Command,
  props: args
});

export const Default = Template.bind({});
Default.args = {
  cursor: false,
  id: "uuid",
  type: "command",
  title: "A Command",
  row: 1,
  column: 0
};
