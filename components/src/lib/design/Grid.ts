export enum FlowAnchor {
    None,
    Top,
    Left,
    Bottom,
    Right,
}

export type FlowPort = {
  placementId: string,
  anchor: FlowAnchor
}

export type Flow = {
  to: FlowPort,
  from: FlowPort
}
export type InterfacePlacement = {
  id: string,
  interface: string,
  name: string,
  description: string,
  // TODO: supported placement types/config here
  kind: string
};

export type Audience = {
  id?: string,
  name?: string,
  placements: Array<InterfacePlacement>
}

export type TimelinePlacement = {
  id: string,
  component: string,
  kind: ('command' | 'readModel'),
  name: string,
  description: string
}

export type EventPlacement = {
  id: string,
  event: string,
  name: string,
  description: string
}

export type Stream = {
  id?: string,
  name?: string,
  placements: Array<EventPlacement>
}

// Cell IDs

export const placementOrEmptyCellId = (placement: { id: string } | null | undefined, col: number, row: number): string => {
  return (placement && placement.id) || `empty-${col}-${row}`;
}
