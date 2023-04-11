<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { MouseEventHandler } from 'svelte/elements';

  export let row: number;
  export let column: number;
  $: gridRow = row + 1;
  $: gridColumn = column + 1;

  const dispatch = createEventDispatcher();
  const handleClick: MouseEventHandler<HTMLDivElement> = (_event) => {
    dispatch('navigate_cursor', { row, column });
  };
</script>

<div
  on:click|preventDefault|stopPropagation={handleClick}
  class="cell col-border ring-white dark:ring-gray-brand-1 z-20 flex place-self-center align-items-center hover:bg-focus/[.18] transition duration-200 ease-in border-2 border-transparent"
  style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
>
  <slot />
</div>

<style>
  /* Grid column */
  .col-border {
    box-shadow: 1px 0px 0px 0px var(--tw-ring-color) inset;
  }
</style>
