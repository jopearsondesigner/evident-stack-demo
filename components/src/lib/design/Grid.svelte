<svelte:options immutable />

<script lang="ts">
  import FlowCanvas from './flowArrows/FlowCanvas.svelte';
  import { createKeybindingsHandler, type KeyBindingMap } from '../vendor/tinykeys/tinykeys';

  import Cursor from './grid/Cursor.svelte';
  import AudienceLane from './grid/Audience.svelte';
  import Timeline from './grid/Timeline.svelte';
  import StreamLane from './grid/Stream.svelte';

  import {
    type Decider,
    type Audience,
    type EventPlacement,
    type InterfacePlacement,
    type Stream,
    type TimelinePlacement,
    default_decider,
    type Disambiguation,
    type CursorMode,
    type GridMode,
    type Flow
  } from './Grid';
  import { onMount } from 'svelte';
  import { itemAtCursor } from './Grid';
  import TimelineDisambiguation from './grid/TimelineDisambiguation.svelte';

  export let decider: Decider = default_decider;
  export let column_count: number;
  export let default_audience_placements: Array<InterfacePlacement> = new Array(0);
  export let audiences: Array<Audience> = new Array(0);
  export let timeline_placements: Array<TimelinePlacement> = new Array(0);
  export let streams: Array<Stream> = new Array(0);
  export let default_stream_placements: Array<EventPlacement> = new Array(0);
  export let flows: Array<Flow> = [];

  // Grid Mode

  let mode: GridMode = 'loading';

  onMount(() => {
    mode = 'navigation';
  });

  // Disambiguation

  let disambiguation: Disambiguation = null;

  // Command Dispatch

  const handleDefineAndPlaceInterface = async (e: CustomEvent) => {
    await decider.define_and_place_interface(e.detail.name, e.detail.index, e.detail.audience);
    mode = 'navigation';
  };
  const handleDefineAndPlaceCommand = async (e: CustomEvent) => {
    await decider.define_and_place_command(e.detail.name, e.detail.index);
    disambiguation = null;
    mode = 'navigation';
  };
  const handleDefineAndPlaceEvent = async (e: CustomEvent) => {
    await decider.define_and_place_event(e.detail.name, e.detail.index, e.detail.stream);
    mode = 'navigation';
  };
  const handleDefineAndPlaceReadModel = async (e: CustomEvent) => {
    await decider.define_and_place_read_model(e.detail.name, e.detail.index);
    disambiguation = null;
    mode = 'navigation';
  };
  const handleDisambiguateTimelineDefinitionAndPlacement = (e: CustomEvent) => {
    mode = 'disambiguating';
    disambiguation = e.detail;
  };

  const handleMoveInterfacePlacement = async (e: CustomEvent) => {
    await decider.move_interface_placement(e.detail.id, e.detail.index, e.detail.audience);
  };

  const handleDuplicateInterfacePlacement = async (e: CustomEvent) => {
    await decider.duplicate_interface_placement(e.detail.id, e.detail.index, e.detail.audience);
  };

  const handleMoveTimelinePlacement = async (e: CustomEvent) => {
    await decider.move_timeline_placement(e.detail.id, e.detail.index);
  };

  const handleDuplicateTimelinePlacement = async (e: CustomEvent) => {
    await decider.duplicate_timeline_placement(e.detail.id, e.detail.index);
  };

  const handleMoveEventPlacement = async (e: CustomEvent) => {
    await decider.move_event_placement(e.detail.id, e.detail.index, e.detail.stream);
  };

  const handleDuplicateEventPlacement = async (e: CustomEvent) => {
    await decider.duplicate_event_placement(e.detail.id, e.detail.index, e.detail.stream);
  };

  const handleRemovePlacement = async (e: CustomEvent) => {
    await decider.remove_placement(e.detail.placement);
    mode = 'navigation';
  };

  const handleRenamePlacement = async (e: CustomEvent) => {
    await decider.rename_placement(e.detail.placement, e.detail.name);
    mode = 'navigation';
  };

  const handleRenameLane = async (e: CustomEvent) => {
    console.warn('TODO: handleRenameLane');
    console.warn(e.detail);
  };

  const handleReorderLane = async (e: CustomEvent) => {
    console.warn('TODO: handleReorderLane');
    console.warn(e.detail);
    decider.reorder_lane(e.detail.kind, e.detail.lane_id, e.detail.index);
  };

  const handleRemoveLane = async (e: CustomEvent) => {
    console.warn('TODO: handleRemoveLane');
    console.warn(e.detail);
  };

  // Rows

  const default_audience_row = 0;
  $: timeline_row = audiences.length + 1;
  $: default_stream_row = timeline_row + streams.length + 1;
  $: row_count = default_stream_row + 1;

  // Cursor

  let cursor_row = 0;
  onMount(() => {
    cursor_row = timeline_row;
  });
  let cursor_column = 0;
  $: cursor_item = itemAtCursor(
    cursor_row,
    cursor_column,
    default_audience_placements,
    audiences,
    timeline_placements,
    streams,
    default_stream_placements
  );

  const gridModeToCursorMode = (mode: GridMode): CursorMode => {
    return mode === 'editing'
      ? 'editing'
      : mode === 'navigation'
      ? 'navigation'
      : mode === 'linking'
      ? 'linking'
      : 'other';
  };
  $: cursor_mode = gridModeToCursorMode(mode);

  const handleBeginEditing: EventListener = (e) => {
    e.preventDefault();
    mode = 'editing';
  };

  // Columns

  $: max_column = Math.max(column_count, cursor_column + 10);

  // Navigation

  const handleNavigateCursor = (event: CustomEvent) => {
    mode = 'navigation';
    cursor_row = event.detail.row;
    cursor_column = event.detail.column;
  };

  const navUp = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_row = Math.max(cursor_row - 1, 0);
  };

  const navRight = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_column += 1;
  };

  const navDown = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_row = Math.min(cursor_row + 1, default_stream_row);
  };

  const navLeft = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_column = Math.max(cursor_column - 1, 0);
  };

  const navHome = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_row = timeline_row;
    cursor_column = 0;
  };

  const navStart = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_column = 0;
  };

  const navEnd = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_column = max_column;
  };

  const navTop = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_row = default_audience_row;
  };

  const navBottom = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_row = default_stream_row;
  };

  const navigationKeys: KeyBindingMap = {
    ArrowUp: navUp,
    k: navUp,

    ArrowRight: navRight,
    l: navRight,
    Tab: navRight,

    ArrowDown: navDown,
    j: navDown,

    ArrowLeft: navLeft,
    h: navLeft,
    'Shift+Tab': navLeft,

    Home: navHome,

    'Control+a': navStart,
    '0': navStart,

    End: navEnd,
    'Shift+4': navEnd,
    'Control+e': navEnd,

    PageUp: navTop,
    'g g': navTop,

    PageDown: navBottom,
    'Shift+G': navBottom,

    Enter: (event) => {
      event.preventDefault();
      mode = 'editing';
    }
  };

  const navigationKeyboardHandler = createKeybindingsHandler(navigationKeys);

  // Editing

  const handleCancelEditing = (event: any) => {
    event.preventDefault();
    mode = 'navigation';
  };

  // Linking

  // const linkingKeyboardHandler = createKeybindingsHandler({
  // })

  // TODO: wire up to decider
  const handleConnectFlow = async (e: CustomEvent) => {
    console.log('connect_flow:', e);
  };

  // Keyboard
  const keyboardHandler: EventListener = (e) => {
    if (mode === 'navigation') {
      navigationKeyboardHandler(e);
    }
  };
