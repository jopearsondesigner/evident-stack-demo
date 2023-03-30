<script>
	/** @type {number} */
	export let row = 0;
	$: gridRow = row + 1;

	/** @type {string | null} */
	export let id = null;

	/** @type {string} */
	export let name = '';

	$: cursorClass = name ? 'cursor-pointer' : 'cursor-default';
</script>

{#if name}
	<h3
		class="audienceName sticky left-3 z-30 justify-self-start self-start cursor-pointer prose text-body-light dark:text-body-dark mt-3"
		style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
	>
		{name}
	</h3>
{:else}
	<!-- TODO: reduce the color to disabled text -->
	<h3
		class="audienceName sticky left-3 z-30 justify-self-start self-start prose text-body-light dark:text-body-dark mt-3"
		style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
	>
		Default Audience
	</h3>
{/if}

<div
	class="audience absolute -top-px -left-3 bottom-0.5 -right-6 border-t border-gray-primary dark:border-gray-brand-3"
	style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
/>

<slot />
