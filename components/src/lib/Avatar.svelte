<script>
	import classNames from 'classnames';
	export let avatarWrapperClass = 'avatar online placeholder';
	export let avatarClass = 'w-8 rounded-full';
</script>

<!-- Look at the more complex Avatar component at svelte.dev at a later point -->

<!-- A11y: A form label must be associated with a control. -->

<div class={classNames(avatarWrapperClass)}>
	<div class={classNames(avatarClass)}>
		<img src="https://placeimg.com/80/80/people" alt="Placeholder" />
	</div>
</div>

<style>
	.avatar.online:before {
		box-shadow: none !important;
	}
</style>
