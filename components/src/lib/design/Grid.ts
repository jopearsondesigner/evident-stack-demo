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

export const placementOrEmptyCellId = (placement: { id: string } | null | undefined, col: number, row: number): string => {
  return (placement && placement.id) || `empty-${col}-${row}`;
}

export const placementByRowColumn = (row: number,
                                     column: number,
                                     default_audience: Array<InterfacePlacement>,
                                     audiences: Array<Audience>,
                                     timeline: Array<TimelinePlacement>,
                                     streams: Array<Stream>,
                                     default_stream: Array<EventPlacement>): InterfacePlacement | TimelinePlacement | EventPlacement | null => {
  if (row === 0) {
    return default_audience[column];
  } else if (row - 1 < audiences.length) {
    return audiences[row - 1].placements[column];
  } else if (row === audiences.length + 1) {
    return timeline[column];
  } else if (row - 1 - audiences.length - 1 < streams.length) {
    return streams[row - 1 - audiences.length - 1].placements[column];
  } else if (row === 1 + audiences.length + 1 + streams.length) {
    return default_stream[column];
  } else {
    return null;
  }
}
