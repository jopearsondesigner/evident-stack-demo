<script lang="ts">
  import markdown from '../../utils/markdown.js';
  import type { DragEventHandler } from 'svelte/elements';
  import FlowPort from './FlowPort.svelte';
  import { createEventDispatcher } from 'svelte';
  import type { InterfaceConfig } from '../Grid.js';
  import Icon from '../../Icon.svelte';
  import JobGears from '../../icons/JobGears.svelte';

  export let is_cursor = false;
  export let id: string;
  $: dom_id = is_cursor ? `${id}-cursor` : id;

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

<div class="relative group w-full h-full" on:dragover={(e) => e.preventDefault()}>
  <FlowPort
    on:flow_drag_start={forward}
    position="bottom"
    type="interface"
    placement={id}
    placement_kind="interface"
    {column}
  />
  <FlowPort
    on:flow_drag_start={forward}
    position="right"
    type="interface"
    placement={id}
    placement_kind="interface"
    {column}
  />
  {#if config.kind == 'blank'}
    <div class="w-full h-full p-[1.375rem]">
      <div
        id={dom_id}
        draggable="true"
        on:dragstart={handleDragStart}
        class="interface w-full h-full p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-interface bg-gradient-to-b from-interfaceColor to-interfaceColor-dark border-2 border-interfaceColor rounded-[4px] outline outline-2 outline-gray-primary"
      >
        {name}
      </div>
    </div>
  {:else if config.kind == 'job'}
    <div class="w-full h-full p-[1.375rem]">
      <div
        id={dom_id}
        draggable="true"
        on:dragstart={handleDragStart}
        class="interface w-full h-full p-2 overflow-visible text-left text-node font-semibold leading-tight shadow-interface bg-gradient-to-b from-interfaceColor to-interfaceColor-dark border-2 border-interfaceColor rounded-[4px] outline outline-2 outline-gray-primary"
      >
        {name}
        <span class="flex h-auto items-center justify-center">
          <Icon
            class="flex-1 m-2 transition duration-200 ease-in cursor-default"
            name="job-gears"
            size={40}
            viewBox="0 0 34 34"
            iconColor="text-body-light dark:text-gray-brand-1"
            pathName={JobGears}
          />
        </span>
      </div>
    </div>
  {:else if config.kind == 'image'}
    <div
      id={dom_id}
      draggable="true"
      on:dragstart={handleDragStart}
      class="interface flex flex-col items-center w-full h-full p-1.5"
    >
      <h4 class="text-xs">{name}</h4>
      <img class="flex-1 min-h-0 mt-1 object-contain" src={config.url} alt={name} />
    </div>
  {:else if config.kind == 'figma'}
    <div class="w-full h-full p-[1.375rem]">
      <div
        id={dom_id}
        draggable="true"
        on:dragstart={handleDragStart}
        class="interface flex flex-col items-center w-full h-full p-1.5 overflow-visible text-left text-node font-semibold leading-tight shadow-interface bg-gradient-to-b from-interfaceColor to-interfaceColor-dark border-2 border-interfaceColor rounded-[4px] outline outline-2 outline-gray-primary"
      >
        {name}
        <div class="flex items-stretch h-full">
          <img
            class="min-h-0 m-2 h-[40px] self-end"
            src="/images/figma-logo.svg"
            alt="Figma Logo"
          />
        </div>
      </div>
    </div>
  {/if}
</div>
