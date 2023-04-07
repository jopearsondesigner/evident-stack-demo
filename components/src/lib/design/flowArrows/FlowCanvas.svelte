<script lang="ts">
  import type { Flow } from '../Grid';
  import FlowMarker from './FlowMarker.svelte';
  import FlowPath from './FlowPath.svelte';

  export let flows: Array<Flow> = [];
  export let color: string = 'black';
  export let strokeWidth: number = 1;
  export let dashness: boolean = false;
  export let refreshTime: number = 16;
  export let curveShapeFactor: number = 0.1;
  export let markerSize: number = 10;

  $: pathConfig = { color, strokeWidth, dashness, refreshTime, curveShapeFactor };
  $: markerConfig = { color, markerSize };
</script>

<svg class="absolute" width="100%" height="100%" preserveAspectRatio="none" xmlns="http://www.w3.org/2000/svg">
  <defs>
    {#each flows as { id }}
      <FlowMarker {...{ ...markerConfig, id }} />
    {/each}
  </defs>
  {#each flows as { id, to, from }}
    <FlowPath {...{id, to, from, ...pathConfig}} />
  {/each}
</svg>
