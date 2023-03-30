<script lang="ts">
	import MaybeTooltip from '../../utils/MaybeTooltip.svelte';
	import markdown from '../../utils/markdown.js';

	export let id: string; // TODO: UUID

	export let event: string; // TODO: UUID

	export let name: string;

	export let description = '';

	export let row: number;

	export let column: number;

	export let cursor = false;

	$: descriptionHTML = markdown(description);
	$: gridRow = row + 1;
	$: gridColumn = column + 1;
</script>

<div
	{id}
	class="placement z-20 flex place-self-center align-items-center p-[1.375rem] border-r border-white dark:border-gray-brand-1 -ml-px mb-px hover:bg-focus/[.18] transition duration-200 ease-in"
	class:cursor
	style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
>
	<MaybeTooltip tip={descriptionHTML}>
		<div
			class="event w-[6.125rem] h-[6.125rem] p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-event-dark via-event to-event-light"
		>
			{name}
		</div>
	</MaybeTooltip>
</div>
