import Blog from '../lib/blog/BlogExample.svelte';

export default {
	title: 'website/Blog',
	component: Blog,
	parameters: {
		layout: 'fullscreen'
	},
	argTypes: {}
};

// eslint-disable-next-line no-unused-vars
const Template = (/** @type {any} */ args) => ({});

export const Default = Template.bind({});
// @ts-ignore
Default.args = {};
