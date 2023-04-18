<script lang="ts">
  import {placementOrEmptyCellId, type Stream} from "../Grid";
  import EmptyCell from "./EmptyCell.svelte";
  import Event from "./Event.svelte";
  import { createEventDispatcher } from 'svelte';
  import Cell from "./Cell.svelte";
  import type { DragEventHandler } from "svelte/elements";

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail)
  }

  export let row: number;
  $: gridRow = row + 1;

  export let max_column: number;

  export let stream: Stream;

  $: stream.placements.length = max_column

    const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    console.warn("Stream Drag Start", e);

    if (transfer && stream.id) {
      console.warn(`Setting transfer with stream ${stream.id}`);
      transfer.setData('lane', stream.id);
      transfer.effectAllowed = 'move';
    }
  }
</script>

{#if stream.name}
  <h3
    on:dragstart={handleDragStart}
    draggable=true
    class="streamName laneName sticky left-3 z-30 justify-self-start self-end cursor-pointer prose text-body-light nndark:text-body-dark mb-3 cursor-move"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};">
    {stream.name}
  </h3>
{:else}
  <!-- TODO: reduce the color to disabled text -->
  <h3
    class="streamName sticky left-3 z-30 justify-self-start self-end prose text-body-light dark:text-body-dark mb-3 select-none"
    style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};" >
    Default Stream
  </h3>
{/if}

<div
  on:dragstart={handleDragStart}
  draggable=true
  id={stream.id}
  class="stream absolute top-0 -left-3 bottom-0 -right-6 border-b border-gray-primary dark:border-gray-brand-3 cursor-move"
  style="grid-column: 1 / -1; grid-row: {gridRow} / {gridRow};" />

{#each stream.placements as placement, column (placementOrEmptyCellId(placement, column, row))}
  <Cell {row} {column} on:navigate_cursor={forward}>
  {#if placement?.id}
    <Event
      id={placement.id}
      event={placement.event}
      {column}
      name={placement.name}
      description={placement.description}
      on:connect_flow={forward} />
  {:else}
    <EmptyCell {column} kind='event' lane={stream.id}
               on:move_event_placement={forward}
               on:duplicate_event_placement={forward} />
  {/if}
  </Cell>
{/each}
