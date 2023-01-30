import BlogPage from '../lib/blog/BlogPage.svelte';

export default {
	title: 'website/Blog Page',
	component: BlogPage,
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
