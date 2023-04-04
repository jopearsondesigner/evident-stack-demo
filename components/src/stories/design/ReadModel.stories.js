import ReadModel from '$lib/design/grid/ReadModel.svelte';

export default {
  title: 'Evident Design/Placement/Read Model',
  component: ReadModel,
};

// eslint-disable-next-line no-unused-vars
const Template = (args) => ({
  Component: ReadModel,
  props: args
});

export const Default = Template.bind({});
Default.args = {
  id: "uuid",
  title: "A Read Model",
  row: 1,
  column: 0
};
