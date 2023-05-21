<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { DragEventHandler, MouseEventHandler } from 'svelte/elements';
  import type { DropTargetStatus, PlacementType } from '../Grid';
  import type { RowTarget, WithRowIndex } from './util';

  export let column: number;
  export let row: RowTarget & WithRowIndex;
  export let maybe_placement: string | undefined;
  export let maybe_placement_kind: PlacementType | undefined;
  export let target_status: DropTargetStatus | undefined;

  $: gridRow = row.rowIndex + 1;
  $: gridColumn = column + 1;
  $: good_target = target_status == 'good';
  $: bad_target = target_status == 'bad';

  const dispatch = createEventDispatcher();
  const handleClick: MouseEventHandler<HTMLDivElement> = (_event) => {
    dispatch('navigate_cursor', { row: row.rowIndex, column });
  };

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();
    dispatch('cell_drag_enter', {
      column,
      row,
      placementId: maybe_placement,
      placementKind: maybe_placement_kind
    });
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
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
