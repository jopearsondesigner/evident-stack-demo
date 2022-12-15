<script>
	import classNames from 'classnames';
	export let navbarClass = 'relative shadow-header bg-white w-full max-h-12 px-3 py-1.5 text-body z-10';
</script>

<header class={classNames(navbarClass)}>
	<slot />
</header>
