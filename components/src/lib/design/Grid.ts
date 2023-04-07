export type Decider = {
  define_and_place_interface: (name: string, index: number, audience: string | undefined) => any,
  define_and_place_command: (name: string, index: number) => any,
  define_and_place_event: (name: string, index: number, stream: string | undefined) => any,
  define_and_place_read_model: (name: string, index: number) => any,
  delete_model: () => any,
  import_json: (json_bytes: Uint8Array, offset: number) => any,
  remove_placement: (placement: string) => any,
  rename_placement: (placement: string, name: string) => any,
}

export const default_decider: Decider = {
  define_and_place_interface: console.log,
  define_and_place_command: console.log,
  define_and_place_event: console.log,
  define_and_place_read_model: console.log,
  delete_model: console.log,
  import_json: console.log,
  remove_placement: console.log,
  rename_placement: console.log,
}

export type Flow = {
  id: string,
  to: FlowPort,
  from: FlowPort,
}

export type FlowPort = {
  placement_id: string,
  anchor: FlowAnchor
}

export enum FlowAnchor {
  None,
  Top,
  Left,
  Bottom,
  Right,
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

export type PlacementCell = {placement: InterfacePlacement | TimelinePlacement | EventPlacement,
                             empty?: false, audience?: null, stream?: null}
export type EmptyInterfaceCell = { placement: null, empty: 'interface', audience?: string, stream?: null }
export type EmptyTimelineCell = { placement: null, empty: 'timeline', audience?: null, stream?: null }
export type EmptyEventCell = { placement: null, empty: 'event', stream?: string, audience?: null }

export type ItemAtCursor = PlacementCell | EmptyInterfaceCell | EmptyTimelineCell | EmptyEventCell

export const itemAtCursor = (
  row: number,
  column: number,
  default_audience: Array<InterfacePlacement>,
  audiences: Array<Audience>,
  timeline: Array<TimelinePlacement>,
  streams: Array<Stream>,
  default_stream: Array<EventPlacement>
): ItemAtCursor => {
  if (row === 0) {
    let placement = default_audience[column];
    return placement ? {placement} : { empty: 'interface', placement: null };
  } else if (row - 1 < audiences.length) {
    let audience = audiences[row - 1];
    let placement = audience?.placements[column];
    return placement? {placement} : { empty: 'interface', audience: audience?.id, placement: null };
  } else if (row === audiences.length + 1) {
    let placement = timeline[column]
    return placement ? {placement} : { empty: 'timeline', placement: null };
  } else if (row - 1 - audiences.length - 1 < streams.length) {
    let stream = streams[row - 1 - audiences.length - 1]
    let placement = stream?.placements[column]
    return placement ? {placement} : { empty: 'event', stream: stream?.id, placement: null };
  } else if (row === 1 + audiences.length + 1 + streams.length) {
    let placement = default_stream[column]
    return placement ? {placement} : { empty: 'event', placement: null };
  }
  throw new Error("No valid item at cursor!");
}

export type Disambiguation = {name: string, index: number, top: number, left: number} | null

export const placementOrEmptyCellId = (placement: { id: string } | null | undefined, col: number, row: number): string => {
  return (placement && placement.id) || `empty-${col}-${row}`;
}

export type GridMode = 'loading' | 'navigation' | 'editing' | 'disambiguating' | 'linking';
export type CursorMode = 'editing' | 'navigation' | 'linking' | 'other';
