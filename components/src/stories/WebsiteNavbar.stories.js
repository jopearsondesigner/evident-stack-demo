import WebsiteNavbar from '../lib/navbar/NavbarWebExample.svelte';

export default {
	title: 'Website/Navbar',
	component: WebsiteNavbar,
	argTypes: {}
};

const Template = (/** @type {any} */ args) => ({
	Component: WebsiteNavbar,
	props: args
});

export const Default = Template.bind({});
// @ts-ignore
Default.args = {};
