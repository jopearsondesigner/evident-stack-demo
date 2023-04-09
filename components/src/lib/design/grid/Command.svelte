<script lang="ts">
  import MaybeTooltip from '../../utils/MaybeTooltip.svelte';
  import markdown from '../../utils/markdown.js';
  import { createEventDispatcher } from 'svelte';
  import type { DragEventHandler, MouseEventHandler, TouchEventHandler } from 'svelte/elements';

  export let id: string;
  export let command: string;
  export let name: string;
  export let description = '';
  export let row: number;
  export let column: number;

  $: descriptionHTML = markdown(description);
  $: gridRow = row + 1;
  $: gridColumn = column + 1;

  const dispatch = createEventDispatcher();
  const handleClick: MouseEventHandler<HTMLDivElement> = (_event) => {
    dispatch('navigate_cursor', {row, column})
  }

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer) {
      transfer.dropEffect = 'move';
      transfer.setData('kind', 'timeline')
      transfer.setData('id', id)
    }
  }
</script>

<div
  on:click|preventDefault|stopPropagation={handleClick}
  class="placement command z-20 flex place-self-center align-items-center p-[1.375rem] dark:border-gray-brand-1 -ml-px mb-px hover:bg-focus/[.18] transition duration-200 ease-in"
  style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
  >
  <MaybeTooltip tip={descriptionHTML}>
    <div
      {id}
      draggable=true
      on:dragstart={handleDragStart}
      class="command w-[6.125rem] h-[6.125rem] p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-command-dark via-command to-command-light" >
      {name}
    </div>
  </MaybeTooltip>
</div>
