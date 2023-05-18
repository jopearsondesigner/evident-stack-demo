<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import type { Flow } from '../Grid';
  import FlowMarker from './FlowMarker.svelte';
  import FlowPath from './FlowPath.svelte';

  export let flows: Array<Flow> = [];
  export let color: string = '#657B83';
  export let strokeWidth: number = 1;
  export let refreshTime: number = 16;
  export let curveShapeFactor: number = 0.1;
  export let markerSize: number = 8;
  let svgClass: string = 'absolute z-0';
  export { svgClass as class };

  let containerRef: SVGSVGElement;

  $: pathConfig = {
    color,
    strokeWidth,
    refreshTime,
    curveShapeFactor,
    markerSize,
    boundingParent: containerRef
  };
  $: markerConfig = { color, markerSize };
</script>

<svg
  bind:this={containerRef}
  class={svgClass}
  width="100%"
  height="100%"
  preserveAspectRatio="none"
  xmlns="http://www.w3.org/2000/svg"
>
  <defs>
    <FlowMarker {...markerConfig} />
  </defs>
  {#each flows as { id, to, from, dashed }}
    <FlowPath {...{ id, to, from, dashed, ...pathConfig }} />
  {/each}
</svg>

