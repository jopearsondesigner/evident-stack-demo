<script>
	import classNames from 'classnames';
	export let items = [];
	export let value;
	export let placeholder = 'Choose option ...';
	export let underline = false;
	export let size = 'md';
	export let defaultClass =
		'text-body bg-black/[.04] border border-border-light dark:border-border-dark rounded focus:border focus:ring-focus focus:border-focus focus-visible:ring-focus focus-visible:ring-2 dark:bg-black/[.04] dark:border-border-dark dark:placeholder-gray-400 dark:text-white dark:focus:ring-focus dark:focus:border-focus';
	export let underlineClass =
		'text-gray-500 bg-transparent border-0 border-b-2 border-gray-200 appearance-none dark:text-gray-400 dark:border-gray-700 focus:outline-none focus:ring-0 focus:border-gray-200 peer';
	import Arrow from '../assets/images/global/ArrowDown.svg';
	const common = 'block w-full';
	const sizes = {
		sm: 'text-sm p-2',
		md: 'text-sm py-2.5 pl-2.5 pr-10',
		lg: 'text-base py-3 px-4'
	};
	let selectClass;
	$: selectClass = classNames(
		common,
		underline ? underlineClass : defaultClass,
		sizes[size],
		underline && '!px-0',
		$$restProps.class
	);
</script>

<select {...$$restProps} bind:value class={selectClass} on:change on:contextmenu on:input>
	{#if placeholder}
		<option disabled selected value="">{placeholder}</option>
	{/if}

	{#each items as { value, name }}
		<option {value}>{name}</option>
	{:else}
		<slot />
	{/each}
</select>

<style>
	select {
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
		background-image: url('../assets/images/global/ArrowDown.svg');
		background-repeat: no-repeat;
		background-size: 16px;
		background-position: 98% center;
	}
	option {
		background-color: #f5f5f5;
	}
</style>
