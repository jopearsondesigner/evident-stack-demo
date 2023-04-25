<script lang="ts">
  import { placementOrEmptyCellId, type Stream } from '../Grid';
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

  export let drop_target: 'target' | 'bad-target' | 'none' = 'none';

  export let lane_index: number;

  // export let target_index: number | undefined = undefined;

  // let drop_target: 'target' | 'bad-target' | 'none' = 'none';

  $: stream.placements.length = max_column;
  $: good_target = drop_target == 'target';
  $: bad_target = drop_target == 'bad-target';

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    console.warn('Lane(Stream) Drag Start', e);

    if (transfer && stream.id) {
      // console.warn(`Setting transfer with stream ${stream.id}`);
      transfer.setData('lane', stream.id);
      transfer.effectAllowed = 'move';

      dispatch('lane_drag_start', {
        laneId: stream.id,
        laneType: 'stream'
      });
    }
  };

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    console.warn('Lane(Stream) Enter', e);
    console.warn('TRANSFER DATA', transfer, transfer?.types);

    dispatch('lane_drag_enter', {
      laneIndex: lane_index,
      laneType: 'stream'
    });
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (_e) => {
    console.warn('Lane(Stream) Leave');
    dispatch('lane_drag_leave', {});
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
    console.warn('Lane(Stream) Drop');
    dispatch('lane_drag_drop');
    // handleDragLeave(e);
    // let transfer = e.dataTransfer;
    // let id = transfer?.getData('lane');

    // if (transfer && id && transfer.effectAllowed == 'move') {
    //   console.warn(`GOT A LANE DROP!!! ${id} => index: ${lane_index}`);
    //   dispatch('reorder_lane', { kind: 'stream', lane_id: id, index: lane_index });
    // }
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
  <Cell
    {row}
    {column}
    on:navigate_cursor={forward}
    on:placement_drag_enter={forward}
    on:placement_drag_leave={forward}
    on:placement_drag_drop={forward}
  >
    {#if placement?.id}
      <Event
        id={placement.id}
        event={placement.event}
        {column}
        name={placement.name}
        description={placement.description}
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
