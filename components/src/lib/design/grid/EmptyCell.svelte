<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { DragEventHandler, MouseEventHandler } from 'svelte/elements';
  import type { CellType } from '../Grid';

  export let row: number;
  export let column: number;
  export let kind: CellType;
  export let lane: string | undefined = undefined;

  $: gridRow = row + 1;
  $: gridColumn = column + 1;

  const dispatch = createEventDispatcher();

  const handleClick: MouseEventHandler<HTMLDivElement> = (_event) => {
    dispatch('navigate_cursor', {row, column})
  }

  let drop_target = false;

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer && transfer.getData('kind') == kind) {
      e.preventDefault();
      drop_target = true;
    }
  }

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    drop_target = false;
  }

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer && transfer.getData('kind') == kind) {
      let id = transfer.getData('id');
      if (kind == 'interface') {
        dispatch('move_interface_placement', {id: id, index: column, audience: lane})
      } else if (kind == 'timeline') {
        dispatch('move_timeline_placement', {id: id, index: column})
      } else if (kind == 'event') {
        dispatch('move_event_placement', {id: id, index: column, stream: lane})
      }
    }
  }
</script>

<div
  on:click|preventDefault|stopPropagation={handleClick}
  on:dragenter={handleDragEnter}
  on:dragover={(e) => {e.preventDefault()}}
  on:dragleave={handleDragLeave}
  on:drop={handleDragDrop}
  class:bg-pink-200={drop_target}
  class="empty-cell z-10 self-stretch relative min-w-placementPadded min-h-placementPadded dark:border-gray-brand-1 mb-px hover:bg-focus/[.18] transition duration-200 ease-in"
  style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};" />
