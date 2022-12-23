import Main from '../lib/MainExample.svelte';

export default {
	title: 'website/Main',
	component: Main,
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
