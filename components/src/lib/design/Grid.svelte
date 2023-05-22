<svelte:options immutable />

<script lang="ts">
  import {
    type DraggingState,
    DraggingStateKind,
    type DragCommand,
    DraggingCommandKind,
    buildEvolveAndReact,
    type LaneSource,
    type PlacementSource,
    type RowTarget,
    type CellTarget,
    type WithSourceEffect,
    type FlowPortSource,
    DEFAULT_LANE,
    linkingFlowFromState,
    type CursorTarget
  } from './grid/util';

  import ContextMenu from '../context/ContextMenu.svelte';
  import ContextMenuItem from '../context/ContextMenuItem.svelte';
  import ContextMenuDivider from '../context/ContextMenuDivider.svelte';
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
  import type { DragEventHandler } from 'svelte/elements';

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

  // Drag Drop
  let dragState: DraggingState = { kind: DraggingStateKind.NONE };

  const evolveAndReactDraggingState = buildEvolveAndReact(decider);

  const handleLaneDragStart = async (e: CustomEvent<LaneSource>) => {
    const command: DragCommand = {
      kind: DraggingCommandKind.LANE_DRAG_START,
      value: e.detail
    };

    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handleLaneDragEnter = async (e: CustomEvent<RowTarget>) => {
    const command: DragCommand = {
      kind: DraggingCommandKind.LANE_DRAG_ENTER,
      value: e.detail
    };

    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handleLaneDragDrop = async (e: CustomEvent) => {
    const command: DragCommand = {
      kind: DraggingCommandKind.LANE_DRAG_DROP
    };

    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handlePlacementDragStart = async (e: CustomEvent<PlacementSource & WithSourceEffect>) => {
    const command: DragCommand = {
      kind: DraggingCommandKind.PLACEMENT_DRAG_START,
      value: e.detail
    };

    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handleFlowDragStart = async (e: CustomEvent<FlowPortSource>) => {
    const command: DragCommand = {
      kind: DraggingCommandKind.FLOW_PORT_DRAG_START,
      value: e.detail
    };

    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handleCellDragEnter = async (e: CustomEvent<CellTarget>) => {
    const command: DragCommand = {
      kind: DraggingCommandKind.CELL_DRAG_ENTER,
      value: e.detail
    };

    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handleCellDragDrop = async (e: CustomEvent) => {
    const command: DragCommand = { kind: DraggingCommandKind.CELL_DRAG_DROP };

    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handleOutOfBoundsDragEnter: DragEventHandler<EventTarget> = (_e) => {
    console.info('DRAG OUT OF BOUNDS');
    const command: DragCommand = { kind: DraggingCommandKind.OUT_OF_BOUNDS_DRAG_ENTER };
    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handleOutOfBoundsDragEnd: DragEventHandler<EventTarget> = (_e) => {
    console.info('DRAG DROP OUT OF BOUNDS END');
    const command: DragCommand = { kind: DraggingCommandKind.OUT_OF_BOUNDS_DRAG_END };
    dragState = evolveAndReactDraggingState(dragState, command);
  };

  // Context Menu
  const handleOpenContextMenu = async (e: CustomEvent<CellTarget & CursorTarget>) => {
    const command: DragCommand = { kind: DraggingCommandKind.OPEN_CONTEXT_MENU, value: e.detail };
    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const handleCloseContextMenu = async (e: CustomEvent) => {
    const command: DragCommand = { kind: DraggingCommandKind.CLOSE_CONTEXT_MENU };
    dragState = evolveAndReactDraggingState(dragState, command);
  };

  const stateToContextMenu = (state: DraggingState) => {
    if (state.kind !== DraggingStateKind.CONTEXT) {
      return undefined;
    }

    const clientRect = containerRef.getBoundingClientRect();
    const { x, y, placementId, placementKind, row } = state.source;
    const { rowKind } = row;

    return {
      x: x - clientRect.x,
      y: y - clientRect.y,
      rowKind,
      placementId,
      placementKind,
      defaultLane:
        rowKind === 'audience'
          ? row.audienceId === DEFAULT_LANE
          : rowKind === 'stream'
          ? row.streamId === DEFAULT_LANE
          : false
    };
  };

  $: contextMenu = stateToContextMenu(dragState);

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

  // Placement Events
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

  // Rows

  const default_audience_row = 0;
  $: timeline_row = audiences.length + 1;
  $: default_stream_row = timeline_row + streams.length + 1;
  $: row_count = default_stream_row + 1;

  // Lanes
  $: default_stream_lane_index = streams.length;
  $: default_audience_lane_index = audiences.length;
  $: lane_drop_target =
    dragState.kind === DraggingStateKind.LANE ? dragState.value.target : undefined;
  $: audience_drop_target =
    lane_drop_target?.rowKind == 'audience'
      ? { index: lane_drop_target.laneIndex, targetStatus: lane_drop_target.targetStatus }
      : undefined;
  $: stream_drop_target =
    lane_drop_target?.rowKind == 'stream'
      ? { index: lane_drop_target.laneIndex, targetStatus: lane_drop_target.targetStatus }
      : undefined;

  $: timeline_drop_target =
    lane_drop_target?.rowKind == 'timeline' ? lane_drop_target.targetStatus : undefined;

  $: cell_drop_target =
    dragState.kind === DraggingStateKind.PLACEMENT ? dragState.value.target : undefined;

  // Flows
  $: linkingFlow = linkingFlowFromState(dragState);
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

  // Keyboard
  const keyboardHandler: EventListener = (e) => {
    if (mode === 'navigation') {
      navigationKeyboardHandler(e);
    }
  };

  let containerRef: HTMLDivElement;

  // Drag Test
  const handleDragOver = (e: DragEvent) => {
    const clientRect = containerRef.getBoundingClientRect();
    const command: DragCommand = {
      kind: DraggingCommandKind.CURSOR_MOVE,
      value: { x: e.clientX - clientRect.x, y: e.clientY - clientRect.y }
    };

    dragState = evolveAndReactDraggingState(dragState, command);
  };

  $: allFlows = linkingFlow ? flows.concat(linkingFlow) : flows;
</script>

<svelte:window
  on:keydown={keyboardHandler}
  on:dragenter={handleOutOfBoundsDragEnter}
  on:dragend={handleOutOfBoundsDragEnd}
/>

<h3>{mode}</h3>
<h3>{ JSON.stringify(contextMenu) }</h3>
<div class="overflow-auto z-[0] relative h-full w-full bg-gray-canvas dark:bg-dark-1">
  <div
    bind:this={containerRef}
    on:dragover={handleDragOver}
    class="grid w-max p-3 relative justify-items-center items-center"
    style="grid-template-columns: repeat({max_column}, min-content); grid-template-rows: repeat({row_count}, minmax(108px, min-content));"
  >
    <FlowCanvas flows={allFlows} />
    {#if contextMenu}
      <ContextMenu
        x={contextMenu.x}
        y={contextMenu.y}
        on:click={handleCloseContextMenu}
        on:clickoutside={handleCloseContextMenu}
      >
        {#if !contextMenu.placementId}
          {#if contextMenu.rowKind === "stream"}
            <ContextMenuItem>Add Event</ContextMenuItem>
          {:else if contextMenu.rowKind === "audience"}
            <ContextMenuItem>Add Interface</ContextMenuItem>
          {:else if contextMenu.rowKind === "timeline"}
            <ContextMenuItem>Add Read Model</ContextMenuItem>
            <ContextMenuItem>Add Command</ContextMenuItem>
          {/if}
        {:else}
          {#if contextMenu.placementKind === "event"}
            <ContextMenuItem>Delete Event</ContextMenuItem>
          {:else if contextMenu.placementKind === "interface"}
            <ContextMenuItem>Delete Interface</ContextMenuItem>
          {:else if contextMenu.placementKind === "readModel"}
            <ContextMenuItem>Delete Read Model</ContextMenuItem>
          {:else if contextMenu.placementKind === "command"}
            <ContextMenuItem>Delete Command</ContextMenuItem>
          {/if}
        {/if}
        <ContextMenuDivider />
        <ContextMenuItem>Insert Column Left</ContextMenuItem>
        <ContextMenuItem>Insert Column Right</ContextMenuItem>
        {#if contextMenu.rowKind !== "audience" || !contextMenu.defaultLane}
          <ContextMenuItem>Insert Lane Above</ContextMenuItem>
        {/if}
        {#if contextMenu.rowKind !== "stream" || !contextMenu.defaultLane}
          <ContextMenuItem>Insert Lane Below</ContextMenuItem>
        {/if}
        <ContextMenuDivider />
        <ContextMenuItem>Import Event Model JSON</ContextMenuItem>
      </ContextMenu>
    {/if}
    <AudienceLane
      on:navigate_cursor={handleNavigateCursor}
      on:lane_drag_drop={handleLaneDragDrop}
      on:placement_drag_start={handlePlacementDragStart}
      on:cell_drag_enter={handleCellDragEnter}
      on:cell_drag_drop={handleCellDragDrop}
      on:flow_drag_start={handleFlowDragStart}
      on:open_context_menu={handleOpenContextMenu}
      targeted_cell={cell_drop_target &&
      cell_drop_target.row.rowKind === 'audience' &&
      cell_drop_target.row.audienceId == DEFAULT_LANE
        ? {
            column: cell_drop_target.column,
            targetStatus: cell_drop_target.targetStatus
          }
        : undefined}
      targeted_lane={audience_drop_target?.index === default_audience_lane_index
        ? audience_drop_target.targetStatus
        : undefined}
      row={default_audience_row}
      audience={{ placements: default_audience_placements }}
      lane_index={default_audience_lane_index}
      {max_column}
    />

    {#each audiences as audience, i (audience.id)}
      {@const row = i + 1}
      {@const lane_index = audiences.length - 1 - i}
      {@const targeted_lane =
        audience_drop_target?.index == lane_index ? audience_drop_target.targetStatus : undefined}
      {@const targeted_cell =
        cell_drop_target &&
        cell_drop_target.row.rowKind === 'audience' &&
        cell_drop_target.row.audienceId == audience.id
          ? {
              column: cell_drop_target.column,
              targetStatus: cell_drop_target.targetStatus
            }
          : undefined}
      <AudienceLane
        on:navigate_cursor={handleNavigateCursor}
        on:lane_drag_start={handleLaneDragStart}
        on:lane_drag_enter={handleLaneDragEnter}
        on:lane_drag_drop={handleLaneDragDrop}
        on:placement_drag_start={handlePlacementDragStart}
        on:cell_drag_enter={handleCellDragEnter}
        on:cell_drag_drop={handleCellDragDrop}
        on:flow_drag_start={handleFlowDragStart}
        on:open_context_menu={handleOpenContextMenu}
        {targeted_cell}
        {targeted_lane}
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
      on:placement_drag_start={handlePlacementDragStart}
      on:cell_drag_enter={handleCellDragEnter}
      on:cell_drag_drop={handleCellDragDrop}
      on:flow_drag_start={handleFlowDragStart}
      on:open_context_menu={handleOpenContextMenu}
      row={timeline_row}
      placements={timeline_placements}
      targeted_lane={timeline_drop_target}
      targeted_cell={cell_drop_target && cell_drop_target.row.rowKind === 'timeline'
        ? { column: cell_drop_target.column, targetStatus: cell_drop_target.targetStatus }
        : undefined}
      {max_column}
    />

    {#each streams as stream, lane_index (stream.id)}
      {@const row = lane_index + timeline_row + 1}
      {@const targeted_lane =
        stream_drop_target?.index == lane_index ? stream_drop_target.targetStatus : undefined}
      {@const targeted_cell =
        cell_drop_target &&
        cell_drop_target.row.rowKind === 'stream' &&
        cell_drop_target.row.streamId == stream.id
          ? {
              column: cell_drop_target.column,
              targetStatus: cell_drop_target.targetStatus
            }
          : undefined}
      <StreamLane
        on:navigate_cursor={handleNavigateCursor}
        on:lane_drag_start={handleLaneDragStart}
        on:lane_drag_enter={handleLaneDragEnter}
        on:lane_drag_drop={handleLaneDragDrop}
        on:placement_drag_start={handlePlacementDragStart}
        on:cell_drag_enter={handleCellDragEnter}
        on:cell_drag_drop={handleCellDragDrop}
        on:flow_drag_start={handleFlowDragStart}
        on:open_context_menu={handleOpenContextMenu}
        {targeted_lane}
        {targeted_cell}
        {row}
        {stream}
        {max_column}
        {lane_index}
      />
    {/each}
    <StreamLane
      on:navigate_cursor={handleNavigateCursor}
      on:lane_drag_drop={handleLaneDragDrop}
      on:placement_drag_start={handlePlacementDragStart}
      on:cell_drag_enter={handleCellDragEnter}
      on:cell_drag_drop={handleCellDragDrop}
      on:flow_drag_start={handleFlowDragStart}
      on:open_context_menu={handleOpenContextMenu}
      targeted_cell={cell_drop_target &&
      cell_drop_target.row.rowKind === 'stream' &&
      cell_drop_target.row.streamId == DEFAULT_LANE
        ? {
            column: cell_drop_target.column,
            targetStatus: cell_drop_target.targetStatus
          }
        : undefined}
      targeted_lane={stream_drop_target?.index === default_stream_lane_index
        ? stream_drop_target.targetStatus
        : undefined}
      lane_index={default_stream_lane_index}
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
      on:placement_drag_start={handlePlacementDragStart}
      on:cell_drag_enter={handleCellDragEnter}
      on:cell_drag_drop={handleCellDragDrop}
      on:flow_drag_start={handleFlowDragStart}
      on:open_context_menu={handleOpenContextMenu}
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
