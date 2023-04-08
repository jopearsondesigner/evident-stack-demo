<script lang="ts">
  import MaybeTooltip from '../../utils/MaybeTooltip.svelte';
  import markdown from '../../utils/markdown.js';
  import { createEventDispatcher } from 'svelte';
  import type { MouseEventHandler } from 'svelte/elements';

  export let id: string;
  export let event: string;
  export let name: string;
  export let description = '';
  export let row: number;
  export let column: number;

  $: descriptionHTML = markdown(description);
  $: gridRow = row + 1;
  $: gridColumn = column + 1;

  const dispatch = createEventDispatcher();
  const handleClick: MouseEventHandler<HTMLDivElement> = (_event) => {
    dispatch('navigateCursor', { row, column });
  };
</script>

<div
  {id}
  on:click|preventDefault|stopPropagation={handleClick}
  class="placement event z-20 flex place-self-center align-items-center col-border ring-white dark:ring-gray-brand-1 p-[1.375rem] -ml-px mb-px hover:bg-focus/[.18] transition duration-200 ease-in"
  style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
>
  <MaybeTooltip tip={descriptionHTML}>
    <div
      class="event w-[6.125rem] h-[6.125rem] p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-event-dark via-event to-event-light"
    >
      {name}
    </div>
  </MaybeTooltip>
</div>

<style>
  /* Grid column */
  .col-border {
    box-shadow: 1px 0px 0px var(--tw-ring-color) inset;
  }
</style>
