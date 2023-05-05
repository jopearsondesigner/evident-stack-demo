<script lang="ts">
  import { DropTargetStatus, placementOrEmptyCellId, type Stream } from '../Grid';
  import EmptyCell from './EmptyCell.svelte';
  import Event from './Event.svelte';
  import { createEventDispatcher } from 'svelte';
  import Cell from './Cell.svelte';
  import type { DragEventHandler } from 'svelte/elements';

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  };

  export let row: number;
  $: gridRow = row + 1;

  export let max_column: number;

  export let stream: Stream;

  export let lane_index: number;

  export let drop_target: DropTargetStatus | undefined = undefined;

  export let cell_drop_target: { column: number; targetStatus: DropTargetStatus } | undefined =
    undefined;

  $: stream.placements.length = max_column;
  $: good_target = drop_target == 'good';
  $: bad_target = drop_target == 'bad';

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    console.warn('Lane(Stream) Drag Start', e);

    if (transfer && stream.id) {
      transfer.setData('lane', stream.id);
      transfer.effectAllowed = 'move';

      dispatch('lane_drag_start', {
        laneId: stream.id,
        laneType: 'stream'
      });
    }
  };

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    console.warn('Lane(Stream) Enter', e);
    e.stopPropagation();

    dispatch('lane_drag_enter', {
      laneIndex: lane_index,
      laneType: 'stream'
    });
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    console.warn('Lane(Stream) Leave');
    e.stopPropagation();
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
    console.warn('Lane(Stream) Drop');
    dispatch('lane_drag_drop');
  };
</script>

{#if stream.name}
  <h3
    on:dragstart={handleDragStart}
    draggable="true"
    class="streamName laneName sticky left-3 z-30 justify-self-start self-end cursor-pointer prose text-body-light nndark:text-body-dark mb-3 cursor-move"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
  >
    {stream.name + lane_index}
  </h3>
{:else}
  <!-- TODO: reduce the color to disabled text -->
  <h3
    class="streamName sticky left-3 z-30 justify-self-start self-end prose text-body-light dark:text-body-dark mb-3 select-none"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
  >
    Default Stream {lane_index}
  </h3>
{/if}

<div
  on:dragenter={handleDragEnter}
  on:dragover={(e) => {
    e.preventDefault();
  }}
  on:dragleave={handleDragLeave}
  on:drop={handleDragDrop}
  class:bg-emerald-200={good_target}
  class:bg-rose-400={bad_target}
  id={stream.id}
  class="stream absolute top-0 -left-3 bottom-0 -right-6 border-b border-gray-primary dark:border-gray-brand-3 cursor-move"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
/>

{#each stream.placements as placement, column (placementOrEmptyCellId(placement, column, row))}
  {@const drop_target =
    cell_drop_target && cell_drop_target.column === column
      ? cell_drop_target.targetStatus
      : undefined}
  <Cell
    {row}
    {column}
    {lane_index}
    {drop_target}
    lane_id={stream.id || ''}
    lane_kind="stream"
    on:navigate_cursor={forward}
    on:cell_drag_enter={forward}
    on:cell_drag_drop={forward}
  >
    {#if placement?.id}
      <Event
        id={placement.id}
        event={placement.event}
        {column}
        name={placement.name}
        description={placement.description}
        on:placement_drag_start={forward}
        on:connect_flow={forward}
      />
    {:else}
      <EmptyCell
        {column}
        kind="event"
        lane={stream.id}
        on:move_event_placement={forward}
        on:duplicate_event_placement={forward}
      />
    {/if}
  </Cell>
{/each}
