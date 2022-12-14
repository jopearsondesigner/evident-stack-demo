<script>
	import { Meta, Template, Story } from '@storybook/addon-svelte-csf';
	import Navbar from '../lib/navbar/Navbar.svelte';
</script>

<Meta title="Global/Navbarr" component={Navbar} argTypes={{}} />

<Template let:args>
	<Navbar
		{...args}
		on:login={args.onLogin}
		on:logout={args.onLogout}
		on:createAccount={args.onCreateAccount}
	/>
</Template>

<Story
	name="LoggedIn"
	args={{
		user: {}
	}}
/>

<Story name="LoggedOut" args={{}} />
