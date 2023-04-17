<script lang="ts">
  import { createKeybindingsHandler } from '../../vendor/tinykeys/tinykeys';
  import { tick, createEventDispatcher } from 'svelte';
  import type { CursorMode, ItemAtCursor } from '../Grid';
  import Interface from './Interface.svelte';
  import Command from './Command.svelte';
  import Event from './Event.svelte';
  import ReadModel from './ReadModel.svelte';
  import EmptyCell from './EmptyCell.svelte';

  export let row: number;
  export let column: number;
  export let mode: CursorMode;
  export let item: ItemAtCursor;

  $: gridRow = row + 1;
  $: gridColumn = column + 1;

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
      if (item.placement) {
        if (item.placement.name != name) {
          dispatch('rename_placement', { name, placement: item.placement.id });
        } else {
          cancelEditing(e);
        }
      } else if (item.type === 'interface') {
        dispatch('define_and_place_interface', { name, index: column, ...item });
      } else if (item.type === 'timeline') {
        let rect = form.getBoundingClientRect();
        dispatch('disambiguate_timeline_definition_and_placement', {
          name,
          left: rect.left,
          top: rect.top,
          index: column,
          ...item
        });
      } else if (item.type === 'event') {
        dispatch('define_and_place_event', { name, index: column, ...item });
      }
    }
  };

  const removePlacement: EventListener = (event) => {
    event.preventDefault();
    let placement = item.placement?.id;
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
</script>

<svelte:window on:keydown={keyboardHandler} />

{#if mode === 'editing'}
  <div
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
        value={item.placement?.name || ''}
        bind:this={input}
      />
    </form>
  </div>
{:else}
  <div
    bind:this={element}
    on:click={beginEditing}
    class="cursor z-20 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300"
    style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};"
  >
    {#if item.placement}
      {#if item.type == 'interface'}
        <Interface
          id={item.placement.id}
          interface_id={item.placement.interface}
          {column}
          name={item.placement.name}
          description={item.placement.description}
          on:connect_flow={forward}
        />
      {:else if item.type == 'timeline'}
        {#if item.placement.kind == 'command'}
          <Command
            id={item.placement.id}
            command={item.placement.component}
            {column}
            name={item.placement.name}
            description={item.placement.description}
            on:connect_flow={forward}
          />
        {:else if item.placement.kind == 'readModel'}
          <ReadModel
            id={item.placement.id}
            readModel={item.placement.component}
            {column}
            name={item.placement.name}
            description={item.placement.description}
            on:connect_flow={forward}
          />
        {/if}
      {:else if item.type == 'event'}
        <Event
          id={item.placement.id}
          event={item.placement.event}
          {column}
          name={item.placement.name}
          description={item.placement.description}
          on:connect_flow={forward}
        />
      {/if}
    {:else}
      <EmptyCell
        {column}
        kind={item.type}
        lane={item.audience || item.stream}
        on:move_interface_placement={forward}
        on:move_timeline_placement={forward}
        on:move_event_placement={forward}
        on:duplicate_interface_placement={forward}
        on:duplicate_timeline_placement={forward}
        on:duplicate_event_placement={forward}
      />
    {/if}
  </div>
{/if}
