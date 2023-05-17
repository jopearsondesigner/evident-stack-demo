<script lang="ts">
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';
  import FlowPort from './FlowPort.svelte';
  import { createEventDispatcher } from 'svelte';

  export let id: string;
  export let event: string;
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
          placementKind: 'event',
          sourceEffect: 'DUPLICATE'
        });
      } else {
        transfer.effectAllowed = 'move';
        dispatch('placement_drag_start', {
          placementId: id,
          placementKind: 'event',
          sourceEffect: 'MOVE'
        });
      }
    }
  };
</script>

<div
  class="relative group"
  on:dragover={(e) => e.preventDefault()}
>
  <FlowPort on:flow_drag_start={forward} position="top" type="event" placement={id} placement_kind="event" {column} />
  <FlowPort on:flow_drag_start={forward} position="right" type="event" placement={id} placement_kind="event"{column} />
  <!-- TODO: tooltip interferes with link dragging -->
  <!-- <MaybeTooltip tip={descriptionHTML}> -->
  <div
    {id}
    draggable="true"
    on:dragstart={handleDragStart}
    class="event col-border ring-white dark:ring-gray-brand-1 m-[1.375rem] w-[6.125rem] h-[6.125rem] p-3 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-event-dark via-event to-event-light"
  >
    {name}
  </div>
  <!-- </MaybeTooltip> -->
</div>
