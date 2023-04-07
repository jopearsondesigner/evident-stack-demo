<script lang="ts">
  import { createKeybindingsHandler } from "../../vendor/tinykeys/tinykeys";
  import { tick, createEventDispatcher } from "svelte";
  import type { CursorMode, ItemAtCursor } from "../Grid";

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
  }

  $: if (mode === 'editing') {
    focusInput()
  }

  // Scroll Into View

  let element: HTMLDivElement;

  const scrollIntoView = async () => {
    await tick()
    element.scrollIntoView({behavior: "smooth", block: "nearest", inline: "center"})
  }

  $: if (element && gridRow > 0 && gridColumn > 0) {
    scrollIntoView()
  }

  // Dispatch

  const dispatch = createEventDispatcher();

  const beginEditing: EventListener = (event) => {
    event.preventDefault();
    dispatch('begin_editing')
  }

  const cancelEditing: EventListener = (event) => {
    event.preventDefault();
    dispatch('cancel_editing')
  }

  const handleSubmit = (e: SubmitEvent) => {
    let form = e.target as HTMLFormElement
    let data = new FormData(form);
    let name = data.get("name")?.toString();
    if (name) {
      if (item.empty === 'interface') {
        dispatch('define_and_place_interface', {name, index: column, ...item})
      } else if (item.empty === 'timeline') {
        let rect = form.getBoundingClientRect()
        dispatch('disambiguate_timeline_definition_and_placement', {name, left: rect.left, top: rect.top, index: column, ...item})
      } else if (item.empty === 'event') {
        dispatch('define_and_place_event', {name, index: column, ...item})
      } else {
        if (item.placement.name != name) {
          dispatch('rename_placement', {name, placement: item.placement.id})
        } else {
          cancelEditing(e)
        }
      }
    }
  }

  const removePlacement: EventListener = (event) => {
    event.preventDefault();
    let placement = item.placement?.id;
    if (placement) {
      dispatch('remove_placement', {placement});
    }
  }

  const navigationKeyboardHandler = createKeybindingsHandler({
    "Delete": removePlacement,
    "Backspace": removePlacement
  })

  const editingKeyboardHandler = createKeybindingsHandler({
    "Escape": cancelEditing,
    "Control+g": cancelEditing
  })

  const keyboardHandler: EventListener = (e) => {
    if (mode === 'editing') {
      editingKeyboardHandler(e)
    } else if (mode === 'navigation') {
      navigationKeyboardHandler(e)
    }
  }
</script>

<svelte:window on:keydown={keyboardHandler}/>

{#if mode === 'editing'}
  <div
    bind:this={element}
    class="cursor z-20 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300 bg-gray-canvas dark:bg-dark-1"
    style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};">
    <form class="w-full, h-full" on:submit|preventDefault={handleSubmit}>
      <input name="name" class="w-full" type="text" value={item.placement?.name || ''} bind:this={input} />
    </form>
  </div>
{:else}
  <div
    on:click={beginEditing}
    bind:this={element}
    class="cursor z-20 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300"
    style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};" />
  {/if}
