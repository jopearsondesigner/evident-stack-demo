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
          placementKind: 'interface',
          sourceEffect: 'DUPLICATE'
        });
      } else {
        transfer.effectAllowed = 'move';
        dispatch('placement_drag_start', {
          placementId: id,
          placementKind: 'interface',
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
  <FlowPort on:flow_drag_start={forward} position="bottom" type="interface" placement={id} placement_kind="interface" {column} />
  <FlowPort on:flow_drag_start={forward} position="right" type="interface" placement={id} placement_kind="interface" {column} />
  <!-- TODO: tooltip interferes with link dragging -->
  <!-- <MaybeTooltip tip={descriptionHTML}> -->
  <div
    {id}
    draggable="true"
    on:dragstart={handleDragStart}
    class="interface m-[1.4375rem] w-24 h-24 p-1.5 overflow-visible text-left text-node font-semibold leading-tight shadow-interface bg-gradient-to-b from-interfaceColor to-interfaceColor-dark border-2 border-interfaceColor rounded-[4px] outline outline-2 outline-gray-primary"
  >
    {name + id}
  </div>
  <!-- </MaybeTooltip> -->
</div>
