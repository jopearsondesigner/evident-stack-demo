<script lang='ts'>
	import classNames from 'classnames';
	import './navbar.css';
	export let href = '/';
	export let alt: string;
	export let src: string;
	export let navBrandClass = 'mx-3 flex justify-center cursor-default h-[24px]';
</script>

<a {href} class={classNames(navBrandClass)}>
	<img {src} height="22" class={classNames(navBrandClass)} {alt} />
</a>
