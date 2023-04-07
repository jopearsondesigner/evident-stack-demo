<script lang="ts">
  import { createKeybindingsHandler } from "../../vendor/tinykeys/tinykeys";
  import { tick, createEventDispatcher } from "svelte";
  import type { ItemAtCursor } from "../Grid";

  export let row: number;
  export let column: number;
  export let editing: boolean;
  export let item: ItemAtCursor;

  $: gridRow = row + 1;
  $: gridColumn = column + 1;

  let input: HTMLInputElement;

  const focusInput = async () => {
    await tick();
    input.focus();
  }

  $: if (editing) {
    focusInput()
  }

  let element: HTMLDivElement;

  const scrollIntoView = async () => {
    await tick()
    element.scrollIntoView({behavior: "smooth", block: "nearest", inline: "center"})
  }

  $: if (element && gridRow > 0 && gridColumn > 0) {
    scrollIntoView()
  }

  const dispatch = createEventDispatcher();

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
        // TODO: disambiguation
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

  const editingKeyboardHandler = createKeybindingsHandler({
    "Escape": cancelEditing,
    "Control+g": cancelEditing
  })

  const keyboardHandler: EventListener = (e) => {
    if (editing) {
      editingKeyboardHandler(e)
    }
  }

  // TODO: clickaway listener cancels edit
</script>

<svelte:window on:keydown={keyboardHandler}/>

{#if editing}
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
    bind:this={element}
    class="cursor z-20 self-stretch w-full h-full transition duration-200 ease-in border-2 border-cyan-300"
    style="grid-row: {gridRow} / {gridRow}; grid-column: {gridColumn} / {gridColumn};" />
  {/if}
