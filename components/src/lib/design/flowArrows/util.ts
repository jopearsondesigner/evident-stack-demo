import { FlowAnchor } from "../Grid";
import type { Point } from "./types";

export const bezierControlPoint = (origin: Point, mid: Point, anchor: FlowAnchor, curveShapeFactor: number): Point | null => {
  switch (anchor) {
    case FlowAnchor.Top:
      return { ...origin, y: mid.y - curveShapeFactor * (mid.y / 4) };
    case FlowAnchor.Bottom:
      return { ...origin, y: mid.y + curveShapeFactor * (mid.y / 4) };
    case FlowAnchor.Left:
      return { ...origin, x: mid.x - curveShapeFactor * (mid.x / 4) };
    case FlowAnchor.Right:
      return { ...origin, x: mid.x + curveShapeFactor * (mid.x / 4) };
    default:
      return null;
  }
};

export const findMidPoint = (to: Point, from: Point): Point => {
    return {
        x: Math.abs((to.x + from.x) / 2),
        y: Math.abs((to.y + from.y) / 2),
    }
}

export const pointFromRect = ({ x, y }: DOMRect): Point => ({
    x, y
})

export const makeMarkerId = (id: string): string => `flow-marker-${id}`;
export const makePathId = (id: string): string => `flow-path-${id}`