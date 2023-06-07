<script lang="ts">
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';
  import FlowPort from './FlowPort.svelte';
  import { createEventDispatcher } from 'svelte';

  export let is_cursor = false;
  export let id: string;
  $: dom_id = is_cursor ? `${id}-cursor` : id;

  export let read_model: string;
  export let column: number;
  export let name: string;
  export let description = '';

  $: descriptionHTML = markdown(description);

  const dispatch = createEventDispatcher();
  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  };

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;

    if (transfer) {
      if (e.shiftKey) {
        transfer.effectAllowed = 'copy';
        dispatch('placement_drag_start', {
          placementId: id,
          placementKind: 'read_model',
          sourceEffect: 'DUPLICATE'
        });
      } else {
        transfer.effectAllowed = 'move';
        dispatch('placement_drag_start', {
          placementId: id,
          placementKind: 'read_model',
          sourceEffect: 'MOVE'
        });
      }
    }
  };
</script>

<div
  class="relative group w-full h-full p-[1.375rem]"
  on:dragover={(e) => e.preventDefault()}>
  <FlowPort on:flow_drag_start={forward} position="top" type="read_model" placement={id} placement_kind="read_model" {column} />
  <FlowPort on:flow_drag_start={forward} position="right" type="read_model" placement={id} placement_kind="read_model" {column} />
  <!-- TODO: tooltip interferes with link dragging -->
  <!-- <MaybeTooltip tip={descriptionHTML}> -->
  <div
    id={dom_id}
    draggable="true"
    on:dragstart={handleDragStart}
    class="read_model w-full h-full p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-read_model-dark via-read_model to-read_model-light">
    {name}
  </div>
  <!-- </MaybeTooltip> -->
</div>
