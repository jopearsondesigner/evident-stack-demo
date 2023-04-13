<script lang="ts">
  import type { DragEventHandler } from "svelte/elements";
  import type { PlacementType } from "../Grid";

  type Position =  'top' | 'bottom' | 'right';
  export let position: Position;
  export let type: PlacementType;
  export let placement: string;
  export let column: number;

  const classesByPosition = (position: Position) => {
    if (position == 'top') {
      return "border-x-transparent border-x-8 border-b-slate-400 border-b-16"
    } else if (position == 'bottom') {
      return "border-x-transparent border-x-8 border-t-slate-400 border-t-16"
    } else if (position == 'right') {
      return "border-y-transparent border-y-8 border-l-slate-400 border-l-16"
    }
  }

  $: right  = position == 'right'  ? "0" : "calc(50% - 8px)";
  $: top    = position == 'top'    ? "0" : position == 'right' ? "calc(50% - 8px)" : null;
  $: bottom = position == 'bottom' ? "0" : null;

  // Drag/drop

  const handleDragStart: DragEventHandler<HTMLDivElement> = (e) => {
    let transfer = e.dataTransfer;
    if (transfer) {
      transfer.effectAllowed = 'link';
      transfer.setData(type, JSON.stringify({placement, position}));
    }
  }

  const handleDragEnd: DragEventHandler<HTMLDivElement> = (_e) => {
    // TODO: show linking flow
  }
</script>

<div style:right style:top style:bottom
     class="absolute hidden group-hover:block {classesByPosition(position)}"
     draggable={true}
     on:dragstart={handleDragStart}
     on:dragend={handleDragEnd} />
