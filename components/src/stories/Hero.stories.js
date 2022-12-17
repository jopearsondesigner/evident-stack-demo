import Hero from '../lib/hero/HeroExample.svelte';

export default {
	title: 'website/Hero',
	component: Hero,
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
