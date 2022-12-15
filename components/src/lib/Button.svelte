<script lang="ts">
	import classNames from 'classnames';
	import {createEventDispatcher} from 'svelte';
	import type { ButtonType } from '$lib/types';
	export let gradient: boolean = false;
	export let size: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
	export let href: string | undefined = undefined;
	export let btnClass: string | undefined = undefined;
	export let type: ButtonType = 'button';
	export let label = '';
	export let color:
		| 'default'
		| 'primary'
		| 'secondary'
		| 'warning'
		| 'success'
		| 'disabled'
		| 'brandPrimary'
		| 'brandSecondary';

	const colorClasses = {
		default: 'text-white bg-active hover:bg-[#054FDE] ring-active',
		secondary:
			'text-active bg-active/0 hover:bg-active/100 hover:text-white ring-active',
		warning:
		'text-white bg-active/0 hover:bg-active/20 ring-red',
		success:
		'text-white bg-active/0 hover:bg-active/20 ring-green',
	};
	const gradientClasses = {
		primary: 'text-white bg-gradient-to-b from-active to-brand-primary hover:from-active hover:to-active ring-active',
		brandPrimary:
			'text-white bg-gradient-to-b from-brand-primary to-[#248CAD] hover:from-[#248CAD] hover:to-[#248CAD] ring-brand-primary',
		brandSecondary:
			'text-white bg-gradient-to-b from-brand-secondary to-[#E15B2F] hover:to-[#E15B2F] hover:from-[#E15B2F] ring-brand-secondary',
	};
	const sizeClasses = {
		xs: 'px-3 py-2 text-xs',
		sm: 'px-4 py-2 text-xs',
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
	type={href ? undefined : type}
	{href}
	{...$$restProps}
	class={buttonClass}
	on:click={handleClick}
	on:change
	on:keydown
	on:keyup
	on:mouseenter
	on:mouseleave>
	<slot name='label'>
		{label || 'Button'}
	</slot>
	<slot name="icon"/>
</svelte:element>
