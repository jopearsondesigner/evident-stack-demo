import Navbar from '../lib/navbar/NavbarWebExample.svelte';

export default {
	title: 'Website/Navbar',
	component: Navbar,
	argTypes: {}
};

const Template = (/** @type {any} */ args) => ({
	Component: Navbar,
	props: args
});

export const Default = Template.bind({});
// @ts-ignore
Default.args = {};
