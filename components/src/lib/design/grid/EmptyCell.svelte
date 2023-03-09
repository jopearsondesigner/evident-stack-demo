<script>
	/** @type {number} */
	export let row = 0;

	/** @type {number} */
	export let column = 0;

	/** @type {boolean} */
	export let cursor = false;

	$: gridRow = row + 1;
	$: gridColumn = column + 1;
</script>

<div
	class="empty-cell z-10 self-stretch relative min-w-placementPadded min-h-placementPadded border-r border-white dark:border-gray-brand-1 mb-px hover:bg-focus/[.18] transition duration-200 ease-in"
	class:cursor
	style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
/>
