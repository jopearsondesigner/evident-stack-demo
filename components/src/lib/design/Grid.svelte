<svelte:options immutable />

<script lang="ts">
  import Cursor from "./grid/Cursor.svelte";
  import Audience from './grid/Audience.svelte';
  import Timeline from './grid/Timeline.svelte';
  import Stream from './grid/Stream.svelte';

  import Interface from './grid/Interface.svelte';
  import Command from './grid/Command.svelte';
  import Event from './grid/Event.svelte';
  import ReadModel from './grid/ReadModel.svelte';
  import EmptyCell from './grid/EmptyCell.svelte';
  import {
    maxSparseArrayIndex,
    maxSparseArrayIndexInArray,
    setAllPlacementArrayLengths,
    setArrayLength
  } from './grid';

  type InterfacePlacement = {id: string,
                             interface: string,
                             name: string,
                             description: string,
                             // TODO: supported placement types/config here
                             kind: string};

  export let default_audience_placements: Array<InterfacePlacement> = new Array(0);

  type Audience = {id: string,
                   name: string,
                   placements: Array<InterfacePlacement>}

  export let audiences: Array<Audience> = new Array(0);

  type TimelinePlacement = {id: string,
                            component: string,
                            kind: ('command' | 'readModel'),
                            name: string,
                            description: string}

  export let timeline_placements: Array<TimelinePlacement> = new Array(0);

  type EventPlacement = {id: string,
                         event: string,
                         name: string,
                         description: string}

  type Stream = {id: string,
                 name: string,
                 placements: Array<EventPlacement>}

  export let streams: Array<Stream> = new Array(0);

  export let default_stream_placements: Array<EventPlacement> = new Array(0);

  // Rows

  const default_audience_row = 0;

  let timeline_row = audiences.length + 1;

  let default_stream_row = timeline_row + streams.length + 1;

  let row_count = default_stream_row + 1;

  let cursor_row = timeline_row

  // Columns

  const right_buffer = 10;

  let cursor_column = 0;

  $: max_column =
  Math.max(
    maxSparseArrayIndex(default_audience_placements),
    maxSparseArrayIndexInArray(audiences.map((a) => a.placements)),
    maxSparseArrayIndex(timeline_placements),
    maxSparseArrayIndexInArray(streams.map((s) => s.placements)),
    maxSparseArrayIndex(default_stream_placements),
    cursor_column
  ) + right_buffer;

  $: setArrayLength(default_audience_placements, max_column);
  $: setAllPlacementArrayLengths(audiences, max_column);
  $: setArrayLength(timeline_placements, max_column);
  $: setAllPlacementArrayLengths(streams, max_column);
  $: setArrayLength(default_stream_placements, max_column);

  // List IDs

  const placementOrEmptyCellId = (placement: {id: string} | null | undefined, col: number, row: number): string => {
    return (placement && placement.id) || `empty-${col}-${row}`;
  }

  // Keyboard

  import { createKeybindingsHandler } from "../vendor/tinykeys/tinykeys"

  const keyboardHandler = createKeybindingsHandler({
    "ArrowUp": event => {
      event.preventDefault()
      cursor_row = Math.max(cursor_row - 1, 0)
      console.log("cursor:", cursor_row, cursor_column)
    },
    "ArrowRight": event => {
      event.preventDefault()
      cursor_column += 1
      console.log("cursor:", cursor_row, cursor_column)
    },
    "ArrowDown": event => {
      event.preventDefault()
      cursor_row = Math.min(cursor_row + 1, default_stream_row)
      console.log("cursor:", cursor_row, cursor_column)
    },
    "ArrowLeft": event => {
      event.preventDefault()
      cursor_column = Math.max(cursor_column - 1, 0)
      console.log("cursor:", cursor_row, cursor_column)
    },
  })
</script>

<svelte:window on:keydown={keyboardHandler}/>

<div class="overflow-auto h-full w-full bg-gray-canvas dark:bg-dark-1">
  <div
    class="p-3 relative grid justify-items-center items-center"
    style="grid-template-columns: repeat({max_column}, min-content); grid-template-rows: repeat({row_count}, minmax(108px, min-content));"
    >
    <Cursor row={cursor_row} column={cursor_column} />
    <!-- TODO: make the card size a constant someplace -->
    <Audience row={default_audience_row}>
      {#each default_audience_placements as placement, index (placementOrEmptyCellId(placement, index, default_audience_row))}
        {#if placement}
        <Interface
          id={placement.id}
          interface_id={placement.interface}
          name={placement.name}
          description={placement.description}
          row={default_audience_row}
          column={index}
          />
        {:else}
          <EmptyCell row={default_audience_row} column={index} />
        {/if}
      {/each}
    </Audience>

    {#each audiences as audience, i}
      {@const row = i + 1}
    <Audience {row} name={audience.name}>
      {#each audience.placements as placement, index (placementOrEmptyCellId(placement, index, row))}
      {#if placement}
        <Interface
          id={placement.id}
          interface_id={placement.interface}
          name={placement.name}
          description={placement.description}
          {row}
          column={index}
          />
        {:else}
          <EmptyCell {row} column={index} />
        {/if}
      {/each}
    </Audience>
  {/each}

<Timeline row={timeline_row}>
  {#each timeline_placements as placement, index (placementOrEmptyCellId(placement, index, timeline_row))}
  <!-- {@const is_editing = isEditing(timeline_row, index)} -->
  {#if placement && placement.kind === 'command'}
    <Command
      id={placement.id}
      command={placement.component}
      name={placement.name}
      description={placement.description}
      row={timeline_row}
      column={index}
      />
    {:else if placement && placement.kind === 'readModel'}
      <ReadModel
        id={placement.id}
        readModel={placement.component}
        name={placement.name}
        description={placement.description}
        row={timeline_row}
        column={index}
        />
      {:else}
        <EmptyCell row={timeline_row} column={index} />
      {/if}
    {/each}
  </Timeline>

{#each streams as stream, i}
  {@const row = i + timeline_row + 1}
<Stream {row} name={stream.name}>
  {#each stream.placements as placement, index (placementOrEmptyCellId(placement, index, row))}
  <!-- {@const is_editing = isEditing(row, index)} -->
  {#if placement}
    <Event
      id={placement.id}
      event={placement.event}
      name={placement.name}
      description={placement.description}
      {row}
      column={index}
      />
    {:else}
      <EmptyCell {row} column={index} />
    {/if}
  {/each}
</Stream>
    {/each}

<Stream row={default_stream_row}>
  {#each default_stream_placements as placement, index (placementOrEmptyCellId(placement, index, default_stream_row))}
  <!-- {@const is_editing = isEditing(default_stream_row, index)} -->
  {#if placement}
    <Event
      id={placement.id}
      event={placement.event}
      name={placement.name}
      description={placement.description}
      row={default_stream_row}
      column={index}
      />
    {:else}
      <EmptyCell row={default_stream_row} column={index} />
    {/if}
  {/each}
</Stream>
  </div>
</div>
