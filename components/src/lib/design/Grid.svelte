<svelte:options immutable />

<script lang="ts">
  import { createKeybindingsHandler, type KeyBindingMap } from "../vendor/tinykeys/tinykeys"

  import Cursor from "./grid/Cursor.svelte";
  import AudienceLane from './grid/Audience.svelte';
  import Timeline from './grid/Timeline.svelte';
  import StreamLane from './grid/Stream.svelte';

  import {type Decider, type Audience, type EventPlacement, type InterfacePlacement, type Stream, type TimelinePlacement, default_decider, type Disambiguation} from './Grid';
  import { onMount } from "svelte";
  import { itemAtCursor } from "./Grid";
  import TimelineDisambiguation from "./grid/TimelineDisambiguation.svelte";

  export let decider: Decider = default_decider;
  export let default_audience_placements: Array<InterfacePlacement> = new Array(0);
  export let audiences: Array<Audience> = new Array(0);
  export let timeline_placements: Array<TimelinePlacement> = new Array(0);
  export let streams: Array<Stream> = new Array(0);
  export let default_stream_placements: Array<EventPlacement> = new Array(0);

  // Grid Mode

  let mode: 'loading' | 'navigation' | 'editing' | 'disambiguating' | 'linking' = 'loading'

  onMount(() => {
    mode = 'navigation'
  })

  // Disambiguation

  let disambiguation: Disambiguation = null;

  // Command Dispatch

  const handleDefineAndPlaceInterface = async (e: CustomEvent) => {
    await decider.define_and_place_interface(e.detail.name, e.detail.index, e.detail.audience);
    mode = 'navigation'
  }
  const handleDefineAndPlaceCommand = async (e: CustomEvent) => {
    await decider.define_and_place_command(e.detail.name, e.detail.index);
    disambiguation = null;
    mode = 'navigation';
  }
  const handleDefineAndPlaceEvent = async (e: CustomEvent) => {
    await decider.define_and_place_event(e.detail.name, e.detail.index, e.detail.stream);
    mode = 'navigation'
  }
  const handleDefineAndPlaceReadModel = async (e: CustomEvent) => {
    await decider.define_and_place_read_model(e.detail.name, e.detail.index);
    disambiguation = null;
    mode = 'navigation';
  }
  const handleDisambiguateTimelineDefinitionAndPlacement = (e: CustomEvent) => {
    mode = 'disambiguating';
    disambiguation = e.detail;
  }
  const handleRenamePlacement = async (e: CustomEvent) => {
    await decider.rename_placement(e.detail.placement, e.detail.name);
    mode = 'navigation'
  }

  // Rows

  const default_audience_row = 0;
  $: timeline_row = audiences.length + 1;
  $: default_stream_row = timeline_row + streams.length + 1;
  $: row_count = default_stream_row + 1;

  // Cursor

  let cursor_row = 0;
  onMount(() => {cursor_row = timeline_row})
  let cursor_column = 0;
  $: cursor_item = itemAtCursor(cursor_row, cursor_column,
                                default_audience_placements,
                                audiences,
                                timeline_placements,
                                streams,
                                default_stream_placements);
  $: cursor_is_editing = mode === 'editing';

  // Columns

  const right_buffer = 10;

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

  // Navigation

  const handleNavigateCursor = (event: CustomEvent) => {
    mode = 'navigation'
    cursor_row = event.detail.row
    cursor_column = event.detail.column
  }

  const navUp = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_row = Math.max(cursor_row - 1, 0)
  }

  const navRight = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_column += 1
  }

  const navDown = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_row = Math.min(cursor_row + 1, default_stream_row)
  }

  const navLeft = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_column = Math.max(cursor_column - 1, 0)
  }

  const navHome = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_row = timeline_row
    cursor_column = 0
  }

  const navStart = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_column = 0
  }

  const navEnd = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_column = max_column - right_buffer
  }

  const navTop = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_row = default_audience_row
  }

  const navBottom = (event: KeyboardEvent) => {
    event.preventDefault()
    cursor_row = default_stream_row
  }

  const navigationKeys: KeyBindingMap = {
    "ArrowUp": navUp,
    "k": navUp,

    "ArrowRight": navRight,
    "l": navRight,
    "Tab": navRight,

    "ArrowDown": navDown,
    "j": navDown,

    "ArrowLeft": navLeft,
    "h": navLeft,
    "Shift+Tab": navLeft,

    "Home": navHome,

    "Control+a": navStart,
    "0": navStart,

    "End": navEnd,
    "Shift+4": navEnd,
    "Control+e": navEnd,

    "PageUp": navTop,
    "g g": navTop,

    "PageDown": navBottom,
    "Shift+G": navBottom,

    "Enter": (event) => {
      event.preventDefault()
      mode = 'editing'
    }
  }

  const navigationKeyboardHandler = createKeybindingsHandler(navigationKeys)

  // Editing

  const handleCancelEditing = (event: any) => {
    event.preventDefault();
    mode = 'navigation'
  }

  // Linking

  // const linkingKeyboardHandler = createKeybindingsHandler({
  // })

  // Keyboard
  const keyboardHandler: EventListener = (e) => {
    if (mode === 'navigation') {
      navigationKeyboardHandler(e)
    }
  }
</script>

<svelte:window on:keydown={keyboardHandler}/>

<h3>{mode}</h3>

<div class="overflow-auto h-full w-full bg-gray-canvas dark:bg-dark-1">
  <div
    class="p-3 relative grid justify-items-center items-center"
    style="grid-template-columns: repeat({max_column}, min-content); grid-template-rows: repeat({row_count}, minmax(108px, min-content));">

    <AudienceLane
      on:navigateCursor={handleNavigateCursor}
      row={default_audience_row}
      audience={{placements: default_audience_placements}}
      {max_column} />

    {#each audiences as audience, i} {@const row = i + 1}
      <AudienceLane on:navigateCursor={handleNavigateCursor} {row} {audience} {max_column} />
    {/each}

<Timeline on:navigateCursor={handleNavigateCursor}
          row={timeline_row}
          placements={timeline_placements}
          {max_column} />

{#each streams as stream, i} {@const row = i + timeline_row + 1}
  <StreamLane on:navigateCursor={handleNavigateCursor} {row} {stream} {max_column} />
{/each}
<StreamLane on:navigateCursor={handleNavigateCursor} row={default_stream_row} stream={{placements: default_stream_placements}} {max_column} />
<Cursor
  on:define_and_place_interface={handleDefineAndPlaceInterface}
  on:disambiguate_timeline_definition_and_placement={handleDisambiguateTimelineDefinitionAndPlacement}
  on:define_and_place_event={handleDefineAndPlaceEvent}
  on:rename_placement={handleRenamePlacement}
  on:cancel_editing={handleCancelEditing}
  row={cursor_row}
  column={cursor_column}
  item={cursor_item}
  editing={cursor_is_editing} />
{#if mode === 'disambiguating' && disambiguation}
  <TimelineDisambiguation name={disambiguation.name}
                          index={disambiguation.index}
                          top={disambiguation.top}
                          left={disambiguation.left}
                          on:define_and_place_command={handleDefineAndPlaceCommand}
                          on:define_and_place_read_model={handleDefineAndPlaceReadModel} />
                        {/if}
                      </div>
</div>
