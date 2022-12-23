import DrawerExample from '../lib/drawer/DrawerExample.svelte';

// More on default export: https://storybook.js.org/docs/react/writing-stories/introduction#default-export
// More on argTypes: https://storybook.js.org/docs/svelte/api/argtypes
export default {
	title: 'Global/Drawer',
	component: DrawerExample,
	argTypes: {}
};

// More on component templates: https://storybook.js.org/docs/svelte/writing-stories/introduction#using-args
const Template = (/** @type {any} */ args) => ({
	Component: DrawerExample,
	props: args
});

// More on args: https://storybook.js.org/docs/svelte/writing-stories/args
export const Default = Template.bind({});
// @ts-ignore
Default.args = {};
