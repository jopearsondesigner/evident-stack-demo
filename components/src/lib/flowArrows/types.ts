import type { FlowAnchor } from "$lib/design/Grid";

export const RECT_ATTRIBUTES_TO_COMPARE = ['x', 'y', 'width', 'height'];

export type Id = string;

export interface Point {
    x: number;
    y: number;
}

// export enum Anchor {
//     TOP,
//     LEFT,
//     BOTTOM,
//     RIGHT
// }

export interface PathCoordinates {
    to: Point;
    from: Point;
    mid: Point;
    fromBezPoint: Point;
    toBezPoint: Point;
}

export interface PathProps {
    from: Id,
    to: Id,
    fromAnchor: FlowAnchor,
    toAnchor: FlowAnchor,
    color?: string,
    strokeWidth?: number,
    curveFactor?: number,
    offsetMarket?: number,
    markerSize?: number,
    dashness?: boolean,
    debug?: boolean,
    refreshTime?: number,
}