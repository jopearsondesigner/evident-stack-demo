<script lang="ts">
  import { type DropTargetStatus, placementOrEmptyCellId, type TimelinePlacement } from '../Grid';
  import Command from './Command.svelte';
  import EmptyCell from './EmptyCell.svelte';
  import ReadModel from './ReadModel.svelte';
  import { createEventDispatcher } from 'svelte';
  import Cell from './Cell.svelte';

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  };

  export let row: number;
  $: gridRow = row + 1;

  export let max_column: number;

  export let placements: Array<TimelinePlacement>;

  export let targeted_lane: DropTargetStatus | undefined = undefined;

  export let targeted_cell: { column: number; targetStatus: DropTargetStatus } | undefined =
    undefined;

  $: placements.length = max_column;
  $: good_target = targeted_lane == 'good';
  $: bad_target = targeted_lane == 'bad';
</script>

<h3
  class="timelineName sticky left-3 justify-self-start self-center prose text-body-light dark:text-body-dark"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
>
  Timeline
</h3>

<div
  class="timeline z-[-1] absolute -top-px -left-3 bottom-0 -right-6 border-t border-b border-gray-brand-2 dark:border-white"
  class:bg-emerald-200={good_target}
  class:bg-rose-400={bad_target}
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
/>

{#each placements as placement, column (placementOrEmptyCellId(placement, column, row))}
  {@const drop_target =
    targeted_cell && targeted_cell.column === column ? targeted_cell.targetStatus : undefined}
  <Cell
    {column}
    row={{
      rowIndex: row,
      rowKind: 'timeline'
    }}
    maybe_placement={placement?.id}
    maybe_placement_kind={placement?.kind}
    target_status={drop_target}
    on:navigate_cursor={forward}
    on:cell_drag_enter={forward}
    on:cell_drag_drop={forward}
  >
    {#if placement && placement.kind === 'command'}
      <Command
        id={placement.id}
        command={placement.component}
        {column}
        name={placement.name}
        description={placement.description}
        on:placement_drag_start={forward}
        on:connect_flow={forward}
        on:flow_drag_start={forward}
      />
    {:else if placement && placement.kind === 'readModel'}
      <ReadModel
        id={placement.id}
        readModel={placement.component}
        {column}
        name={placement.name}
        description={placement.description}
        on:placement_drag_start={forward}
        on:connect_flow={forward}
        on:flow_drag_start={forward}
      />
    {:else}
      <EmptyCell
        {column}
        kind="timeline"
        on:move_timeline_placement={forward}
        on:duplicate_timeline_placement={forward}
      />
    {/if}
  </Cell>
{/each}
