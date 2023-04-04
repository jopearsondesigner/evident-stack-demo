<script lang="ts">
  import {placementOrEmptyCellId, type Audience} from "../Grid";
  import EmptyCell from "./EmptyCell.svelte";
  import Interface from "./Interface.svelte";

  export let row: number;
  $: gridRow = row + 1;

  export let max_column: number;

  export let audience: Audience;

  $: audience.placements.length = max_column
</script>

{#if audience.name}
  <h3
    class="audienceName sticky left-3 z-30 justify-self-start self-start cursor-pointer prose text-body-light dark:text-body-dark mt-3"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
    >
    {audience.name}
  </h3>
{:else}
  <!-- TODO: reduce the color to disabled text -->
  <h3
    class="audienceName sticky left-3 z-30 justify-self-start self-start prose text-body-light dark:text-body-dark mt-3"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
    >
    Default Audience
  </h3>
{/if}

<div
  id={audience.id}
  class="audience absolute -top-px -left-3 bottom-0.5 -right-6 border-t border-gray-primary dark:border-gray-brand-3"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};"
  />

{#each audience.placements as placement, column (placementOrEmptyCellId(placement, column, row))}
  {#if placement}
    <Interface
      id={placement.id}
      interface_id={placement.interface}
      name={placement.name}
      description={placement.description}
      {row}
      {column} />
  {:else}
    <EmptyCell {row} {column} />
  {/if}
{/each}
