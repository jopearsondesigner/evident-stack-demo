<script lang="ts">
  import { onDestroy } from 'svelte';
  import type { FlowPort } from '../Grid';
  import { anchorPoint, bezierControlPoint, findMidPoint, makePathId } from './util';

  export let id: string;
  export let to: FlowPort;
  export let from: FlowPort;
  export let color: string;
  export let strokeWidth: number;
  export let dashness: boolean;
  export let refreshTime: number;
  export let curveShapeFactor: number;
  export let boundingParent: SVGSVGElement;
  export let markerSize: number;

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
    } else if (rectsUpdated(toRect, nextTo) || rectsUpdated(fromRect, nextFrom)) {
      toRect = nextTo;
      fromRect = nextFrom;
    }
  }, refreshTime);

  const pathGeometry = (toRect: MaybeRect, fromRect: MaybeRect): string | undefined => {
    if (!toRect || !fromRect) {
      return;
    }

    const pathHeadOffset = Math.abs(markerSize / 2);
    const toAnchorPoint = anchorPoint(boundingParent, to.anchor, toRect, pathHeadOffset);
    const fromAnchorPoint = anchorPoint(boundingParent, from.anchor, fromRect, 0, markerSize * 2);
    const midPoint = findMidPoint(toAnchorPoint, fromAnchorPoint);
    const fromBezPoint = bezierControlPoint(
      fromAnchorPoint,
      midPoint,
      from.anchor,
      curveShapeFactor
    );
    const toBezPoint = bezierControlPoint(toAnchorPoint, midPoint, to.anchor, curveShapeFactor);

    if (!fromBezPoint || !toBezPoint) {
      return;
    }

    return `M ${fromAnchorPoint.x} ${fromAnchorPoint.y}
        C ${fromBezPoint.x} ${fromBezPoint.y},
            ${toBezPoint.x} ${toBezPoint.y},
            ${toAnchorPoint.x} ${toAnchorPoint.y}`;
  };

  onDestroy(() => {
    clearInterval(updateLoop);
  });

  $: pathId = makePathId(id);
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
    marker-end={'url(#flow-marker-arrow)'}
  />
{/if}
