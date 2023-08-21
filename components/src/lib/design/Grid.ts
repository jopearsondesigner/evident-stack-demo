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
  rename_lane: (kind: ReorderableLaneType, lane_id: string, name: string) => any;
  reorder_lane: (kind: ReorderableLaneType, lane_id: string, index: number) => any;
  remove_lane: (kind: ReorderableLaneType, lane_id: string) => any;
  add_lane: (kind: string, index: number, name: string) => any;
  insert_columns: (index: number, direction: string, count: number) => any;
  edit_description: (index: number, deletion_count: number, addition: string) => any;
  connect_flow: (
    source_placement_id_str: string,
    source_anchor_str: string | undefined,
    target_placement_id_str: string,
    target_anchor_str: string | undefined
  ) => any;
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
  edit_description: console.log,
  connect_flow: console.log
};

export type Flow = {
  id: string;
  to: FlowPort | FlowCursor;
  from: FlowPort;
  dashed?: boolean;
  color?: string;
  strokeWidth?: number;
};

export type FlowPort = {
  placement_id: string;
  anchor: FlowAnchor;
  kind: 'FlowPort';
};

export type FlowCursor = {
  x: number;
  y: number;
  kind: 'FlowCursor';
};

export enum FlowAnchor {
  None,
  Top,
  Left,
  Bottom,
  Right
}

// Adapted from grid.rs
export type InterfaceType = 'blank' | 'figma' | 'image' | 'job';
export type InterfaceConfig = {
  kind: InterfaceType;
  url?: string;
};

// Adapted from grid.rs
export type PlacementType = 'interface' | 'command' | 'event' | 'read_model';
export type Placement = {
  kind: PlacementType;
  id: string;
  component_id: string;
  index: number;
  name: string;
  description: string;
  interface_config?: InterfaceConfig;
};

// Adapted from grid.rs
export type CellType = 'interface' | 'timeline' | 'event';
export type Cell = {
  kind: CellType;
  row: number;
  column: number;
  placement?: Placement;
  audience?: string;
  stream?: string;
};

// Adapted from grid.rs
export type LaneType = 'default_audience' | 'audience' | 'timeline' | 'stream' | 'default_stream';
export type Lane = {
  kind: LaneType;
  id?: string;
  index?: number;
  row: number;
  name: string;
  cells: Cell[];
};

export type EventModelGrid = {
  id: string;
  name: string;
  description: string;
  column_count: number;
  row_count: number;
  default_audience: Lane;
  audiences: Lane[];
  timeline: Lane;
  streams: Lane[];
  default_stream: Lane;

  flows: Flow[];

  cell_by_row_col: (row: number, col: number) => Cell | undefined;
};

export type Disambiguation = { name: string; index: number; top: number; left: number } | null;

export const cellId = (col: number, row: number): string => {
  return `cell-${col}-${row}`;
};

export type ReorderableLaneType = 'audience' | 'stream';
export type GridMode =
  | 'loading'
  | 'navigation'
  | 'editing'
  | 'disambiguating'
  | 'linking'
  | 'modal';
export type CursorMode = 'editing' | 'navigation' | 'linking' | 'other';
export type DropTargetStatus = 'good' | 'bad';
export type LinkingFlowColor = 'red' | 'green' | 'black';
