<script lang="ts">
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';
  import FlowPort from './FlowPort.svelte';
  import { createEventDispatcher } from 'svelte';

  export let id: string;
  export let interface_id: string;
  export let column: number;

  type InterfaceConfig =
    | { type: 'blank' }
    | {
        type: 'figma';
        url: string;
        width?: number | undefined | null;
        height?: number | undefined | null;
      }
    | {
        type: 'image';
        url: string;
        width?: number | undefined | null;
        height?: number | undefined | null;
      }
    | { type: 'job' };

  export let config: InterfaceConfig = { type: 'blank' };
  export let name: string;
  export let description = '';

  $: descriptionHTML = markdown(description);

  const dispatch = createEventDispatcher();

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    // let transfer = e.dataTransfer;
    // if (transfer) {
    //   transfer.setData('interface', id);
    //   if (e.shiftKey) {
    //     transfer.effectAllowed = 'copy';
    //   } else {
    //     transfer.effectAllowed = 'move';
    //   }
    // }
    dispatch('placement_drag_start', {
      placementId: interface_id,
      placementType: 'interface'
    });
  };

  // Linking
  let drop_target = false;

  const handleDragEnter: DragEventHandler<HTMLDivElement> = (e) => {
    // let transfer = e.dataTransfer;
    // if (transfer) {
    //   if (transfer.effectAllowed == 'link' && transfer.types.includes('readModel')) {
    //     e.preventDefault();
    //     transfer.dropEffect = 'link';
    //     drop_target = true;
    //   }
    // }
    e.stopPropagation();
  };

  const handleDragLeave: DragEventHandler<HTMLDivElement> = (e) => {
    // drop_target = false; // TODO: conditionally change style (cursor maybe?) while linking
    e.stopPropagation();
  };

  const handleDragDrop: DragEventHandler<HTMLDivElement> = (e) => {
    // handleDragLeave(e);
    // let transfer = e.dataTransfer;
    // if (transfer) {
    //   if (transfer.effectAllowed == 'link') {
    //     let fromData = transfer.getData('readModel');
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
  <FlowPort position="bottom" type="interface" placement={id} {column} />
  <FlowPort position="right" type="interface" placement={id} {column} />
  <!-- TODO: tooltip interferes with link dragging -->
  <!-- <MaybeTooltip tip={descriptionHTML}> -->
  <div
    {id}
    draggable="true"
    on:dragstart={handleDragStart}
    class="interface m-[1.4375rem] w-24 h-24 p-1.5 overflow-visible text-left text-node font-semibold leading-tight shadow-interface bg-gradient-to-b from-interfaceColor to-interfaceColor-dark border-2 border-interfaceColor rounded-[4px] outline outline-2 outline-gray-primary"
  >
    {name}
  </div>
  <!-- </MaybeTooltip> -->
</div>
