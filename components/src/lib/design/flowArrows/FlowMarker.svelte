<script lang="ts">
  import { makeMarkerId } from './util';

  export let id: string;
  export let color: string;
  export let markerSize: number;

  type MarkerGeometry = {
    refX: number;
    refY: number;
    markerWidth: number;
    markerHeight: number;
    viewBox: string;
    path: string;
  };

  const markerGeometry = (size: number): MarkerGeometry => {
    const halfSize = Math.ceil(size / 2);
    return {
      refX: halfSize,
      refY: 0,
      markerWidth: size,
      markerHeight: size,
      viewBox: `0 -${halfSize} ${size} ${size}`,
      path: `M0,-${halfSize} L${size},0L0,${halfSize}`
    };
  };

  $: geometry = markerGeometry(markerSize);
  $: markerId = makeMarkerId(id);
</script>

<marker
  id={markerId}
  viewBox={geometry.viewBox}
  refX={geometry.refX}
  refY={geometry.refY}
  markerWidth={geometry.markerWidth}
  markerHeight={geometry.markerHeight}
  orient="auto" fill={color}>
  <path d={geometry.path} />
</marker>
