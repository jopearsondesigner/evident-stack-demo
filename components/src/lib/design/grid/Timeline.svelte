<script lang="ts">
  import { type DropTargetStatus, cellId, type Lane } from '../Grid';
  import Command from './Command.svelte';
  import EmptyCell from './EmptyCell.svelte';
  import ReadModel from './ReadModel.svelte';
  import { createEventDispatcher } from 'svelte';
  import Cell from './Cell.svelte';

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  };

  export let timeline: Lane;

  $: row = timeline.row;
  $: gridRow = row + 1;

  export let max_column: number;

  export let targeted_lane: DropTargetStatus | undefined = undefined;

  export let targeted_cell: { column: number; targetStatus: DropTargetStatus } | undefined =
    undefined;

  $: timeline.cells.length = max_column;
  $: good_target = targeted_lane == 'good';
  $: bad_target = targeted_lane == 'bad';
</script>

<h3
  class="timelineName sticky left-3 justify-self-start self-center prose text-body-light dark:text-body-dark"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};">
  Timeline
</h3>

<div
  class="timeline -ml-5 h-full w-full border-t border-b border-gray-brand-2 dark:border-white"
  class:bg-emerald-200={good_target}
  class:bg-rose-400={bad_target}
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"/>

{#each timeline.cells as cell, column (cellId(column, row))}
  {@const drop_target =
    targeted_cell && targeted_cell.column === column ? targeted_cell.targetStatus : undefined}
  <Cell
    {column}
    row={{
      rowIndex: row,
      rowKind: 'timeline'
    }}
    maybe_placement={cell.placement?.id}
    maybe_placement_kind={cell.placement?.kind}
    target_status={drop_target}
    on:navigate_cursor={forward}
    on:cell_drag_enter={forward}
    on:cell_drag_drop={forward}
    on:open_context_menu={forward}>
    {#if cell.placement && cell.placement.kind === 'command'}
      <Command
        id={cell.placement.id}
        command={cell.placement.component_id}
        {column}
        name={cell.placement.name}
        description={cell.placement.description}
        on:placement_drag_start={forward}
        on:connect_flow={forward}
        on:flow_drag_start={forward} />
    {:else if cell.placement && cell.placement.kind === 'read_model'}
      <ReadModel
        id={cell.placement.id}
        read_model={cell.placement.component_id}
        {column}
        name={cell.placement.name}
        description={cell.placement.description}
        on:placement_drag_start={forward}
        on:connect_flow={forward}
        on:flow_drag_start={forward} />
    {:else}
      <EmptyCell
        {column}
        kind="timeline"
        on:move_timeline_placement={forward}
        on:duplicate_timeline_placement={forward} />
    {/if}
  </Cell>
{/each}
