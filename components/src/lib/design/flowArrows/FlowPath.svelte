<script lang="ts">
  import { onDestroy } from 'svelte';
  import type { FlowPort } from '../Grid';
  import {
    bezierControlPoint,
    findMidPoint,
    makeMarkerId,
    makePathId,
    pointFromRect
  } from './util';

  export let id: string;
  export let to: FlowPort;
  export let from: FlowPort;
  export let color: string;
  export let strokeWidth: number;
  export let dashness: boolean;
  export let refreshTime: number;
  export let curveShapeFactor: number;

  type MaybeRect = DOMRect | undefined;

  const rectForPlacement = (id: string): DOMRect | undefined => {
    const element = document.getElementById(id);
    if (element) {
      return element.getBoundingClientRect();
    }
  };

  let toRect: MaybeRect = rectForPlacement(to.placement_id);
  let fromRect: MaybeRect = rectForPlacement(from.placement_id);

  const rectsUpdated = (current: DOMRect, next: DOMRect): boolean => {
    return (
      current.x !== next.x ||
      current.y !== next.y ||
      current.width !== next.width ||
      current.height !== next.height
    );
  };

  let updateLoop = setInterval(() => {
    const nextTo = rectForPlacement(to.placement_id);
    const nextFrom = rectForPlacement(from.placement_id);

    if (!nextTo || !nextFrom) {
      return;
    }

    if (!toRect || !fromRect) {
      toRect = nextTo;
      fromRect = nextFrom;
      update();
    } else if (rectsUpdated(toRect, nextTo) || rectsUpdated(fromRect, nextFrom)) {
      toRect = nextTo;
      fromRect = nextFrom;
      update();
    }
  }, refreshTime);

  let update = () => {
    console.info(`UPDATE: TO: ${to.placement_id}, FROM: ${from.placement_id}`);
    console.info('Bounding To: ', toRect);
    console.info('Bounding From: ', fromRect);
  };

  const pathGeometry = (toRect: MaybeRect, fromRect: MaybeRect): string | undefined => {
    if (!toRect || !fromRect) {
      return;
    }

    const toPoint = pointFromRect(toRect);
    const fromPoint = pointFromRect(fromRect);
    const midPoint = findMidPoint(toPoint, fromPoint);

    const fromBezPoint = bezierControlPoint(fromPoint, midPoint, from.anchor, curveShapeFactor);
    const toBezPoint = bezierControlPoint(toPoint, midPoint, to.anchor, curveShapeFactor);

    if (!fromBezPoint || !toBezPoint) {
      return;
    }

    return `M ${fromPoint.x} ${fromPoint.y}
        C ${fromBezPoint.x} ${fromBezPoint.y},
            ${toBezPoint.x} ${toBezPoint.y},
            ${toPoint.x} ${toPoint.y}`;
  };

  onDestroy(() => {
    clearInterval(updateLoop);
  });

  $: pathId = makePathId(id);
  $: markerId = makeMarkerId(id);
  $: pathCoords = pathGeometry(toRect, fromRect);
  $: dashArray = dashness ? 6 * strokeWidth : null;
</script>

{#if pathCoords}
  <path
    id={pathId}
    d={pathCoords}
    stroke={color}
    stroke-width={strokeWidth}
    stroke-dasharray={dashArray}
    fill="none"
    marker-end={`url(#${markerId})`}
  />
{/if}
