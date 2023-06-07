<script lang="ts">
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';
  import { createEventDispatcher } from 'svelte';
  import FlowPort from './FlowPort.svelte';

  export let is_cursor = false;
  export let id: string;
  $: dom_id = is_cursor ? `${id}-cursor` : id;

  export let command: string;
  export let column: number;
  export let name: string;
  export let description = '';

  $: descriptionHTML = markdown(description);

  const dispatch = createEventDispatcher();
  const forward = (event: CustomEvent) => {
    dispatch(event.type, event.detail);
  }

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;

    if (transfer) {
      if (e.shiftKey) {
        transfer.effectAllowed = 'copy';
        dispatch('placement_drag_start', {
          placementId: id,
          placementKind: 'command',
          sourceEffect: 'DUPLICATE'
        });
      } else {
        transfer.effectAllowed = 'move';
        dispatch('placement_drag_start', {
          placementId: id,
          placementKind: 'command',
          sourceEffect: 'MOVE'
        });
      }
    }
  };
</script>

<div
  class="relative group w-full h-full p-[1.375rem]"
  on:dragover={(e) => e.preventDefault()}>
  <FlowPort on:flow_drag_start={forward} position="bottom" type="command" placement={id} placement_kind="command" {column} />
  <FlowPort on:flow_drag_start={forward} position="right" type="command" placement={id} placement_kind="command" {column} />
  <!-- TODO: tooltip interferes with link dragging -->
  <!-- <MaybeTooltip tip={descriptionHTML}> -->
  <div
    id={dom_id}
    draggable="true"
    on:dragstart={handleDragStart}
    class="command w-full h-full p-3 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-command-dark via-command to-command-light">
    {name}
  </div>
  <!-- </MaybeTooltip> -->
</div>
