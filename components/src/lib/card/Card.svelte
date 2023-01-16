<script lang="ts">
	import classNames from 'classnames';
	export let imgClass: string = '';
	export let cardClass: string = '';
	export let href: string | undefined = undefined;
	export let img: string | undefined = undefined;
	export let width: number;
</script>

<svelte:element
	this={href ? 'a' : 'div'}
	{href}
	class={classNames(cardClass, $$props.class)}
	{...$$restProps}
>
	{#if img}
		<img class={imgClass} src={img} alt="" {width} />
		<slot />
	{:else}
		<slot />
	{/if}
</svelte:element>
