import Canvas from '$lib/design/CanvasComposite.svelte';

export default {
	title: 'Evident Design/Canvas',
	component: Canvas,
	argTypes: {}
};

// eslint-disable-next-line no-unused-vars
const Template = (/** @type {any} */ args) => ({
	Component: Canvas,
	props: args
});

export const Default = Template.bind({});
Default.args = {};
