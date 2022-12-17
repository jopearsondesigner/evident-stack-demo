<script>
	import classNames from 'classnames';
	export let label = '';
	export let featureData = false;
	export let headerText = false;
	export let headerTextClass='font-extrabold text-default text-body';
	export let featureTextClass='font-medium text-default text-body';
	export let sidebarItemClass='sidebarItem flex items-center pl-4 pr-1 transition duration-200 ease-in';
	export let sidebarBtnClass='bg-white hover:bg-focus/[.20] focus:text-white focus:bg-focus/[100] transition duration-200 ease-in w-full space-x-3 h-8 pointer-default';
	export let sidebarSpanClass = 'flex-1 ml-3 text-left whitespace-nowrap';
</script>

<li class="sidebarItem">
	{#if !featureData}
		<button class={classNames(sidebarItemClass, sidebarBtnClass)}>
			{#if headerText}
				<span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
			{:else}
				<span class={classNames(featureTextClass, sidebarSpanClass)}>{label}</span>
			{/if}
			<slot name="icon" />
		</button>
	{:else}
		<button class={classNames(sidebarItemClass, sidebarBtnClass)}>
			{#if headerText}
				<span class={classNames(headerTextClass, sidebarSpanClass)}>{label}</span>
			{:else}
				<span class={classNames(featureTextClass, sidebarSpanClass)}>{label}</span>
			{/if}
			<slot />
		</button>
	{/if}
</li>

<style>
	.sidebarItem button:focus span,
	.sidebarDropdownWrapper button:focus span {
		color: white !important;
		transition: all 200ms ease-in !important;
	}
</style>
