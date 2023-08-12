<script lang="ts">
  import { cellId, type Lane, type DropTargetStatus } from '../Grid';
  import EmptyCell from './EmptyCell.svelte';
  import Interface from './Interface.svelte';
  import { createEventDispatcher } from 'svelte';
  import Cell from './Cell.svelte';
  import type { DragEventHandler } from 'svelte/elements';
  import { DEFAULT_LANE } from './util';

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  };

  export let max_column: number; // TODO: remove this

  export let audience: Lane;

  $: row = audience.row;
  $: lane_index = audience.index;
  $: gridRow = row + 1;

  export let targeted_lane: DropTargetStatus | undefined = undefined;

  export let targeted_cell: { column: number; targetStatus: DropTargetStatus } | undefined =
    undefined;

  $: audience.cells.length = max_column;
  $: good_target = targeted_lane == 'good';
  $: bad_target = targeted_lane == 'bad';

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e: DragEvent) => {
    let transfer = e.dataTransfer;

    if (transfer) {
      transfer.effectAllowed = 'move';
    }

    dispatch('lane_drag_start', {
      laneId: audience.id,
      laneKind: 'audience'
    });
  };

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();

    dispatch('lane_drag_enter', {
      audienceId: audience.id,
      laneIndex: lane_index,
      rowKind: 'audience'
    });
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (_e) => {
    dispatch('lane_drag_drop');
  };
</script>

<h3
  on:dragstart={handleDragStart}
  draggable="true"
  class="audienceName z-[-1] sticky left-3 justify-self-start self-start cursor-pointer prose text-body-light dark:text-body-dark mt-3 select-none cursor-move"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
>
  {audience.name}
</h3>

<div
  on:dragenter={handleDragEnter}
  on:dragover={(e) => {
    e.preventDefault();
  }}
  on:dragleave={handleDragLeave}
  on:drop={handleDragDrop}
  id={audience.id}
  class:bg-emerald-200={good_target}
  class:bg-rose-400={bad_target}
  class="audience -ml-5 h-full w-full border-t border-gray-primary dark:border-gray-brand-1"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
/>

{#each audience.cells as cell, column (cellId(column, row))}
  {@const drop_target =
    targeted_cell && targeted_cell.column === column ? targeted_cell.targetStatus : undefined}
  <Cell
    {column}
    row={{
      rowIndex: row,
      audienceId: audience.id || DEFAULT_LANE,
      laneIndex: lane_index,
      rowKind: 'audience'
    }}
    maybe_placement={cell.placement?.id}
    maybe_placement_kind="interface"
    target_status={drop_target}
    on:navigate_cursor={forward}
    on:cell_drag_enter={forward}
    on:cell_drag_drop={forward}
    on:open_context_menu={forward}
  >
    {#if cell.placement}
      <Interface
        id={cell.placement.id}
        interface_id={cell.placement.component_id}
        {column}
        name={cell.placement.name}
        description={cell.placement.description}
        config={cell.placement.interface_config || { kind: 'blank' }}
        on:placement_drag_start={forward}
        on:connect_flow={forward}
        on:flow_drag_start={forward}
      />
    {:else}
      <EmptyCell
        {column}
        kind="interface"
        lane={audience.id}
        on:move_interface_placement={forward}
        on:duplicate_interface_placement={forward}
      />
    {/if}
  </Cell>
{/each}
