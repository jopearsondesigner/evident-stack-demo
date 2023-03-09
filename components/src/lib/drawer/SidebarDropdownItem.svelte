<script>
	import classNames from 'classnames';
	export let label = '';
	export let href = '';
	export let aLink = false;
	export let featureTextClass = 'font-medium text-default text-body dark:text-white';
	export let sidebarItemClass = 'flex items-center pl-4 pr-1 transition duration-200 ease-in';
	export let sidebarBtnClass =
		'bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] focus:text-body focus:bg-focus/[.20] dark:focus:bg-focus/[.20] transition duration-200 ease-in w-full space-x-3 h-8 pointer-default';
	export let sidebarActiveClass =
		'bg-focus/[100] text-white dark:bg-focus/[100] hover:bg-focus/[100] dark:hover:bg-focus/[100] focus:bg-focus/[100] dark:focus:bg-focus/[100] transition duration-200 ease-in w-full space-x-3 h-8 pointer-default';
	export let sidebarLinkClass =
		'bg-white dark:bg-dark-2 hover:bg-focus/[.20] dark:hover:bg-focus/[.20] focus:text-body focus:bg-focus/[.20] dark:focus:bg-focus/[.20] transition duration-200 ease-in w-full space-x-3 h-8 pointer-default';
	export let sidebarSpanClass = 'flex-1 ml-3 text-left whitespace-nowrap';
	export let active = false;
</script>

<li class="sidebarDropdownWrapper bg-white dark:bg-dark-2">
	{#if !aLink}
		{#if active}
			<button class={classNames(sidebarItemClass, sidebarActiveClass)}>
				<slot name="icon" />
				<span style="color: white;" class={classNames(featureTextClass, sidebarSpanClass)}
					>{label}</span
				>
			</button>
		{:else}
			<button class={classNames(sidebarItemClass, sidebarBtnClass)}>
				<slot name="icon" />
				<span class={classNames(featureTextClass, sidebarSpanClass)}>{label}</span>
			</button>
		{/if}
	{:else if aLink}
		<a {href} class={classNames(sidebarItemClass, sidebarLinkClass)}>
			<slot name="icon" />
			<span class={classNames(featureTextClass, sidebarSpanClass)}>{label}</span>
		</a>
	{/if}
</li>
