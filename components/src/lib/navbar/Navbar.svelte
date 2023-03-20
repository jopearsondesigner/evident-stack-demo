<script lang="ts">
	export let navbarClass =
		'sticky top-0 shadow-header w-full px-3 py-3.5 text-light dark:text-dark bg-white dark:bg-dark-2 z-50 cursor-default';
	export let navbarWebClass = 'relative shadow-header bg-white w-full px-3 py-3.5 text-body z-50';
	export let website: boolean = false;
</script>

{#if !website}
	<header class={navbarClass}>
		<slot />
	</header>
{:else}
	<header class={navbarWebClass}>
		<slot />
	</header>
{/if}
