import TreeView from '$lib/data/TreeViewExample.svelte';

export default {
  title: 'Evident Data/TreeView',
  component: TreeView,
  argTypes: {}
};

// eslint-disable-next-line no-unused-vars
const Template = (/** @type {any} */ args) => ({
  Component: TreeView,
  props: args
});

export const Default = Template.bind({});
// @ts-ignore
Default.args = {};
