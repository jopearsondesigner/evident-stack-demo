<script lang="ts">
  import { placementOrEmptyCellId, type Audience } from '../Grid';
  import EmptyCell from './EmptyCell.svelte';
  import Interface from './Interface.svelte';
  import { createEventDispatcher } from 'svelte';
  import Cell from './Cell.svelte';

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  };

  export let row: number;
  $: gridRow = row + 1;

  export let max_column: number;

  export let audience: Audience;

  $: audience.placements.length = max_column;
</script>

{#if audience.name}
  <h3
    class="audienceName sticky left-3 justify-self-start self-start cursor-pointer prose text-body-light dark:text-body-dark mt-3"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
  >
    {audience.name}
  </h3>
{:else}
  <!-- TODO: reduce the color to disabled text -->
  <h3
    class="audienceName sticky left-3 justify-self-start self-start prose text-body-light dark:text-body-dark mt-3"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
  >
    Default Audience
  </h3>
{/if}

<div
  id={audience.id}
  class="audience z-[-1] absolute -top-px -left-3 bottom-0.5 -right-6 border-t border-gray-primary dark:border-gray-brand-3"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
/>

{#each audience.placements as placement, column (placementOrEmptyCellId(placement, column, row))}
  <Cell {row} {column} on:navigate_cursor={forward}>
    {#if placement?.id}
      <Interface
        id={placement.id}
        interface_id={placement.interface}
        {column}
        name={placement.name}
        description={placement.description}
        on:connect_flow={forward}
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
