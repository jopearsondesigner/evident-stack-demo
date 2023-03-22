<script lang="ts">
	import classNames from 'classnames';
	import { createEventDispatcher } from 'svelte';
	export let gradient: boolean = false;
	export let size: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
	export let href: string | undefined = undefined;
	export let btnClass: string | undefined = undefined;
	export let label = '';
	export let color:
		| 'default'
		| 'primary'
		| 'secondary'
		| 'warning'
		| 'success'
		| 'disabled'
		| 'brandDesignPrimary'
		| 'brandStackPrimary';

	const colorClasses = {
		default: 'text-white bg-focus hover:bg-[#054FDE] ring-focus',
		secondary: 'text-focus bg-focus/0 hover:bg-focus/100 hover:text-white ring-focus',
		warning: 'text-white bg-focus/0 hover:bg-focus/20 ring-red',
		success: 'text-white bg-focus/0 hover:bg-focus/20 ring-green'
	};
	const gradientClasses = {
		primary:
			'text-white bg-gradient-to-b from-focus to-brand-primary hover:from-focus hover:to-focus ring-focus',
		brandDesignPrimary:
			'text-white bg-gradient-to-t from-design-brand-700 via-design-brand-500 to-design-brand-300 hover:to-design-brand-700 hover:via-design-brand-700 hover:from-design-brand-700 ring-design-brand-500',
		brandStackPrimary:
			'text-white bg-gradient-to-t from-stack-brand-700 via-stack-brand-500 to-stack-brand-300 hover:to-stack-brand-700 hover:via-stack-brand-700 hover:from-stack-brand-700 ring-stack-brand-500'
	};
	const sizeClasses = {
		xs: 'px-3 py-1 text-xs',
		sm: 'px-4 py-1.5 text-xs',
		md: 'px-4 py-2.5 text-sm',
		lg: 'px-4 py-2 text-base',
		xl: 'px-5 py-3 text-base'
	};
	let buttonClass: string;
	$: buttonClass = btnClass
		? btnClass
		: classNames(
				'font-primary font-medium rounded uppercase ring-1 h-auto min-h-0 transition duration-200 ease-in',
				// @ts-ignore
				gradient ? gradientClasses[color] : colorClasses[color],
				sizeClasses[size],
				color,
				$$props.disabled && 'cursor-not-allowed bg-gray-primary ring-gray-primary',
				$$props.class
		  );
	const dispatch = createEventDispatcher();
	function handleClick(event: any) {
		dispatch('click', event);
	}
</script>

<svelte:element
	this={href ? 'a' : 'button'}
	{href}
	{...$$restProps}
	class={buttonClass}
	on:click={handleClick}
>
	<slot name="label">
		{label || 'Button'}
	</slot>
	<slot name="icon" />
</svelte:element>
