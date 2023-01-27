<script>
	import classNames from 'classnames';
	export let navClass = 'inline-flex space-x-3 mx-3 items-center';
</script>

<nav class={classNames(navClass)}>
	<slot />
</nav>
