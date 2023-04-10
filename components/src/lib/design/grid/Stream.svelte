<script lang="ts">
  import {placementOrEmptyCellId, type Stream} from "../Grid";
  import EmptyCell from "./EmptyCell.svelte";
  import Event from "./Event.svelte";
  import { createEventDispatcher } from 'svelte';
  import Cell from "./Cell.svelte";

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail)
  }

  export let row: number;
  $: gridRow = row + 1;

  export let max_column: number;

  export let stream: Stream;

  $: stream.placements.length = max_column
</script>

{#if stream.name}
  <h3
    class="streamName laneName sticky left-3 z-30 justify-self-start self-end cursor-pointer prose text-body-light nndark:text-body-dark mb-3"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};">
    {stream.name}
  </h3>
{:else}
  <!-- TODO: reduce the color to disabled text -->
  <h3
    class="streamName sticky left-3 z-30 justify-self-start self-end prose text-body-light dark:text-body-dark mb-3"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};" >
    Default Stream
  </h3>
{/if}

<div
  id={stream.id}
  class="stream absolute top-0 -left-3 bottom-0 -right-6 border-b border-gray-primary dark:border-gray-brand-3"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};" />

{#each stream.placements as placement, column (placementOrEmptyCellId(placement, column, row))}
  <Cell {row} {column} on:navigate_cursor={forward}>
  {#if placement}
    <Event
      id={placement.id}
      event={placement.event}
      name={placement.name}
      description={placement.description} />
  {:else}
    <EmptyCell {column} kind='event' lane={stream.id} on:move_event_placement={forward} />
  {/if}
  </Cell>
{/each}
