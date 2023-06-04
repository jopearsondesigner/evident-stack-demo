<script lang="ts">
  import type { Flow } from '../Grid';
  import FlowMarker from './FlowMarker.svelte';
  import FlowPath from './FlowPath.svelte';

  export let flows: Array<Flow> = [];
  export let baseColor: string = '#657B83';
  export let strokeWidth: number = 1;
  export let refreshTime: number = 16;
  export let curveShapeFactor: number = 0.1;
  export let markerSize: number = 8;
  let svgClass: string = 'absolute top-0 left-0 bottom-0 right-0 z-0';
  export { svgClass as class };

  let containerRef: SVGSVGElement;

  $: pathConfig = {
    color: baseColor,
    strokeWidth,
    refreshTime,
    curveShapeFactor,
    markerSize,
    boundingParent: containerRef
  };
  $: markerConfig = { color: baseColor, markerSize };
</script>

<svg
  bind:this={containerRef}
  class={svgClass}
  width="100%"
  height="100%"
  preserveAspectRatio="none"
  xmlns="http://www.w3.org/2000/svg">
  <defs>
    <FlowMarker {...markerConfig} />
  </defs>
  {#each flows as { id, to, from, dashed, color, strokeWidth }}
    <FlowPath
      {...{
        id,
        to,
        from,
        dashed,
        ...pathConfig,
        color: color || pathConfig.color,
        strokeWidth: strokeWidth || pathConfig.strokeWidth
      }}
    />
  {/each}
</svg>
