<svelte:options immutable />

<script>
	import Audience from './grid/Audience.svelte';
	import Timeline from './grid/Timeline.svelte';
	import Stream from './grid/Stream.svelte';

	import Interface from './grid/Interface.svelte';
	import Command from './grid/Command.svelte';
	import Event from './grid/Event.svelte';
	import ReadModel from './grid/ReadModel.svelte';
	import EmptyCell from './grid/EmptyCell.svelte';
	import {
		maxSparseArrayIndex,
		maxSparseArrayIndexInArray,
		setAllPlacementArrayLengths,
		setArrayLength
	} from './grid';

	/** @type Array.<{id: string,
      title: string,
      description: string}> */
	export let default_audience_placements = new Array(0);

	/** @type Array.<{title: string,
      placements: Array.<{id: string,
      type: string, // TODO: supported placement types/config here
      title: string,
      description: string}>}> */
	export let audiences = new Array(0);

	/** @type Array<{id: string,
      type: ('command' | 'readModel'),
      title: string,
      description: string}> */
	export let timeline_placements = new Array(0);

	/** @type Array.<{title: string,
      placements: Array.<{id: string,
      title: string,
      description: string}>}> */
	export let streams = new Array(0);

	/** @type Array.<{id: string,
      title: string,
      description: string}> */
	export let default_stream_placements = new Array(0);

	// Columns

	const right_buffer = 10;

	$: max_column =
		Math.max(
			maxSparseArrayIndex(default_audience_placements),
			maxSparseArrayIndexInArray(audiences.map((a) => a.placements)),
			maxSparseArrayIndex(timeline_placements),
			maxSparseArrayIndexInArray(streams.map((s) => s.placements)),
			maxSparseArrayIndex(default_stream_placements)
		) + right_buffer;

	$: setArrayLength(default_audience_placements, max_column);
	$: setAllPlacementArrayLengths(audiences, max_column);
	$: setArrayLength(timeline_placements, max_column);
	$: setAllPlacementArrayLengths(streams, max_column);
	$: setArrayLength(default_stream_placements, max_column);

	// Rows

	const default_audience_row = 0;

	$: timeline_row = audiences.length + 1;

	$: default_stream_row = timeline_row + streams.length + 1;

	$: row_count = default_stream_row + 1;

	// List IDs

	/**
     @param {{id: string} | undefined | null} placement - the placement
     @param {number} col - the column index
     @param {number} row - the row index
     @returns {string} - returns a string ID, either the placement.id or an empty cell id from row-col
  */
	function placementOrEmptyCellId(placement, col, row) {
		return (placement && placement.id) || `empty-${col}-${row}`;
	}

	// Canvas State

	/** @type {{row: number, col: number}} */
	let cursor_position = { row: timeline_row, col: 0 };

	/**
     @param {number} col - the column index
     @param {number} row - the row index
     @returns {boolean} - whether the given col, row is equal to the current cursor position
  */
	function isCursor(col, row) {
		return row == cursor_position.row && col == cursor_position.col;
	}

	/** @type {{row: number, col: number} | null} */
	let editing_position = null;

	/**
     @param {number} col - the column index
     @param {number} row - the row index
     @returns {boolean} - whether the given col, row is equal to the current cursor position
  */
	function isEditing(col, row) {
		return !!editing_position && row == editing_position.row && col == editing_position.col;
	}
</script>

<div
	class="p-3 relative grid justify-items-center items-center w-max bg-gray-canvas dark:bg-dark-1"
	style="grid-template-columns: repeat({max_column}, min-content); grid-template-rows: repeat({row_count}, minmax(108px, min-content));"
>
	<!-- TODO: make the card size a constant someplace -->
	<Audience row={default_audience_row}>
		{#each default_audience_placements as placement, index (placementOrEmptyCellId(placement, index, default_audience_row))}
			{@const is_cursor = isCursor(default_audience_row, index)}
			<!-- {@const is_editing = isEditing(default_audience_row, index)} -->
			{#if placement}
				<Interface
					id={placement.id}
					title={placement.title}
					description={placement.description}
					row={default_audience_row}
					column={index}
					cursor={is_cursor}
				/>
			{:else}
				<EmptyCell row={default_audience_row} column={index} cursor={is_cursor} />
			{/if}
		{/each}
	</Audience>

	{#each audiences as audience, i}
		{@const row = i + 1}
		<Audience {row} title={audience.title}>
			{#each audience.placements as placement, index (placementOrEmptyCellId(placement, index, row))}
				{@const is_cursor = isCursor(row, index)}
				<!-- {@const is_editing = isEditing(row, index)} -->
				{#if placement}
					<Interface
						id={placement.id}
						title={placement.title}
						description={placement.description}
						{row}
						column={index}
						cursor={is_cursor}
					/>
				{:else}
					<EmptyCell {row} column={index} cursor={is_cursor} />
				{/if}
			{/each}
		</Audience>
	{/each}

	<Timeline row={timeline_row}>
		{#each timeline_placements as placement, index (placementOrEmptyCellId(placement, index, timeline_row))}
			{@const is_cursor = isCursor(timeline_row, index)}
			<!-- {@const is_editing = isEditing(timeline_row, index)} -->
			{#if placement && placement.type === 'command'}
				<Command
					id={placement.id}
					title={placement.title}
					description={placement.description}
					row={timeline_row}
					column={index}
					cursor={is_cursor}
				/>
			{:else if placement && placement.type === 'readModel'}
				<ReadModel
					id={placement.id}
					title={placement.title}
					description={placement.description}
					row={timeline_row}
					column={index}
					cursor={is_cursor}
				/>
			{:else}
				<EmptyCell row={timeline_row} column={index} cursor={is_cursor} />
			{/if}
		{/each}
	</Timeline>

	{#each streams as stream, i}
		{@const row = i + timeline_row + 1}
		<Stream {row} title={stream.title}>
			{#each stream.placements as placement, index (placementOrEmptyCellId(placement, index, row))}
				{@const is_cursor = isCursor(row, index)}
				<!-- {@const is_editing = isEditing(row, index)} -->
				{#if placement}
					<Event
						id={placement.id}
						title={placement.title}
						description={placement.description}
						{row}
						column={index}
						cursor={is_cursor}
					/>
				{:else}
					<EmptyCell {row} column={index} cursor={is_cursor} />
				{/if}
			{/each}
		</Stream>
	{/each}

	<Stream row={default_stream_row}>
		{#each default_stream_placements as placement, index (placementOrEmptyCellId(placement, index, default_stream_row))}
			{@const is_cursor = isCursor(default_stream_row, index)}
			<!-- {@const is_editing = isEditing(default_stream_row, index)} -->
			{#if placement}
				<Event
					id={placement.id}
					title={placement.title}
					description={placement.description}
					row={default_stream_row}
					column={index}
					cursor={is_cursor}
				/>
			{:else}
				<EmptyCell row={default_stream_row} column={index} cursor={is_cursor} />
			{/if}
		{/each}
	</Stream>
</div>
