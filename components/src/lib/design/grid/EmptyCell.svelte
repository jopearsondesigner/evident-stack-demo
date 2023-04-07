<script lang="ts">
  export let row: number;
  export let column: number;
  import { createEventDispatcher } from 'svelte';
  import type { MouseEventHandler } from 'svelte/elements';

  $: gridRow = row + 1;
  $: gridColumn = column + 1;

  const dispatch = createEventDispatcher();
  const handleClick: MouseEventHandler<HTMLDivElement> = (_event) => {
    dispatch('navigateCursor', { row, column });
  };
</script>

<div
  on:click|preventDefault|stopPropagation={handleClick}
  class="empty-cell z-20 self-stretch relative min-w-placementPadded min-h-placementPadded col-border ring-white -ml-px mb-px hover:bg-focus/[.18] transition duration-200 ease-in"
  style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
/>

<style>
  /* Grid column */
  .col-border {
    box-shadow: 1px 0px 0px var(--tw-ring-color) inset;
  }
</style>
