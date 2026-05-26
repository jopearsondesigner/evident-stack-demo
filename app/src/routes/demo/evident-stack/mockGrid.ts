import type { EventModelGrid, Cell, Lane, Placement, Flow } from '$components/design/Grid';
import { default_decider, FlowAnchor } from '$components/design/Grid';

const columnCount = 16;

const placement = (
  kind: Placement['kind'],
  id: string,
  index: number,
  name: string,
  description: string,
  interfaceKind: 'blank' | 'figma' | 'job' = 'blank'
): Placement => ({
  kind,
  id,
  component_id: id.replace('placement-', ''),
  index,
  name,
  description,
  interface_config:
    kind === 'interface'
      ? {
          kind: interfaceKind
        }
      : undefined
});

const placements = {
  ownerApp: placement(
    'interface',
    'placement-interface-owner-app',
    1,
    'Owner App',
    'Mobile interface used by vehicle owners.',
    'figma'
  ),
  riderApp: placement(
    'interface',
    'placement-interface-rider-app',
    2,
    'Rider App',
    'Mobile interface used by riders.',
    'figma'
  ),
  vehiclePortal: placement(
    'interface',
    'placement-interface-vehicle-portal',
    4,
    'Vehicle Portal',
    'Administrative interface for fleet and vehicle management.',
    'figma'
  ),
  addVehicle: placement(
    'command',
    'placement-command-add-vehicle',
    2,
    'Add Vehicle',
    'Command triggered when an owner adds a vehicle.'
  ),
  vehicleAdded: placement(
    'event',
    'placement-event-vehicle-added',
    3,
    'Vehicle Added',
    'Event recorded after a vehicle is added.'
  ),
  vehicleProfile: placement(
    'read_model',
    'placement-read-model-vehicle-profile',
    4,
    'Vehicle Profile',
    'Read model showing vehicle details and availability.'
  ),
  requestRide: placement(
    'command',
    'placement-command-request-ride',
    6,
    'Request Ride',
    'Command triggered when a rider requests a ride.'
  ),
  rideRequested: placement(
    'event',
    'placement-event-ride-requested',
    7,
    'Ride Requested',
    'Event recorded when a ride request is created.'
  ),
  rideStatus: placement(
    'read_model',
    'placement-read-model-ride-status',
    8,
    'Ride Status',
    'Read model representing the rider-facing ride state.'
  ),
  assignVehicle: placement(
    'command',
    'placement-command-assign-vehicle',
    10,
    'Assign Vehicle',
    'Command assigning an available vehicle to a ride.'
  ),
  vehicleAssigned: placement(
    'event',
    'placement-event-vehicle-assigned',
    11,
    'Vehicle Assigned',
    'Event recorded when a vehicle is assigned.'
  )
};

const makeCell = (
  kind: Cell['kind'],
  row: number,
  column: number,
  placement?: Placement,
  laneId?: string
): Cell => ({
  kind,
  row,
  column,
  placement,
  audience: kind === 'interface' ? laneId : undefined,
  stream: kind === 'event' ? laneId : undefined
});

const createCells = (
  kind: Cell['kind'],
  row: number,
  count: number,
  cellPlacements: Record<number, Placement>,
  laneId?: string
): Cell[] =>
  Array.from({ length: count }, (_, column) =>
    makeCell(kind, row, column, cellPlacements[column], laneId)
  );

const defaultAudience: Lane = {
  kind: 'default_audience',
  row: 0,
  name: 'Interfaces',
  cells: createCells(
    'interface',
    0,
    columnCount,
    {
      1: placements.ownerApp,
      2: placements.riderApp,
      4: placements.vehiclePortal
    },
    'default'
  )
};

const ownerAudience: Lane = {
  kind: 'audience',
  id: 'audience-owner',
  index: 1,
  row: 1,
  name: 'Owner',
  cells: createCells('interface', 1, columnCount, {}, 'audience-owner')
};

const riderAudience: Lane = {
  kind: 'audience',
  id: 'audience-rider',
  index: 2,
  row: 2,
  name: 'Rider',
  cells: createCells('interface', 2, columnCount, {}, 'audience-rider')
};

const timeline: Lane = {
  kind: 'timeline',
  row: 3,
  name: 'Timeline',
  cells: createCells('timeline', 3, columnCount, {
    2: placements.addVehicle,
    4: placements.vehicleProfile,
    6: placements.requestRide,
    8: placements.rideStatus,
    10: placements.assignVehicle
  })
};

const vehicleStream: Lane = {
  kind: 'stream',
  id: 'stream-vehicle',
  index: 4,
  row: 4,
  name: 'Vehicle',
  cells: createCells(
    'event',
    4,
    columnCount,
    {
      3: placements.vehicleAdded,
      11: placements.vehicleAssigned
    },
    'stream-vehicle'
  )
};

const rideStream: Lane = {
  kind: 'stream',
  id: 'stream-ride',
  index: 5,
  row: 5,
  name: 'Ride',
  cells: createCells(
    'event',
    5,
    columnCount,
    {
      7: placements.rideRequested
    },
    'stream-ride'
  )
};

const defaultStream: Lane = {
  kind: 'default_stream',
  row: 6,
  name: 'Events',
  cells: createCells('event', 6, columnCount, {}, 'default')
};

const lanes = [
  defaultAudience,
  ownerAudience,
  riderAudience,
  timeline,
  vehicleStream,
  rideStream,
  defaultStream
];

const allCells = lanes.flatMap((lane) => lane.cells);

const flows: Flow[] = [
  {
    id: 'flow-add-vehicle-to-vehicle-added',
    from: {
      kind: 'FlowPort',
      placement_id: placements.addVehicle.id,
      anchor: FlowAnchor.Right
    },
    to: {
      kind: 'FlowPort',
      placement_id: placements.vehicleAdded.id,
      anchor: FlowAnchor.Left
    }
  },
  {
    id: 'flow-request-ride-to-ride-requested',
    from: {
      kind: 'FlowPort',
      placement_id: placements.requestRide.id,
      anchor: FlowAnchor.Right
    },
    to: {
      kind: 'FlowPort',
      placement_id: placements.rideRequested.id,
      anchor: FlowAnchor.Left
    }
  },
  {
    id: 'flow-assign-vehicle-to-vehicle-assigned',
    from: {
      kind: 'FlowPort',
      placement_id: placements.assignVehicle.id,
      anchor: FlowAnchor.Right
    },
    to: {
      kind: 'FlowPort',
      placement_id: placements.vehicleAssigned.id,
      anchor: FlowAnchor.Left
    }
  }
];

export const mockGrid: EventModelGrid = {
  id: 'demo-grid-autonomo',
  name: 'Autonomo Mobile iOS App',
  description: 'Portfolio-safe demo of the Evident Stack event modeling interface.',
  column_count: columnCount,
  row_count: lanes.length,
  default_audience: defaultAudience,
  audiences: [ownerAudience, riderAudience],
  timeline,
  streams: [vehicleStream, rideStream],
  default_stream: defaultStream,
  flows,
  cell_by_row_col: (row: number, col: number) =>
    allCells.find((cell) => cell.row === row && cell.column === col)
};

export const mockDecider = default_decider;