</script>

<svelte:window on:keydown={keyboardHandler} />

<h3>{mode}</h3>

<div class="overflow-auto z-[0] relative h-full w-full bg-gray-canvas dark:bg-dark-1">
  <FlowCanvas {flows} />
  <div
    class="p-3 relative grid justify-items-center items-center"
    style="grid-template-columns: repeat({max_column}, min-content); grid-template-rows: repeat({row_count}, minmax(108px, min-content));"
  >
    <AudienceLane
      on:navigate_cursor={handleNavigateCursor}
      on:move_interface_placement={handleMoveInterfacePlacement}
      on:duplicate_interface_placement={handleDuplicateInterfacePlacement}
      on:connect_flow={handleConnectFlow}
      row={default_audience_row}
      lane_index={audiences.length}
      audience={{ placements: default_audience_placements }}
      {max_column}
    />

    {#each audiences as audience, lane_index (audience.id)}
      {@const row = lane_index + 1}
      <AudienceLane
        on:navigate_cursor={handleNavigateCursor}
        on:move_interface_placement={handleMoveInterfacePlacement}
        on:duplicate_interface_placement={handleDuplicateInterfacePlacement}
        on:connect_flow={handleConnectFlow}
        {row}
        {audience}
        {max_column}
        {lane_index}
      />
    {/each}

    <Timeline
      on:navigate_cursor={handleNavigateCursor}
      on:move_timeline_placement={handleMoveTimelinePlacement}
      on:duplicate_timeline_placement={handleDuplicateTimelinePlacement}
      on:connect_flow={handleConnectFlow}
      row={timeline_row}
      placements={timeline_placements}
      {max_column}
    />

    {#each streams as stream, lane_index (stream.id)}
      {@const row = lane_index + timeline_row + 1}
      <StreamLane
        on:navigate_cursor={handleNavigateCursor}
        on:move_event_placement={handleMoveEventPlacement}
        on:duplicate_event_placement={handleDuplicateEventPlacement}
        on:connect_flow={handleConnectFlow}
        on:reorder_lane={handleReorderLane}
        {row}
        {stream}
        {max_column}
        {lane_index}
      />
    {/each}
    <StreamLane
      on:navigate_cursor={handleNavigateCursor}
      on:move_event_placement={handleMoveEventPlacement}
      on:duplicate_event_placement={handleDuplicateEventPlacement}
      on:connect_flow={handleConnectFlow}
      lane_index={streams.length}
      row={default_stream_row}
      stream={{ placements: default_stream_placements }}
      {max_column}
    />
    <Cursor
      on:begin_editing={handleBeginEditing}
      on:cancel_editing={handleCancelEditing}
      on:define_and_place_interface={handleDefineAndPlaceInterface}
      on:define_and_place_event={handleDefineAndPlaceEvent}
      on:disambiguate_timeline_definition_and_placement={handleDisambiguateTimelineDefinitionAndPlacement}
      on:remove_placement={handleRemovePlacement}
      on:rename_placement={handleRenamePlacement}
      on:move_interface_placement={handleMoveInterfacePlacement}
      on:move_timeline_placement={handleMoveTimelinePlacement}
      on:move_event_placement={handleMoveEventPlacement}
      on:duplicate_interface_placement={handleDuplicateInterfacePlacement}
      on:duplicate_timeline_placement={handleDuplicateTimelinePlacement}
      on:duplicate_event_placement={handleDuplicateEventPlacement}
      on:connect_flow={handleConnectFlow}
      row={cursor_row}
      column={cursor_column}
      item={cursor_item}
      mode={cursor_mode}
    />
    {#if mode === 'disambiguating' && disambiguation}
      <TimelineDisambiguation
        name={disambiguation.name}
        index={disambiguation.index}
        top={disambiguation.top}
        left={disambiguation.left}
        on:define_and_place_command={handleDefineAndPlaceCommand}
        on:define_and_place_read_model={handleDefineAndPlaceReadModel}
      />
    {/if}
  </div>
</div>
