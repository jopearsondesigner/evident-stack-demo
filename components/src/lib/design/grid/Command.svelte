<script lang="ts">
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';
  import { createEventDispatcher } from 'svelte';
  import FlowPort from './FlowPort.svelte';

  export let id: string;
  export let command: string;
  export let column: number;
  export let name: string;
  export let description = '';

  $: descriptionHTML = markdown(description);

  const dispatch = createEventDispatcher();

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;

    if (transfer) {
      if (e.shiftKey) {
        transfer.effectAllowed = 'copy';
        dispatch('placement_drag_start', {
          placementId: id,
          placementType: 'command',
          sourceEffect: 'DUPLICATE'
        });
      } else {
        transfer.effectAllowed = 'move';
        dispatch('placement_drag_start', {
          placementId: id,
          placementType: 'command',
          sourceEffect: 'MOVE'
        });
      }
    }
  };

  // Linking
  // const dispatch = createEventDispatcher();
  // let drop_target = false;

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    // let transfer = e.dataTransfer;
    // if (transfer) {
    //   if (
    //     (transfer.effectAllowed == 'link' && transfer.types.includes('interface')) ||
    //     transfer.types.includes('event')
    //   ) {
    //     e.preventDefault();
    //     transfer.dropEffect = 'link';
    //     drop_target = true;
    //   }
    // }
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    // drop_target = false; // TODO: conditionally change style (cursor maybe?) while linking
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
    // handleDragLeave(e);
    // let transfer = e.dataTransfer;
    // if (transfer) {
    //   if (transfer.effectAllowed == 'link') {
    //     let fromData = transfer.getData('interface') || transfer.getData('event');
    //     if (fromData) {
    //       let from = JSON.parse(fromData);
    //       transfer.dropEffect = 'link';
    //       dispatch('connect_flow', { from: from, to: id });
    //     }
    //   }
    // }
  };
</script>

<div
  class="relative group"
  on:dragenter={handleDragEnter}
  on:dragover={(e) => e.preventDefault()}
  on:dragleave={handleDragLeave}
  on:drop={handleDragDrop}
>
  <FlowPort position="bottom" type="command" placement={id} {column} />
  <FlowPort position="right" type="command" placement={id} {column} />
  <!-- TODO: tooltip interferes with link dragging -->
  <!-- <MaybeTooltip tip={descriptionHTML}> -->
  <div
    {id}
    draggable="true"
    on:dragstart={handleDragStart}
    class="command m-[1.375rem] w-[6.125rem] h-[6.125rem] p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-command-dark via-command to-command-light"
  >
    {name}
  </div>
  <!-- </MaybeTooltip> -->
</div>
