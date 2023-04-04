<script lang="ts">
	export let row: number;
	export let column: number;

	$: gridRow = row + 1;
	$: gridColumn = column + 1;
</script>

<div
	class="empty-cell z-10 self-stretch relative min-w-placementPadded min-h-placementPadded dark:border-gray-brand-1 mb-px hover:bg-focus/[.18] transition duration-200 ease-in"
	style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
/>
