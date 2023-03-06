<script>
	import MaybeTooltip from '../../utils/MaybeTooltip.svelte';
	import markdown from '../../utils/markdown.js';

	/* TODO: UUID */
	/* @type {string} */
	export let id = '';

	/* @type {string} */
	export let title = '';

	/* @type {string | null} */
	export let description = '';

	/* @type {number} */
	export let row = 0;

	/* @type {number} */
	export let column = 0;

	/* @type {boolean} */
	export let cursor = false;

	$: descriptionHTML = markdown(description);
	$: gridRow = row + 1;
	$: gridColumn = column + 1;
</script>

<div
	{id}
	class="placement z-20 flex place-self-center align-items-center p-[1.375rem] border-r border-white -ml-px mb-0.5 hover:bg-focus/[.18] transition duration-200 ease-in"
	class:cursor
	style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
>
	<MaybeTooltip tip={descriptionHTML}>
		<div
			class="event w-[6.125rem] h-[6.125rem] p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-event-dark via-event to-event-light"
		>
			{title}
		</div>
	</MaybeTooltip>
</div>
