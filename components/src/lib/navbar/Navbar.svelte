<script>
	import classNames from 'classnames';
	import './navbar.css';
	export let navbarClass = 'relative shadow-header bg-white w-full max-h-12 px-3 py-1.5 text-body';
</script>

<header class={classNames(navbarClass)}>
	<slot />
</header>
