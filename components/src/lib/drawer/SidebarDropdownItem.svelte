<script>
	import classNames from 'classnames';
	import './drawer.css';
	export let label = '';
	export let href = '';
	export let aLink = false;
	export let featureTextClass = 'font-medium text-default text-body';
	export let sidebarItemClass = 'flex items-center pl-4 pr-1 transition duration-200 ease-in';
	export let sidebarBtnClass =
		'bg-white hover:bg-focus/[.20] focus:text-white focus:bg-focus/[100] transition duration-200 ease-in w-full space-x-3 h-8 pointer-default';
	export let sidebarLinkClass =
		'bg-white hover:bg-focus/[.20] focus:text-white focus:bg-focus/[100] transition duration-200 ease-in w-full space-x-3 h-8 pointer-default';
	export let sidebarSpanClass = 'flex-1 ml-3 text-left whitespace-nowrap';
</script>

<li class="sidebarDropdownWrapper bg-white">
	{#if !aLink}
		<button class={classNames(sidebarItemClass, sidebarBtnClass)}>
			<slot name="icon" />
			<span class={classNames(featureTextClass, sidebarSpanClass)}>{label}</span>
		</button>
	{:else if aLink}
		<a {href} class={classNames(sidebarItemClass, sidebarLinkClass)}>
			<slot name="icon" />
			<span class={classNames(featureTextClass, sidebarSpanClass)}>{label}</span>
		</a>
	{/if}
</li>
