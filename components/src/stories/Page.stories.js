import Page from '../lib/Page.svelte';

export default {
	title: 'website/Page',
	component: Page,
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
