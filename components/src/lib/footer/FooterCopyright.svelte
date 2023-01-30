<script lang="ts">
	import classNames from 'classnames';
	export let spanClass: string = 'text-center text-xs text-body-light dark:text-body-dark mt-4';
	export let aClass: string = 'hover:underline';
	export let year: number = new Date().getFullYear();
	export let href: string = '';
	export let by: string = '';
	export let target: string | undefined = undefined;
</script>

<span class={classNames(spanClass, $$props.class)}>
	Copyright &copy; {year}
	{#if href}
		<a {...$$restProps} {href} {target} class={aClass}>
			{by}
		</a>
	{:else}
		<span class="ml-1">{by}</span>
	{/if}
	. All Rights Reserved.
</span>
