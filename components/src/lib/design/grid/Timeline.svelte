<script lang="ts">
  import { placementOrEmptyCellId, type TimelinePlacement } from "../Grid";
  import Command from "./Command.svelte";
  import EmptyCell from "./EmptyCell.svelte";
  import ReadModel from "./ReadModel.svelte";
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail)
  }

  export let row: number;
  $: gridRow = row + 1;

  export let max_column: number;

  export let placements: Array<TimelinePlacement>;

  $: placements.length = max_column
</script>

<h3
  class="timelineName sticky left-3 z-30 justify-self-start self-center prose text-body-light dark:text-body-dark"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};" >
  Timeline
</h3>

<div
  class="timeline absolute -top-px -left-3 bottom-0 -right-6 border-t border-b border-gray-brand-2 dark:border-white"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};" />

{#each placements as placement, column (placementOrEmptyCellId(placement, column, row))}
  {#if placement && placement.kind === 'command'}
    <Command
      on:navigateCursor={forward}
      id={placement.id}
      command={placement.component}
      name={placement.name}
      description={placement.description}
      {row}
      {column} />
  {:else if placement && placement.kind === 'readModel'}
    <ReadModel
      on:navigateCursor={forward}
      id={placement.id}
      readModel={placement.component}
      name={placement.name}
      description={placement.description}
      {row}
      {column} />
  {:else}
    <EmptyCell on:navigateCursor={forward} {row} {column} />
  {/if}
{/each}
