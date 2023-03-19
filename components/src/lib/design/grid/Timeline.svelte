<script>
	/** @type {number} */
	export let row = 0;
	$: gridRow = row + 1;
</script>

<h3
	class="timelineTitle sticky left-3 z-30 justify-self-start self-center prose text-body-light dark:text-body-dark"
	style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
>
	Timeline
</h3>

<div
	class="timeline absolute -top-px -left-3 bottom-0 -right-6 border-t border-b border-gray-brand-2 dark:border-white"
	style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
/>

<slot />
