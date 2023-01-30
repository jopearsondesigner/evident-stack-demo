import Placement from '../../lib/design/grid/Placement.svelte';

export default {
  title: 'Evident Design/Placement',
  component: Placement,
};

// eslint-disable-next-line no-unused-vars
const Template = (args) => ({
  Component: Placement,
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
