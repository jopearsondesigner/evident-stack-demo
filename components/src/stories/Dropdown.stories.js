import Dropdown from '../lib/dropdown/Dropdown.svelte';

export default {
	title: 'Global/Dropdown',
	component: Dropdown,
	argTypes: {}
};

const Template = (/** @type {any} */ args) => ({
	Component: Dropdown,
	props: args
});

export const Default = Template.bind({});
// @ts-ignore
Default.args = {};
