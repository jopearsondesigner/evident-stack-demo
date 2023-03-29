<script>
	import classNames from 'classnames';
	export let size = 20;
	export let name = '';
	export let iconColor = 'fill-current text-body-light';
	export let iconClass = 'transition duration-200 ease-in cursor-default';
	export { iconClass as class };
	$: classes = classNames(iconColor, iconClass);
	/**
	 * @type {any}
	 */
	export let pathName = '';

	export let viewBox = '0 0 20 20';
</script>

<svg
	data-name={name}
	xmlns="http://www.w3.org/2000/svg"
	{viewBox}
	width={size}
	height={size}
	class={classes}
>
	<slot><svelte:component this={pathName} /></slot>
</svg>
