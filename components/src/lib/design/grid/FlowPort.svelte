<script lang="ts">
  import type { DragEventHandler } from 'svelte/elements';
  import type { PlacementType } from '../Grid';
  import { createEventDispatcher } from 'svelte';

  type Position = 'top' | 'bottom' | 'right';
  export let position: Position;
  export let type: PlacementType;
  export let placement: string;
  export let placement_kind: PlacementType;
  export let column: number;

  const dispatch = createEventDispatcher();

  const classesByPosition = (position: Position) => {
    if (position == 'top') {
      return 'border-x-transparent border-x-[5px] border-b-gray-brand-4 border-b-[10px]';
    } else if (position == 'bottom') {
      return 'border-x-transparent border-x-[5px] border-t-gray-brand-4 border-t-[10px]';
    } else if (position == 'right') {
      return 'border-y-transparent border-y-[5px] border-l-gray-brand-4 border-l-[10px]';
    }
  };

  $: right = position == 'right' ? '0' : 'calc(50% - 8px)';
  $: top = position == 'top' ? '0' : position == 'right' ? 'calc(50% - 8px)' : null;
  $: bottom = position == 'bottom' ? '0' : null;

  // Drag/drop
  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    dispatch("flow_drag_start", {
      position,
      placement: {
        placementId: placement,
        placementKind: placement_kind
      }
    })
  };
</script>

<div
  style:right
  style:top
  style:bottom
  class="absolute hidden group-hover:block {classesByPosition(position)}"
  draggable={true}
  on:dragstart={handleDragStart}
  on:dragover={e => e.preventDefault()}
  on:dragenter={e => e.preventDefault()}
  on:dragend={e => e.preventDefault()}
/>
