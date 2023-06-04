<script lang="ts">
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';
  import FlowPort from './FlowPort.svelte';
  import { createEventDispatcher } from 'svelte';
  import type { InterfaceConfig } from '../Grid.js';

  export let id: string;
  export let interface_id: string;
  export let column: number;

  export let config: InterfaceConfig;

  export let name: string;
  export let description: string;

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

<div class="relative group w-full h-full"
     on:dragover={(e) => e.preventDefault()}>
  <FlowPort
    on:flow_drag_start={forward}
    position="bottom"
    type="interface"
    placement={id}
    placement_kind="interface"
    {column} />
  <FlowPort
    on:flow_drag_start={forward}
    position="right"
    type="interface"
    placement={id}
    placement_kind="interface"
    {column} />
  <!-- TODO: tooltip interferes with link dragging -->
  <!-- <MaybeTooltip tip={descriptionHTML}> -->
  {#if config.kind == 'blank'}
    <div class="w-full h-full p-[1.375rem]">
      <div
        {id}
        draggable="true"
        on:dragstart={handleDragStart}
        class="interface w-full h-full p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-interface bg-gradient-to-b from-interfaceColor to-interfaceColor-dark border-2 border-interfaceColor rounded-[4px] outline outline-2 outline-gray-primary">
        {name}
      </div>
    </div>
  {:else if config.kind == 'job'}
    <!-- TODO: temporary -->
    <div
      {id}
      draggable="true"
      on:dragstart={handleDragStart}
      class="interface w-full h-full p-1.5 overflow-visible text-left text-node font-semibold leading-tight shadow-interface bg-gradient-to-b from-interfaceColor to-interfaceColor-dark border-2 border-interfaceColor rounded-[4px] outline outline-2 outline-gray-primary">
      {name} with gear icon!
    </div>
  {:else if config.kind == 'image'}
    <div
      {id}
      draggable="true"
      on:dragstart={handleDragStart}
      class="interface flex flex-col items-center w-full h-full p-1.5">
      <h4 class="text-xs">{name}</h4>
      <img class="flex-1 min-h-0 object-contain" src={config.url} alt={name} />
    </div>
  {:else if config.kind == 'figma'}
    <div
      {id}
      draggable="true"
      on:dragstart={handleDragStart}
      class="interface flex flex-col items-center w-full h-full p-1.5">
      {name} with Figma embed at {config.url}
    </div>
  {/if}
  <!-- </MaybeTooltip> -->
</div>
