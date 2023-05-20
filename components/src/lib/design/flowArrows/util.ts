import { FlowAnchor } from "../Grid";
import type { Point } from "./types";

export const findAnchorPoint = (boundingParent: SVGSVGElement, anchor: FlowAnchor, target: DOMRect, pathHeadOffset: number = 0, offsetMarker: number = 0): Point => {
    let x = target.left;
    let y = target.top;

    switch (anchor) {
      case FlowAnchor.Top:
        x = target.left + Math.ceil((target.right - target.left) / 2) + offsetMarker;
        y = target.top - pathHeadOffset;
        break;
      case FlowAnchor.Left:
        x = target.left - pathHeadOffset;
        y = target.top + Math.ceil((target.bottom - target.top) / 2) + offsetMarker;
        break;
      case FlowAnchor.Bottom:
        x = target.left + Math.ceil((target.right - target.left) / 2) + offsetMarker;
        y = target.bottom + pathHeadOffset;
        break;
      case FlowAnchor.Right:
        x = target.right + pathHeadOffset;
        y = target.top + Math.ceil((target.bottom - target.top) / 2) + offsetMarker;
        break;
    }

    const container = boundingParent.getBoundingClientRect();
    return { x: x - container.x, y: y - container.y };
}

export const findMidPoint = (to: Point, from: Point): Point => {
    return {
        x: Math.abs((to.x + from.x) / 2),
        y: Math.abs((to.y + from.y) / 2),
    }
}

export const findBezierControlPoint = (origin: Point, mid: Point, anchor: FlowAnchor, curveShapeFactor: number): Point | null => {
  switch (anchor) {
    case FlowAnchor.Top:
      return { ...origin, y: mid.y - curveShapeFactor * (mid.y / 4) };
    case FlowAnchor.Bottom:
      return { ...origin, y: mid.y + curveShapeFactor * (mid.y / 4) };
    case FlowAnchor.Left:
      return { ...origin, x: mid.x - curveShapeFactor * (mid.x / 4) };
    case FlowAnchor.Right:
      return { ...origin, x: mid.x + curveShapeFactor * (mid.x / 4) };
  }
  return null;
};

export const pointFromRect = ({ x, y }: DOMRect): Point => ({
    x, y
})

export const makePathId = (id: string): string => `flow-path-${id}`

export const rectForPlacementId = (id: string): DOMRect | undefined => {
  const element = document.getElementById(id);
  if (element) {
    return element.getBoundingClientRect();
  }
};

export const rectsUpdated = (current: DOMRect, next: DOMRect): boolean => {
  return (
    current.x !== next.x ||
    current.y !== next.y ||
    current.width !== next.width ||
    current.height !== next.height
  );
};
