<script>
	import classNames from 'classnames';
	import './navbar.css';
	export let href = '/';
	export let navBrandClass = 'mx-3 flex justify-center cursor-default h-[24px]';
</script>

<a {href} class={classNames(navBrandClass)}>
	<slot name="logo" />
</a>
