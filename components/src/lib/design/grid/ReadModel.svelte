<script lang="ts">
  import MaybeTooltip from '../../utils/MaybeTooltip.svelte';
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';
  import FlowPort from './FlowPort.svelte';
  import { createEventDispatcher } from 'svelte';

  export let id: string;
  export let readModel: string;
  export let column: number;
  export let name: string;
  export let description = '';

  $: descriptionHTML = markdown(description);

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer) {
      transfer.setData('timeline', id);
      if (e.shiftKey) {
        transfer.effectAllowed = 'copy';
      } else {
        transfer.effectAllowed = 'move';
      }
    }
  };

  // Linking
  const dispatch = createEventDispatcher();
  let drop_target = false; // TODO: conditionally change style (cursor maybe?) while linking

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer) {
      if (transfer.effectAllowed == 'link' && transfer.types.includes('event')) {
        e.preventDefault();
        transfer.dropEffect = 'link';
        drop_target = true;
      }
    }
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (_e) => {
    drop_target = false;
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
    handleDragLeave(e);
    let transfer = e.dataTransfer;
    if (transfer) {
      if (transfer.effectAllowed == 'link') {
        let fromData = transfer.getData('event');
        if (fromData) {
          let from = JSON.parse(fromData);
          transfer.dropEffect = 'link';
          dispatch('connect_flow', { from: from, to: id });
        }
      }
    }
  };
</script>

<div
  class="relative group"
  on:dragenter={handleDragEnter}
  on:dragover={(e) => e.preventDefault()}
  on:dragleave={handleDragLeave}
  on:drop={handleDragDrop}
>
  <FlowPort position="top" type="readModel" placement={id} {column} />
  <FlowPort position="right" type="readModel" placement={id} {column} />
  <!-- TODO: tooltip interferes with link dragging -->
  <!-- <MaybeTooltip tip={descriptionHTML}> -->
  <div
    {id}
    draggable="true"
    on:dragstart={handleDragStart}
    class="readModel m-[1.4375rem] w-[6.125rem] h-[6.125rem] p-3 overflow-visible text-left text-node font-semibold leading-tight shadow-placement bg-gradient-to-b from-readModel to-readModel-light"
  >
    {name}
  </div>
  <!-- </MaybeTooltip> -->
</div>
