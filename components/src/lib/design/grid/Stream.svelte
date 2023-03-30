<script>
	/** @type {number} */
	export let row = 0;
	$: gridRow = row + 1;

  /** @type {string | null} */
	export let id = null;

	/** @type {string} */
	export let name = '';
</script>

{#if name}
	<h3
		class="streamName laneName sticky left-3 z-30 justify-self-start self-end cursor-pointer prose text-body-light dark:text-body-dark mb-3"
		style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
	>
		{name}
	</h3>
{:else}
	<!-- TODO: reduce the color to disabled text -->
	<h3
		class="streamName sticky left-3 z-30 justify-self-start self-end prose text-body-light dark:text-body-dark mb-3"
		style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
	>
		Default Stream
	</h3>
{/if}

<div
	class="stream absolute top-0 -left-3 bottom-0 -right-6 border-b border-gray-primary dark:border-gray-brand-3"
	style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
/>

<slot />
