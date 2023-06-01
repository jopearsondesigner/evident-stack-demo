export type Decider = {
  define_and_place_interface: (name: string, index: number, audience: string | undefined) => any;
  define_and_place_command: (name: string, index: number) => any;
  define_and_place_event: (name: string, index: number, stream: string | undefined) => any;
  define_and_place_read_model: (name: string, index: number) => any;
  delete_model: () => any;
  duplicate_interface_placement: (
    placement_id: string,
    index: number,
    audience: string | undefined
  ) => any;
  duplicate_timeline_placement: (placement_id: string, index: number) => any;
  duplicate_event_placement: (
    placement_id: string,
    index: number,
    stream: string | undefined
  ) => any;
  import_json: (json_bytes: Uint8Array, offset: number) => any;
  move_interface_placement: (
    placement_id: string,
    index: number,
    audience: string | undefined
  ) => any;
  move_timeline_placement: (placement_id: string, index: number) => any;
  move_event_placement: (placement_id: string, index: number, stream: string | undefined) => any;
  remove_placement: (placement: string) => any;
  rename_placement: (placement: string, name: string) => any;
  rename_lane: (kind: LaneKind, lane_id: string, name: string) => any;
  reorder_lane: (kind: LaneKind, lane_id: string, index: number) => any;
  remove_lane: (kind: LaneKind, lane_id: string) => any;
  add_lane: (kind: string, index: number, name: string) => any
  insert_columns: (index: number, direction: string, count: number) => any;
  add_to_description: (index: number, addition: string) => any;
  delete_from_description: (index: number, count: number) => any;
  connect_flow: (source_placement_id_str: string, source_anchor_str: string | undefined, target_placement_id_str: string, target_anchor_str: string | undefined) => any;
};

export const default_decider: Decider = {
  define_and_place_interface: console.log,
  define_and_place_command: console.log,
  define_and_place_event: console.log,
  define_and_place_read_model: console.log,
  delete_model: console.log,
  duplicate_interface_placement: console.log,
  duplicate_timeline_placement: console.log,
  duplicate_event_placement: console.log,
  import_json: console.log,
  move_interface_placement: console.log,
  move_timeline_placement: console.log,
  move_event_placement: console.log,
  remove_placement: console.log,
  rename_placement: console.log,
  rename_lane: console.log,
  reorder_lane: console.log,
  remove_lane: console.log,
  add_lane: console.log,
  insert_columns: console.log,
  add_to_description: console.log,
  delete_from_description: console.log,
  connect_flow: console.log
};

export type Flow = {
  id: string;
  to: FlowPort | FlowCursor;
  from: FlowPort;
  dashed?: boolean,
  color?: string,
  strokeWidth?: number,
};

export type FlowPort = {
  placement_id: string,
  anchor: FlowAnchor,
  kind: "FlowPort",
};

export type FlowCursor = {
  x: number,
  y: number,
  kind: "FlowCursor",
}

export enum FlowAnchor {
  None,
  Top,
  Left,
  Bottom,
  Right
}

export type Audience = {
  id?: string;
  name?: string;
  placements: Array<InterfacePlacement>;
};

export type InterfacePlacement = {
  id: string;
  interface: string;
  name: string;
  description: string;
  // TODO: supported placement types/config here
  kind: string;
};

export type TimelinePlacement = {
  id: string;
  component: string;
  kind: 'command' | 'readModel';
  name: string;
  description: string;
};

export type EventPlacement = {
  id: string;
  event: string;
  name: string;
  description: string;
};

export type Stream = {
  id?: string;
  name?: string;
  placements: Array<EventPlacement>;
};

export type PlacementType = 'interface' | 'command' | 'event' | 'readModel';
export type CellType = 'interface' | 'timeline' | 'event';

export type InterfacePlacementCell = { type: 'interface'; placement: InterfacePlacement, audience?: string, lane_index: number };
export type TimelinePlacementCell = { type: 'timeline'; placement: TimelinePlacement, lane_index: undefined };
export type EventPlacementCell = { type: 'event'; placement: EventPlacement, stream?: string, lane_index: number };
export type EmptyCell = {
  type: CellType;
  placement?: undefined;
  audience?: string;
  stream?: string;
  lane_index?: number;
};

export type Disambiguation = { name: string; index: number; top: number; left: number } | null;

export const placementOrEmptyCellId = (
  placement: { id: string } | null | undefined,
  col: number,
  row: number
): string => {
  return (placement && placement.id) || `empty-${col}-${row}`;
};

export type GridMode = 'loading' | 'navigation' | 'editing' | 'disambiguating' | 'linking';
export type CursorMode = 'editing' | 'navigation' | 'linking' | 'other';
export type LaneKind = 'audience' | 'stream';
export type DropTargetStatus = 'good' | 'bad';
export type LinkingFlowColor = "red" | "green" | "black";
