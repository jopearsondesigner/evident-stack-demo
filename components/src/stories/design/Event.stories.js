import Event from '$lib/design/grid/Event.svelte';

export default {
	title: 'Evident Design/Placement/Event',
	component: Event
};

// eslint-disable-next-line no-unused-vars
const Template = (/** @type {any} */ args) => ({
	Component: Event,
	props: args
});

export const Default = Template.bind({});
Default.args = {
	id: 'uuid',
	title: 'An Event',
	row: 1,
	column: 0
};
