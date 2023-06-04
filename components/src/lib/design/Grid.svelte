<script lang="ts">
  import {
    type GridState,
    GridStateKind,
    type GridCommand,
    GridCommandKind,
    buildEvolveAndReact,
    type LaneSource,
    type PlacementSource,
    type RowTarget,
    type CellTarget,
    type WithSourceEffect,
    type FlowPortSource,
    DEFAULT_LANE,
    linkingFlowFromState,
    type CursorTarget, ContextMenuCommand
  } from './grid/util';

  import FlowCanvas from './flowArrows/FlowCanvas.svelte';
  import ContextMenu from '../context/ContextMenu.svelte';
  import ContextMenuItem from '../context/ContextMenuItem.svelte';
  import ContextMenuDivider from '../context/ContextMenuDivider.svelte';
  import { createKeybindingsHandler, type KeyBindingMap } from '../vendor/tinykeys/tinykeys';

  import Cursor from './grid/Cursor.svelte';
  import AudienceLane from './grid/Audience.svelte';
  import Timeline from './grid/Timeline.svelte';
  import StreamLane from './grid/Stream.svelte';

  import { createEventDispatcher, onMount } from 'svelte';
  import TimelineDisambiguation from './grid/TimelineDisambiguation.svelte';
  import type { DragEventHandler } from 'svelte/elements';
  import type { CursorMode, Decider, Disambiguation, EventModelGrid, GridMode } from './Grid';

  // Event dispatch

  const dispatch = createEventDispatcher();

  // Grid Element State

  export let decider: Decider;
  export let grid: EventModelGrid; // Read Model

  // Grid Mode

  let mode: GridMode = 'loading';

  onMount(() => {
    mode = 'navigation';
  });

  // Disambiguation

  let disambiguation: Disambiguation = null;

  // Drag Drop
  let gridState: GridState = { kind: GridStateKind.NONE };

  const evolveAndReactDraggingState = buildEvolveAndReact(decider);

  const handleLaneDragStart = async (e: CustomEvent<LaneSource>) => {
    const command: GridCommand = {
      kind: GridCommandKind.LANE_DRAG_START,
      value: e.detail
    };

    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleLaneDragEnter = async (e: CustomEvent<RowTarget>) => {
    const command: GridCommand = {
      kind: GridCommandKind.LANE_DRAG_ENTER,
      value: e.detail
    };

    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleLaneDragDrop = async (_e: CustomEvent) => {
    const command: GridCommand = {
      kind: GridCommandKind.LANE_DRAG_DROP
    };

    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handlePlacementDragStart = async (e: CustomEvent<PlacementSource & WithSourceEffect>) => {
    const command: GridCommand = {
      kind: GridCommandKind.PLACEMENT_DRAG_START,
      value: e.detail
    };

    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleFlowDragStart = async (e: CustomEvent<FlowPortSource>) => {
    const command: GridCommand = {
      kind: GridCommandKind.FLOW_PORT_DRAG_START,
      value: e.detail
    };

    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleCellDragEnter = async (e: CustomEvent<CellTarget>) => {
    const command: GridCommand = {
      kind: GridCommandKind.CELL_DRAG_ENTER,
      value: e.detail
    };

    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleCellDragDrop = async (_e: CustomEvent) => {
    const command: GridCommand = { kind: GridCommandKind.CELL_DRAG_DROP };

    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleOutOfBoundsDragEnter: DragEventHandler<EventTarget> = (_e) => {
    console.info('DRAG OUT OF BOUNDS');
    const command: GridCommand = { kind: GridCommandKind.OUT_OF_BOUNDS_DRAG_ENTER };
    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleOutOfBoundsDragEnd: DragEventHandler<EventTarget> = (_e) => {
    console.info('DRAG DROP OUT OF BOUNDS END');
    const command: GridCommand = { kind: GridCommandKind.OUT_OF_BOUNDS_DRAG_END };
    gridState = evolveAndReactDraggingState(gridState, command);
  };

  // Context Menu
  const handleOpenContextMenu = async (e: CustomEvent<CellTarget & CursorTarget>) => {
    // const command: GridCommand = { kind: GridCommandKind.OPEN_CONTEXT_MENU, value: e.detail };
    // gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleCloseContextMenu = async (_e: CustomEvent) => {
    const command: GridCommand = { kind: GridCommandKind.CLOSE_CONTEXT_MENU };
    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const handleContextMenuCommand = async (cmd: ContextMenuCommand) => {
    const command: GridCommand = { kind: GridCommandKind.CONTEXT_COMMAND, value: cmd };
    gridState = evolveAndReactDraggingState(gridState, command);
  };

  const stateToContextMenu = (state: GridState) => {
    if (state.kind !== GridStateKind.CONTEXT) {
      return undefined;
    }

    const clientRect = containerRef.getBoundingClientRect();
    const { x, y, placementId, placementKind, row } = state.source;
    const { rowKind } = row;

    const defaultLane =
      rowKind === 'audience'
        ? row.audienceId === DEFAULT_LANE
        : rowKind === 'stream'
        ? row.streamId === DEFAULT_LANE
        : false;

    // Offset so that default stream's context menu doesn't overflow
    const yOffset = defaultLane && rowKind === 'stream' ? 150 : 0;

    return {
      x: x - clientRect.x,
      y: y - clientRect.y - yOffset,
      rowKind,
      placementId,
      placementKind,
      defaultLane
    };
  };

  $: contextMenu = stateToContextMenu(gridState);

  const dispatchContextCommand = () => {};

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

  // Lanes
  $: default_audience_lane_index = grid.default_audience.row + 1;
  $: default_stream_lane_index = grid.default_stream.row - 1;
  $: lane_drop_target = gridState.kind === GridStateKind.LANE ? gridState.value.target : undefined;
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
    gridState.kind === GridStateKind.PLACEMENT ? gridState.value.target : undefined;

  // Flows
  $: linkingFlow = linkingFlowFromState(gridState);

  // Cursor

  let cursor_row = 0;
  onMount(() => {
    cursor_row = grid.timeline.row ?? 0;
  });
  let cursor_column = 0;

  $: cursor_cell = grid.cell_by_row_col(
    cursor_row,
    cursor_column,
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

  $: max_column = Math.max(grid.column_count, cursor_column + 10);

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
    cursor_row = Math.min(cursor_row + 1, grid.default_stream.row);
  };

  const navLeft = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_column = Math.max(cursor_column - 1, 0);
  };

  const navHome = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_row = grid.timeline.row;
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
    cursor_row = grid.default_audience.row;
  };

  const navBottom = (event: KeyboardEvent) => {
    event.preventDefault();
    cursor_row = grid.default_stream.row;
  };

  const placementDetailsAtCursor = (event: KeyboardEvent) => {
    event.preventDefault();
    if (cursor_cell?.placement) {
      dispatch('navigateToPlacementDetails', { placement: cursor_cell.placement.id });
    }
  };

  const importAtCursor = (event: KeyboardEvent) => {
    event.preventDefault();
    dispatch('navigateToImportJson', { column: cursor_column });
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
    },

    'Shift+Enter': placementDetailsAtCursor,
    'Control+i': importAtCursor
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
    const command: GridCommand = {
      kind: GridCommandKind.CURSOR_MOVE,
      value: { x: e.clientX - clientRect.x, y: e.clientY - clientRect.y }
    };

    gridState = evolveAndReactDraggingState(gridState, command);
  };

  $: allFlows = linkingFlow ? grid.flows.concat(linkingFlow) : grid.flows;
</script>

<svelte:window
  on:keydown={keyboardHandler}
  on:dragenter={handleOutOfBoundsDragEnter}
  on:dragend={handleOutOfBoundsDragEnd} />

<div class="overflow-auto z-[0] relative h-full w-full bg-gray-canvas dark:bg-dark-1">
  {#if contextMenu}
    <ContextMenu
      x={contextMenu.x}
      y={contextMenu.y}
      on:clickoutside={handleCloseContextMenu}>
      {#if !contextMenu.placementId}
        {#if contextMenu.rowKind === 'stream'}
          <ContextMenuItem>Add Event</ContextMenuItem>
        {:else if contextMenu.rowKind === 'audience'}
          <ContextMenuItem>Add Interface</ContextMenuItem>
        {:else if contextMenu.rowKind === 'timeline'}
          <ContextMenuItem>Add Read Model</ContextMenuItem>
          <ContextMenuItem>Add Command</ContextMenuItem>
        {/if}
      {:else if contextMenu.placementKind === 'event'}
        <ContextMenuItem
          on:click={() => handleContextMenuCommand(ContextMenuCommand.DeletePlacement)} >
          Delete Event
        </ContextMenuItem>
      {:else if contextMenu.placementKind === 'interface'}
        <ContextMenuItem
          on:click={() => handleContextMenuCommand(ContextMenuCommand.DeletePlacement)} >
          Delete Interface
        </ContextMenuItem>
      {:else if contextMenu.placementKind === 'read_model'}
        <ContextMenuItem
          on:click={() => handleContextMenuCommand(ContextMenuCommand.DeletePlacement)} >
          Delete Read Model</ContextMenuItem>
        {:else if contextMenu.placementKind === 'command'}
          <ContextMenuItem
            on:click={() => handleContextMenuCommand(ContextMenuCommand.DeletePlacement)} >
            Delete Command
          </ContextMenuItem>
        {/if}
        <ContextMenuDivider />
        <ContextMenuItem
          on:click={() => handleContextMenuCommand(ContextMenuCommand.InsertColumnLeft)} >
          Insert Column Left
        </ContextMenuItem>
        <ContextMenuItem
          on:click={() => handleContextMenuCommand(ContextMenuCommand.InsertColumnRight)} >
          Insert Column Right
        </ContextMenuItem>
        {#if contextMenu.rowKind !== 'audience' || !contextMenu.defaultLane}
          <ContextMenuItem
            on:click={() => handleContextMenuCommand(ContextMenuCommand.InsertLaneAbove)}>
            Insert Lane Above
          </ContextMenuItem>
        {/if}
        {#if contextMenu.rowKind !== 'stream' || !contextMenu.defaultLane}
          <ContextMenuItem
            on:click={() => handleContextMenuCommand(ContextMenuCommand.InsertLaneBelow)} >
            Insert Lane Below
          </ContextMenuItem>
        {/if}
        <ContextMenuDivider />
        <ContextMenuItem>Import Event Model JSON</ContextMenuItem>
    </ContextMenu>
  {/if}
  <div
    bind:this={containerRef}
    on:dragover={handleDragOver}
    class="grid relative w-max p-3 pl-5 justify-items-center items-center"
    style="grid-template-columns: repeat({max_column}, 9rem); grid-template-rows: repeat({grid.row_count}, 9rem);">
    <FlowCanvas flows={allFlows} />
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
      audience={grid.default_audience}
      {max_column} />

    {#each grid.audiences as audience (audience.id)}
      {@const targeted_lane =
    audience_drop_target?.index == audience.row ? audience_drop_target?.targetStatus : undefined}
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
      {audience}
      {max_column} />
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
  timeline={grid.timeline}
  targeted_lane={timeline_drop_target}
  targeted_cell={cell_drop_target && cell_drop_target.row.rowKind === 'timeline'
  ? { column: cell_drop_target.column, targetStatus: cell_drop_target.targetStatus }
  : undefined}
  {max_column} />

{#each grid.streams as stream (stream.id)}
  {@const targeted_lane =
stream_drop_target?.index == stream.index ? stream_drop_target?.targetStatus : undefined}
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
  {stream}
  {max_column} />
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
  stream={grid.default_stream}
  {max_column} />
{#if cursor_cell}
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
    cell={cursor_cell}
    mode={cursor_mode}
    target_status={cell_drop_target?.column &&
    cell_drop_target.column === cursor_column &&
    cell_drop_target?.row &&
    cell_drop_target.row.rowIndex == cursor_row
    ? cell_drop_target?.targetStatus
    : undefined} />
  {/if}
  {#if mode === 'disambiguating' && disambiguation}
    <TimelineDisambiguation
      name={disambiguation.name}
      index={disambiguation.index}
      top={disambiguation.top}
      left={disambiguation.left}
      on:define_and_place_command={handleDefineAndPlaceCommand}
      on:define_and_place_read_model={handleDefineAndPlaceReadModel} />
    {/if}
  </div>
</div>
