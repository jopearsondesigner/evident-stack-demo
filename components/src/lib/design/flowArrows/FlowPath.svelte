<script lang="ts">
  import { onDestroy } from 'svelte';
  import { FlowAnchor, type FlowCursor, type FlowPort } from '../Grid';
  import {
    findAnchorPoint,
    findBezierControlPoint,
    findMidPoint,
    makePathId,
    rectForPlacementId,
    rectsUpdated
  } from './util';

  export let id: string;
  export let to: FlowPort | FlowCursor;
  export let from: FlowPort;
  export let color: string;
  export let strokeWidth: number;
  export let dashness: boolean;
  export let refreshTime: number;
  export let curveShapeFactor: number;
  export let boundingParent: SVGSVGElement;
  export let markerSize: number;

  type MaybeRect = DOMRect | undefined;

  let toRect: MaybeRect = to.kind === 'FlowPort' ? rectForPlacementId(to.placement_id) : undefined;

  let fromRect: MaybeRect = rectForPlacementId(from.placement_id);

  console.warn("TO RAW: ->", to);
  console.warn("TO, FROM -> ", toRect, fromRect);

  $: fromAnchorPoint = fromRect
    ? findAnchorPoint(boundingParent, from.anchor, fromRect, 0, markerSize * 2)
    : undefined;

  $: toAnchorPoint =
    to.kind === 'FlowCursor'
      ? { x: to.x, y: to.y }
      : toRect
      ? findAnchorPoint(boundingParent, to.anchor, toRect, Math.abs(markerSize / 2))
      : undefined;

  $: midPoint =
    toAnchorPoint && fromAnchorPoint ? findMidPoint(toAnchorPoint, fromAnchorPoint) : undefined;

  $: fromBezPoint =
    fromAnchorPoint && midPoint
      ? findBezierControlPoint(fromAnchorPoint, midPoint, from.anchor, curveShapeFactor)
      : undefined;

  $: toBezPoint =
    toAnchorPoint && midPoint
      ? findBezierControlPoint(
          toAnchorPoint,
          midPoint,
          to.kind === 'FlowPort' ? to.anchor : FlowAnchor.Left,
          curveShapeFactor
        )
      : undefined;

  $: pathCoords =
    fromAnchorPoint && fromBezPoint && toBezPoint && toAnchorPoint
      ? `M ${fromAnchorPoint.x} ${fromAnchorPoint.y}
        C ${fromBezPoint.x} ${fromBezPoint.y},
            ${toBezPoint.x} ${toBezPoint.y},
            ${toAnchorPoint.x} ${toAnchorPoint.y}`
      : undefined;

  $: pathId = makePathId(id);
  $: dashArray = dashness ? 6 * strokeWidth : null;

  const updateLoop = setInterval(() => {
    const nextFrom = rectForPlacementId(from.placement_id);

    if (!fromRect) {
      fromRect = nextFrom;
    } else if (!nextFrom) {
      fromRect = undefined;
    } else if (rectsUpdated(fromRect, nextFrom)) {
      fromRect = nextFrom;
    }

    if (to.kind === 'FlowPort') {
      const nextTo = rectForPlacementId(to.placement_id);

      if (!toRect) {
        toRect = nextTo;
      } else if (!nextTo) {
        toRect = undefined;
      } else if (rectsUpdated(toRect, nextTo)) {
        toRect = nextTo;
      }
    }
  }, refreshTime);

  // const pathGeometry = (toRect: MaybeRect, fromRect: MaybeRect): string | undefined => {
  //   if (!toRect || !fromRect) {
  //     return;
  //   }

  //   const pathHeadOffset = Math.abs(markerSize / 2);
  //   const toAnchorPoint = anchorPoint(boundingParent, to.anchor, toRect, pathHeadOffset);
  //   const fromAnchorPoint = anchorPoint(boundingParent, from.anchor, fromRect, 0, markerSize * 2);
  //   const midPoint = findMidPoint(toAnchorPoint, fromAnchorPoint);
  //   const fromBezPoint = bezierControlPoint(
  //     fromAnchorPoint,
  //     midPoint,
  //     from.anchor,
  //     curveShapeFactor
  //   );
  //   const toBezPoint = bezierControlPoint(toAnchorPoint, midPoint, to.anchor, curveShapeFactor);

  //   if (!fromBezPoint || !toBezPoint) {
  //     return;
  //   }

  //   return `M ${fromAnchorPoint.x} ${fromAnchorPoint.y}
  //       C ${fromBezPoint.x} ${fromBezPoint.y},
  //           ${toBezPoint.x} ${toBezPoint.y},
  //           ${toAnchorPoint.x} ${toAnchorPoint.y}`;
  // };

  onDestroy(() => {
    clearInterval(updateLoop);
  });
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
