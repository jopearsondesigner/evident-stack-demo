<script lang="ts">
	import classNames from 'classnames';

	let defaultClass: string =
		'font-medium py-2 px-4 text-sm hover:bg-gray-100 dark:hover:bg-gray-600 cursor-default';
	export let btnClass: string = 'cursor-default';
	export { defaultClass as class };
	export let href: string | undefined = undefined;
	export let active: boolean = false;

	let liClass: string;
	$: liClass = classNames(defaultClass, href ? 'block' : 'w-full text-left', $$props.class);
</script>

<li class={liClass} class:active>
	<svelte:element
		this={href ? 'a' : 'button'}
		{href}
		class={btnClass}
		{...$$restProps}
		on:click
		on:change
		on:keydown
		on:keyup
		on:focus
		on:blur
		on:mouseenter
		on:mouseleave
	>
		<slot />
	</svelte:element>
</li>

<style>
	.active {
		background: #1e6aff !important;
		color: white !important;
		cursor: default;
	}
</style>
