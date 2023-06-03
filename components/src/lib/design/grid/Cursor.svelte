<script lang="ts">
  import { createKeybindingsHandler } from '../../vendor/tinykeys/tinykeys';
  import { tick, createEventDispatcher } from 'svelte';
  import type { CursorMode, DropTargetStatus, Cell } from '../Grid';
  import Interface from './Interface.svelte';
  import Command from './Command.svelte';
  import Event from './Event.svelte';
  import ReadModel from './ReadModel.svelte';
  import EmptyCell from './EmptyCell.svelte';
  import type { DragEventHandler, MouseEventHandler } from 'svelte/elements';

  export let row: number;
  export let column: number;
  export let mode: CursorMode;
  export let cell: Cell;
  export let target_status: DropTargetStatus | undefined;

  $: gridRow = row + 1;
  $: gridColumn = column + 1;
  $: maybePlacement = cell.placement?.id;
  $: maybePlacementKind = cell.placement?.id
    ? cell.kind === 'interface'
      ? 'interface'
      : cell.kind === 'event'
      ? 'event'
      : cell.kind === 'timeline'
      ? cell.placement?.kind
      : undefined
    : undefined;
  $: rowTarget = {
    rowIndex: gridRow - 1,
    audienceId: cell.audience,
    streamId: cell.stream,
    rowKind: cell.kind
  };

  $: good_target = target_status == 'good';
  $: bad_target = target_status == 'bad';

  // Input Focus on Edit

  let input: HTMLInputElement;

  const focusInput = async () => {
    await tick();
    input.focus();
  };

  $: if (mode === 'editing') {
    focusInput();
  }

  // Scroll Into View

  let element: HTMLDivElement;

  const scrollIntoView = async () => {
    await tick();
    element.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });
  };

  $: if (element && gridRow > 0 && gridColumn > 0) {
    scrollIntoView();
  }

  // Dispatch

  const dispatch = createEventDispatcher();

  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  };

  const beginEditing: EventListener = (event) => {
    event.preventDefault();
    dispatch('begin_editing');
  };

  const cancelEditing: EventListener = (event) => {
    event.preventDefault();
    dispatch('cancel_editing');
  };

  const handleSubmit = (e: SubmitEvent) => {
    let form = e.target as HTMLFormElement;
    let data = new FormData(form);
    let name = data.get('name')?.toString();
    if (name) {
      if (cell.placement) {
        if (cell.placement.name != name) {
          dispatch('rename_placement', { name, placement: cell.placement.id });
        } else {
          cancelEditing(e);
        }
      } else if (cell.kind === 'interface') {
        dispatch('define_and_place_interface', { name, index: column, ...cell });
      } else if (cell.kind === 'timeline') {
        let rect = form.getBoundingClientRect();
        dispatch('disambiguate_timeline_definition_and_placement', {
          name,
          left: rect.left,
          top: rect.top,
          index: column,
          ...cell
        });
      } else if (cell.kind === 'event') {
        dispatch('define_and_place_event', { name, index: column, ...cell });
      }
    }
  };

  const removePlacement: EventListener = (event) => {
    event.preventDefault();
    let placement = cell.placement?.id;
    if (placement) {
      dispatch('remove_placement', { placement });
    }
  };

  const navigationKeyboardHandler = createKeybindingsHandler({
    Delete: removePlacement,
    Backspace: removePlacement
  });

  const editingKeyboardHandler = createKeybindingsHandler({
    Escape: cancelEditing,
    'Control+g': cancelEditing
  });

  const keyboardHandler: EventListener = (e) => {
    if (mode === 'editing') {
      editingKeyboardHandler(e);
    } else if (mode === 'navigation') {
      navigationKeyboardHandler(e);
    }
  };

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();
    dispatch('cell_drag_enter', {
      column,
      row: rowTarget,
      placementId: maybePlacement,
      placementKind: maybePlacementKind
    });
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    e.stopPropagation();
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (_e) => {
    dispatch('cell_drag_drop');
  };

  const handleRightClick: MouseEventHandler<HTMLDivElement> = (e) => {
    dispatch('open_context_menu', {
      x: e.clientX,
      y: e.clientY,
      column,
      row: rowTarget,
      placementId: maybePlacement,
      placementKind: maybePlacementKind
    });
  }
</script>

<svelte:window on:keydown={keyboardHandler} />

{#if mode === 'editing'}
  <div
    on:dragenter={handleDragEnter}
    on:dragover={(e) => {
      e.preventDefault();
    }}
    on:dragleave={handleDragLeave}
    on:drop={handleDragDrop}
    bind:this={element}
    class="cursor z-20 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300 bg-gray-canvas dark:bg-dark-1"
    style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
  >
    <form
      class="w-full h-full flex justify-center items-center"
      on:submit|preventDefault={handleSubmit}
    >
      <input
        name="name"
        class="text-sm text-body dark:text-body-dark m-1 focus:border focus:ring-focus focus:border-focus focus-visible:border-0 focus-visible:outline-0 focus-visible:ring-focus focus-visible:ring-2 bg-transparent"
        type="text"
        value={cell.placement?.name || ''}
        bind:this={input}
      />
    </form>
  </div>
{:else}
  <div
    on:contextmenu|preventDefault={handleRightClick}
    on:dragenter={handleDragEnter}
    on:dragover={(e) => {
      e.preventDefault();
    }}
    on:dragleave={handleDragLeave}
    on:drop={handleDragDrop}
    bind:this={element}
    on:click={beginEditing}
    class:bg-emerald-200={good_target}
    class:bg-rose-400={bad_target}
    class="cursor z-20 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300"
    style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};" >
    {#if cell.placement}
      {#if cell.kind == 'interface'}
        <Interface
          id={cell.placement.id}
          interface_id={cell.placement.component_id}
          {column}
          name={cell.placement.name}
          description={cell.placement.description}
          on:placement_drag_start={forward}
          on:flow_drag_start={forward}
          on:connect_flow={forward} />
      {:else if cell.kind == 'timeline'}
        {#if cell.placement.kind == 'command'}
          <Command
            id={cell.placement.id}
            command={cell.placement.component_id}
            {column}
            name={cell.placement.name}
            description={cell.placement.description}
            on:placement_drag_start={forward}
            on:flow_drag_start={forward}
            on:connect_flow={forward} />
        {:else if cell.placement.kind == 'read_model'}
          <ReadModel
            id={cell.placement.id}
            read_model={cell.placement.component_id}
            {column}
            name={cell.placement.name}
            description={cell.placement.description}
            on:placement_drag_start={forward}
            on:flow_drag_start={forward}
            on:connect_flow={forward} />
        {/if}
      {:else if cell.kind == 'event'}
        <Event
          id={cell.placement.id}
          event={cell.placement.component_id}
          {column}
          name={cell.placement.name}
          description={cell.placement.description}
          on:placement_drag_start={forward}
          on:flow_drag_start={forward}
          on:connect_flow={forward} />
      {/if}
    {:else}
      <EmptyCell
        {column}
        kind={cell.kind}
        lane={cell.audience || cell.stream}
        on:move_interface_placement={forward}
        on:move_timeline_placement={forward}
        on:move_event_placement={forward}
        on:duplicate_interface_placement={forward}
        on:duplicate_timeline_placement={forward}
        on:duplicate_event_placement={forward} />
    {/if}
  </div>
{/if}
