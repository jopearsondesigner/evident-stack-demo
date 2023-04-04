<svelte:options immutable />

<script lang="ts">
  import Cursor from "./grid/Cursor.svelte";
  import AudienceLane from './grid/Audience.svelte';
  import Timeline from './grid/Timeline.svelte';
  import StreamLane from './grid/Stream.svelte';

  import Event from './grid/Event.svelte';
  import EmptyCell from './grid/EmptyCell.svelte';
  import type {Audience, EventPlacement, InterfacePlacement, TimelinePlacement} from './Grid';

  export let default_audience_placements: Array<InterfacePlacement> = new Array(0);
  export let audiences: Array<Audience> = new Array(0);
  export let timeline_placements: Array<TimelinePlacement> = new Array(0);
  export let streams: Array<Stream> = new Array(0);
  export let default_stream_placements: Array<EventPlacement> = new Array(0);

  // Rows

  const default_audience_row = 0;
  $: timeline_row = audiences.length + 1;
  $: default_stream_row = timeline_row + streams.length + 1;
  $: row_count = default_stream_row + 1;
  $: cursor_row = timeline_row

  // Columns

  const right_buffer = 10;

  let cursor_column = 0;

  export function maxSparseArrayIndex(array: Array<any>): number {
    let max = 0;
    for (let i in array) {
      max = Math.max(max, parseInt(i));
    }
    return max
  }

  export function maxSparseArrayIndexInArray(maps: Array<Array<any>>): number {
    return Math.max(...maps.map(m => maxSparseArrayIndex(m)))
  }

  $: max_column = Math.max(
    maxSparseArrayIndex(default_audience_placements),
    maxSparseArrayIndexInArray(audiences.map((a) => a.placements)),
    maxSparseArrayIndex(timeline_placements),
    maxSparseArrayIndexInArray(streams.map((s) => s.placements)),
    maxSparseArrayIndex(default_stream_placements),
    cursor_column
  ) + right_buffer;

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
    style="grid-template-columns: repeat({max_column}, min-content); grid-template-rows: repeat({row_count}, minmax(108px, min-content));">
    <Cursor row={cursor_row} column={cursor_column} />
    <!-- TODO: make the card size a constant someplace -->
    <AudienceLane row={default_audience_row} audience={{placements: default_audience_placements}} {max_column} />

    {#each audiences as audience, i} {@const row = i + 1} <AudienceLane {row} {audience} {max_column} />{/each}

    <Timeline row={timeline_row} placements={timeline_placements} {max_column} />

    {#each streams as stream, i} {@const row = i + timeline_row + 1}
      <StreamLane {row} {stream} {max_column} />
    {/each}

    <StreamLane row={default_stream_row} stream={{placements: default_stream_placements}} {max_column} />
  </div>
</div>
