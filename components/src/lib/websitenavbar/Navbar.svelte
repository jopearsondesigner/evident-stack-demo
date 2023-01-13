<script lang="ts">
	export let navbarClass = 'shadow-md z-30 relative bg-white dark:bg-dark-2';
	export let navbarWebClass =
		'relative shadow-header bg-white w-full max-h-12 px-3 py-1.5 text-body z-10';
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
