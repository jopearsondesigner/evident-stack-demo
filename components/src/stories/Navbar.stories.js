import NavbarExample from '../lib/navbar/NavbarExample.svelte';

export default {
	title: 'global/NavbarExample',
	component: NavbarExample,
	parameters: {
		// More on Story layout: https://storybook.js.org/docs/svelte/configure/story-layout
		layout: 'fullscreen'
	},
	argTypes: {
		onLogin: { action: 'onLogin' },
		onLogout: { action: 'onLogout' },
		onCreateAccount: { action: 'onCreateAccount' }
	}
};

const Template = (/** @type {{ onLogin: any; onLogout: any; onCreateAccount: any; }} */ args) => ({
	Component: NavbarExample,
	props: args,
	on: {
		login: args.onLogin,
		logout: args.onLogout,
		createAccount: args.onCreateAccount
	}
});

export const Default = Template.bind({});
// @ts-ignore
Default.args = {};

// export const LoggedIn = Template.bind({});
// LoggedIn.args = {
// 	user: {
// 		name: 'Jane Doe'
// 	}
// };

// export const LoggedOut = Template.bind({});
// LoggedOut.args = {};
