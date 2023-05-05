<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { DragEventHandler, MouseEventHandler } from 'svelte/elements';
  import type { DropTargetStatus, LaneKind } from '../Grid';

  export let row: number;
  export let column: number;
  export let lane_index: number;
  export let lane_kind: LaneKind;
  export let lane_id: string;
  export let drop_target: DropTargetStatus | undefined;

  $: gridRow = row + 1;
  $: gridColumn = column + 1;
  $: good_target = drop_target == 'good';
  $: bad_target = drop_target == 'bad';

  const dispatch = createEventDispatcher();
  const handleClick: MouseEventHandler<HTMLDivElement> = (_event) => {
    dispatch('navigate_cursor', { row, column });
  };

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    console.warn('Cell Drag Enter');
    e.stopPropagation();
    dispatch('cell_drag_enter', {
      column,
      laneIndex: lane_index,
      laneKind: lane_kind,
      laneId: lane_id
    });
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
    console.warn('Cell Drag Drop');
    dispatch('cell_drag_drop');
  };
</script>

<div
  on:click|preventDefault|stopPropagation={handleClick}
  on:dragenter={handleDragEnter}
  on:dragover={(e) => {
    e.preventDefault();
  }}
  on:dragleave={handleDragLeave}
  on:drop={handleDragDrop}
  class="cell z-20 flex place-self-center align-items-center hover:bg-focus/[.18] transition duration-200 ease-in border-2 border-transparent"
  class:bg-emerald-200={good_target}
  class:bg-rose-400={bad_target}
  style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
>
  <slot />
</div>

<style>
  /* Grid column */
  .col-border {
    --tw-ring-offset-width: 0px;
    box-shadow: 1px 0px 0px 0px var(--tw-ring-color);
  }
</style>
