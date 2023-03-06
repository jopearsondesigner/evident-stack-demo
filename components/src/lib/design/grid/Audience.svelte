<script>
	/** @type {number} */
	export let row = 0;
	$: gridRow = row + 1;

	/** @type {string} */
	export let title = '';

	$: cursorClass = title ? 'cursor-pointer' : 'cursor-default';
</script>

{#if title}
	<h3
		class="audienceTitle sticky left-3 z-30 justify-self-start self-start cursor-pointer prose text-body-light dark:text-body-dark mt-3"
		style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
	>
		{title}
	</h3>
{:else}
	<!-- TODO: reduce the color to disabled text -->
	<h3
		class="audienceTitle sticky left-3 z-30 justify-self-start self-start prose text-body-light dark:text-body-dark mt-3"
		style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
	>
		Default Audience
	</h3>
{/if}

<div
	class="audience absolute -top-0.5 -left-3 bottom-0.5 -right-6 border-t-2 border-gray-primary dark:border-gray-brand-3"
	style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
/>

<slot />
