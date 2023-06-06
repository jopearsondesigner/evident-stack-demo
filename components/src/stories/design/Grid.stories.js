import Grid from '$lib/design/Grid.svelte';

export default {
	title: 'Evident Design/Grid',
	component: Grid,
	argTypes: {}
};

// eslint-disable-next-line no-unused-vars
const Template = (/** @type {any} */ args) => ({
	Component: Grid,
	props: args
});

export const Default = Template.bind({});
Default.args = {
	default_audience_placements: [
		,
		,
		{ id: '1', title: 'An interface!', description: 'blah *blah* **blah**' },
		,
		,
		,
		,
		,
		,
		,
		,
		{ id: '2', title: 'Another interface!', description: 'blah *blah* **blah**' }
	],
	audiences: [
		{
			title: 'A named audience',
			placements: [
				,
				,
				,
				{ id: '3', title: 'An interface!', description: 'blah *blah* **blah**' },
				,
				{ id: '4', title: 'Another interface!', description: 'blah *blah* **blah**' }
			]
		},
		{
			title: 'Another named audience',
			placements: [
				,
				{ id: '5', title: 'An interface!', description: 'blah *blah* **blah**' },
				,
				,
				,
				{ id: '6', title: 'Another interface!', description: 'blah *blah* **blah**' }
			]
		}
	],
	timeline_placements: [
		,
		{ id: '5', title: 'A command!', type: 'command', description: 'blah *blah* **blah**' },
		,
		{ id: '6', title: 'A read model!', type: 'read_model', description: 'blah *blah* **blah**' },
		,
		{ id: '7', title: 'Another command!', type: 'command', description: 'blah *blah* **blah**' },
		{
			id: '8',
			title: 'Another read model!',
			type: 'read_model',
			description: 'blah *blah* **blah**'
		}
	],
	streams: [
		{
			title: 'A named stream',
			placements: [
				,
				,
				,
				{ id: '9', title: 'An event!', description: 'blah *blah* **blah**' },
				,
				,
				,
				,
				,
				,
				,
				,
				,
				{ id: '10', title: 'Another event!', description: 'blah *blah* **blah**' }
			]
		}
	],
	default_stream_placements: [
		,
		,
		{ id: '11', title: 'An event!', description: 'blah *blah* **blah**' },
		,
		,
		{ id: '12', title: 'Another event!', description: 'blah *blah* **blah**' }
	]
};
