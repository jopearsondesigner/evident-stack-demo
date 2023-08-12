import CodeMirror from '$lib/data/CodeMirrorComposite.svelte';

export default {
  title: 'Evident Data/CodeMirror',
  component: CodeMirror,
  argTypes: {}
};

// eslint-disable-next-line no-unused-vars
const Template = (/** @type {any} */ args) => ({
  Component: CodeMirror,
  props: args
});

export const Default = Template.bind({});
// @ts-ignore
Default.args = {};
