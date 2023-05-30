<script lang="ts">
  import { type DropTargetStatus, placementOrEmptyCellId, type Stream } from '../Grid';
  import EmptyCell from './EmptyCell.svelte';
  import Event from './Event.svelte';
  import { createEventDispatcher } from 'svelte';
  import Cell from './Cell.svelte';
  import type { DragEventHandler } from 'svelte/elements';
  import { DEFAULT_LANE } from './util';

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  };

  export let row: number;
  $: gridRow = row + 1;

  export let max_column: number;

  export let stream: Stream;

  export let lane_index: number;

  export let targeted_lane: DropTargetStatus | undefined = undefined;

  export let targeted_cell: { column: number; targetStatus: DropTargetStatus } | undefined =
    undefined;

  $: stream.placements.length = max_column;
  $: good_target = targeted_lane == 'good';
  $: bad_target = targeted_lane == 'bad';

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;

    if (transfer && stream.id) {
      transfer.setData('lane', stream.id);
      transfer.effectAllowed = 'move';

      dispatch('lane_drag_start', {
        laneId: stream.id,
        laneKind: 'stream'
      });
    }
  };

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();

    dispatch('lane_drag_enter', {
      streamId: stream.id,
      laneIndex: lane_index,
      rowKind: 'stream'
    });
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
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
    Default Stream
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
    targeted_cell && targeted_cell.column === column ? targeted_cell.targetStatus : undefined}
  <Cell
    {column}
    row={{
      rowIndex: row,
      streamId: stream.id || DEFAULT_LANE,
      laneIndex: lane_index,
      rowKind: 'stream'
    }}
    maybe_placement={placement?.id}
    maybe_placement_kind="event"
    target_status={drop_target}
    on:navigate_cursor={forward}
    on:cell_drag_enter={forward}
    on:cell_drag_drop={forward}
    on:open_context_menu={forward}
  >
    {#if placement?.id}
      <Event
        id={placement.id}
        event={placement.event}
        {column}
        name={placement.name}
        description={placement.description}
        on:placement_drag_start={forward}
        on:flow_drag_start={forward}
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
