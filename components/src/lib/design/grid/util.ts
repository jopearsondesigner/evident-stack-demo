import { FlowAnchor, type Decider, type DropTargetStatus, type Flow, type FlowCursor, type FlowPort, type LaneKind, type LinkingFlowColor, type PlacementType } from "../Grid";

// Types
export interface WithDropTargetStatus { targetStatus: DropTargetStatus}
export interface WithSourceEffect { sourceEffect: SourceEffect }
export interface WithRowIndex { rowIndex: number }

export const DEFAULT_LANE = "DEFAULT_LANE";
export type DefaultLane = "DEFAULT_LANE";
export type SourceEffect = "MOVE" | "DUPLICATE";
export type RowKind = LaneKind | "timeline";

export interface LaneSource {
    laneId: string,
    laneKind: LaneKind
}

export interface PlacementSource {
    placementId: string,
    placementKind: PlacementType,
}

export interface FlowPortSource {
    placement: PlacementSource,
    position: FlowAnchor 
}

export type AudienceTarget = {
    audienceId: string | DefaultLane,
    laneIndex: number,
    rowKind: "audience"
};

export type StreamTarget = {
    streamId: string | DefaultLane,
    laneIndex: number,
    rowKind: "stream"
};

export type TimelineTarget = {
    rowKind: "timeline"
};

export type RowTarget = | AudienceTarget | StreamTarget | TimelineTarget;

export interface CellTarget {
    column: number,
    row: RowTarget & WithRowIndex,
    // Todo: Nest both placementId and placementKind under one optional - we either have both or neither
    placementId?: string,
    placementKind?: PlacementType,
}

export interface CursorTarget {
    x: number,
    y: number
}

export enum DraggingStateKind {
    LANE,
    PLACEMENT,
    FLOW,
    NONE
}

export type DraggingState =
    | { kind: DraggingStateKind.LANE,
        value: {
            source: LaneSource,
            target?: RowTarget & WithDropTargetStatus} }
    | { kind: DraggingStateKind.PLACEMENT,
        value: {
            source: PlacementSource & WithSourceEffect,
            target?: CellTarget & WithDropTargetStatus } }
    | { kind: DraggingStateKind.FLOW,
        value: {
            source: FlowPortSource,
            cursor?: CursorTarget,
            target?: (CellTarget & WithDropTargetStatus)
        }}
    | { kind: DraggingStateKind.NONE };

export enum DraggingCommandKind {
    LANE_DRAG_START,
    LANE_DRAG_ENTER,
    LANE_DRAG_DROP,
    PLACEMENT_DRAG_START,
    CELL_DRAG_ENTER,
    CELL_DRAG_DROP,
    FLOW_PORT_DRAG_START,
    OUT_OF_BOUNDS_DRAG_ENTER,
    OUT_OF_BOUNDS_DRAG_END,
    CURSOR_MOVE,
}

export type DragCommand =
    | { kind: DraggingCommandKind.LANE_DRAG_START,
        value: LaneSource }
    | { kind: DraggingCommandKind.LANE_DRAG_ENTER,
        value: RowTarget }
    | { kind: DraggingCommandKind.LANE_DRAG_DROP }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_START,
        value: PlacementSource & WithSourceEffect }
    | { kind: DraggingCommandKind.CELL_DRAG_ENTER,
        value: CellTarget }
    | { kind: DraggingCommandKind.CELL_DRAG_DROP }
    | { kind: DraggingCommandKind.FLOW_PORT_DRAG_START,
        value: FlowPortSource }
    | { kind:  DraggingCommandKind.OUT_OF_BOUNDS_DRAG_ENTER }
    | { kind:  DraggingCommandKind.OUT_OF_BOUNDS_DRAG_END }
    | { kind: DraggingCommandKind.CURSOR_MOVE,
        value: CursorTarget }

export function buildEvolveAndReact(reactionDecider: Decider): (s: DraggingState, c: DragCommand) => DraggingState {
    return (state: DraggingState, command: DragCommand) => {
        switch (command.kind) {
                case DraggingCommandKind.LANE_DRAG_START: {
                    let { laneId, laneKind: laneType } = command.value

                    return {
                        kind: DraggingStateKind.LANE,
                        value: {
                            source: { laneId, laneKind: laneType } ,
                        }
                    };
                }
                    
                case DraggingCommandKind.LANE_DRAG_ENTER: {
                    const target = command.value;

                    switch (state.kind) {
                        case DraggingStateKind.LANE:
                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        ...target,
                                        targetStatus: laneTargetStatus(state.value.source, target)
                                    }
                                    
                                }
                            };
                        default:
                            return state;
                    }
                } 
                case DraggingCommandKind.LANE_DRAG_DROP: {
                    switch (state.kind) {
                        case DraggingStateKind.LANE:
                            if (state.value.target?.targetStatus == "good" && state.value.target?.rowKind !== "timeline") {
                                const { source, target } = state.value;
                                const { laneId, laneKind: laneType } = source;
                                const { laneIndex } = target;
                                reactionDecider.reorder_lane(laneType, laneId, laneIndex);
                            }
                        default:
                    }

                    return { kind: DraggingStateKind.NONE }
                };
                case DraggingCommandKind.PLACEMENT_DRAG_START:
                    return {
                        kind: DraggingStateKind.PLACEMENT,
                        value: {
                            source: command.value
                        }
                    };
                case DraggingCommandKind.CELL_DRAG_ENTER: {
                    const { row } = command.value;

                    switch (state.kind) {
                        case DraggingStateKind.LANE: {
                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        ...row,
                                        targetStatus: laneTargetStatus(state.value.source, row)
                                    }
                                }
                            }
                        }
                        case DraggingStateKind.PLACEMENT: {
                            const target = command.value;

                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        ...command.value,                                        
                                        targetStatus: placementTargetStatus(state.value.source, target)
                                    }
                                }
                            }
                        }
                        case DraggingStateKind.FLOW: {
                            const target = command.value;

                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        ...command.value,                                        
                                        targetStatus: flowTargetStatus(state.value.source, target)
                                    }
                                }
                            }
                        }
                            
                        case DraggingStateKind.NONE:
                            return state;
                    }
                }
                case DraggingCommandKind.CELL_DRAG_DROP: {
                    switch (state.kind) {
                        case DraggingStateKind.LANE: {
                            if (state.value.target && state.value.target.rowKind !== "timeline" && state.value.target.targetStatus === "good") {
                                const { source, target } = state.value;
                                const { laneId, laneKind: laneType } = source;
                                const { laneIndex } = target;

                                reactionDecider.reorder_lane(laneType, laneId, laneIndex);
                            }
                            return { kind: DraggingStateKind.NONE }
                        }
                        case DraggingStateKind.PLACEMENT: {
                            if (state.value.target && state.value.target.targetStatus === "good") {
                                const { source, target } = state.value;
                                const { placementId, placementKind, sourceEffect } = source;
                                const { column, row } = target

                                const laneId = laneIdFilterDefault(rowtargetToLaneId(row));

                                switch (sourceEffect) {
                                    case "MOVE":
                                        switch (placementKind) {
                                            case "command":
                                                reactionDecider.move_timeline_placement(placementId, column);
                                                break;
                                            case "readModel":
                                                reactionDecider.move_timeline_placement(placementId, column);
                                                break;
                                            case "event":
                                                reactionDecider.move_event_placement(placementId, column, laneId);
                                                break;
                                            case "interface":
                                                reactionDecider.move_interface_placement(placementId, column, laneId);
                                                break;
                                        }
                                    break;
                                    case "DUPLICATE":
                                        switch (placementKind) {
                                            case "command":
                                                reactionDecider.duplicate_timeline_placement(placementId, column);
                                                break;
                                            case "readModel":
                                                reactionDecider.duplicate_timeline_placement(placementId, column);
                                                break;
                                            case "event":
                                                reactionDecider.duplicate_event_placement(placementId, column, laneId);
                                                break;
                                            case "interface":
                                                reactionDecider.duplicate_interface_placement(placementId, column, laneId);
                                                break;
                                        }
                                    break;
                                }

                            }

                            return { kind: DraggingStateKind.NONE };
                        }
                        case DraggingStateKind.FLOW: {
                            if (state.value.target && state.value.target.targetStatus === "good") {
                                const { source, target } = state.value;
                                if (target.placementId && target.placementKind) {
                                    const [sourceAnchor, targetAnchor] = defaultFlowAnchorsByPlacementType(source.placement.placementKind, target.placementKind)

                                    reactionDecider.connect_flow(
                                        source.placement.placementId,
                                        flowAnchorToString(sourceAnchor),
                                        target.placementId,
                                        flowAnchorToString(targetAnchor),
                                    )
                                }
                            }
                            return { kind: DraggingStateKind.NONE };
                        }
                        default:
                            return { kind: DraggingStateKind.NONE };
                    }
                }
                case DraggingCommandKind.FLOW_PORT_DRAG_START: {
                    return {
                        kind: DraggingStateKind.FLOW,
                        value: {
                            source: command.value
                        }
                    }
                }
                case DraggingCommandKind.OUT_OF_BOUNDS_DRAG_ENTER: {
                    switch (state.kind) {
                        case DraggingStateKind.LANE:
                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: undefined,
                                }
                            };
                        case DraggingStateKind.PLACEMENT:
                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: undefined,

                                }
                            };
                        default:
                            return state;
                    }
                }
                case DraggingCommandKind.OUT_OF_BOUNDS_DRAG_END: {
                    return {
                        kind: DraggingStateKind.NONE
                    };
                }
                case DraggingCommandKind.CURSOR_MOVE: {
                    switch (state.kind) {
                        case DraggingStateKind.FLOW: {
                            const { target } = state.value;

                            if (target?.placementId && target.targetStatus === 'good') {
                                return state;
                            }

                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    cursor: command.value
                                }
                            }
                        }
                        default:
                            return state;
                    }
                }
            }
    }
}

function laneTargetStatus(source: LaneSource, target: RowTarget): DropTargetStatus {
    if (rowtargetToLaneId(target) === "DEFAULT_LANE") {
        return "bad"; // Do not drop rows on default row
    }

    return (source.laneKind == target.rowKind) ? "good" : "bad";
}

function placementTargetStatus({ placementKind }: PlacementSource, { placementId, row: { rowKind }}: CellTarget): DropTargetStatus {
    if (placementId) { // Don't drop on occupied cell
        return "bad";
    }

    switch (rowKind) {
        case "audience":
            return ( placementKind == "interface" ) ? "good" : "bad";
        case "stream":
            return ( placementKind == "event" ) ? "good" : "bad";
        case "timeline":
            return (["readModel", "command"].includes(placementKind) ? "good" : "bad" )
    }
}

function flowTargetStatus({ placement: { placementKind, placementId } }: FlowPortSource, target: CellTarget): DropTargetStatus {
    if (!target.placementId || !target.placementKind) {
        return "bad";
    }

    const targetPlacementId = target.placementId;
    const targetPlacementKind = target.placementKind;

    if (placementId === targetPlacementId) { // Cant link to self
        return "bad";
    }

    switch(placementKind) {
        case "command":
            return (targetPlacementKind === "event") ? "good" : "bad";
        case "event":
            return (targetPlacementKind === "readModel") ? "good" : "bad";
        case "interface":
            return (targetPlacementKind === "command") ? "good" : "bad";
        case "readModel":
            return (targetPlacementKind === "interface") ? "good" : "bad";
    }
}

const flowTargetDefaultPort = (source: PlacementType, target: PlacementType): FlowAnchor => {
    if (source === "interface" && target === "command") {
        return FlowAnchor.Top;
    }
    else if ( source  === "command" && target === "event") {
        return FlowAnchor.Top;
    }
    else if (source === "event" && target === "readModel") {
        return FlowAnchor.Bottom;
    }
    else if (source === "readModel" && target === "interface") {
        return FlowAnchor.Bottom;
    }
    // START IMPOSSIBLE LINKS THAT STILL NEED TO BE DISPLAYED ON A 'bad' TARGET
    else if (source === "command" && target === "readModel") {
        return FlowAnchor.Left;
    }
    else if (source === "readModel" && target === "command") {
        return FlowAnchor.Left;
    }
    else if (source === "readModel" && target === "readModel") {
        return FlowAnchor.Left;
    }
    else if (source === "command" && target === "command") {
        return FlowAnchor.Left;
    }
    else if (source === "readModel" && target === "event") {
        return FlowAnchor.Top;
    }
    else if (source === "command" && target ==="interface") {
        return FlowAnchor.Bottom;
    // END IMPOSSIBLE
    } else {
        return FlowAnchor.Bottom;
    }
}

const defaultFlowAnchorsByPlacementType = (source: PlacementType, target: PlacementType): [FlowAnchor, FlowAnchor] => {
    if (source === "interface" && target === "command") {
        return [FlowAnchor.Bottom, FlowAnchor.Top];
    } else if (source === "command" && target === "event") {
        return [FlowAnchor.Bottom, FlowAnchor.Top];
    } else if (source === "event" && target === "readModel") {
        return [FlowAnchor.Top, FlowAnchor.Bottom];
    } else if (source === "readModel" && target === "interface") {
        return [FlowAnchor.Top, FlowAnchor.Bottom];
    } else {
        return [FlowAnchor.Top, FlowAnchor.Bottom];
    }
}

const flowAnchorToString = (anchor: FlowAnchor): string => {
    switch (anchor) {
        case FlowAnchor.Top:
            return "Top";
        case FlowAnchor.Bottom:
            return "Bottom";
        case FlowAnchor.Left:
            return "Left";
        case FlowAnchor.Right:
            return "Right";
        case FlowAnchor.None:
            return "None";
    }
}

const rowtargetToLaneId = (row: RowTarget): string | DefaultLane | undefined => {
    switch (row.rowKind) {
        case "audience": {
            return row.audienceId;
        }
        case "stream": {
            return row.streamId
        }
        case "timeline": {
            return undefined;
        }
    }
}

const laneIdFilterDefault = (laneId: string | DefaultLane | undefined): string | undefined =>
    (laneId === "DEFAULT_LANE") ? undefined : laneId;

export const linkingFlowFromState = (state: DraggingState): Flow | void => {
    if (state.kind !== DraggingStateKind.FLOW) {
        return;
    }

    const { source, cursor, target } = state.value;

    // Need at least one
    if (!cursor && !target) {
        return;
    }

    const from: FlowPort = {
        placement_id: source.placement.placementId,
        anchor: source.position,
        kind: "FlowPort"
    };

    // Use placement if in proper cell - fallback to cursor position
    const to: FlowPort | FlowCursor | undefined = target && target.placementId && target.placementKind ?
        {
            placement_id: target.placementId,
            anchor: flowTargetDefaultPort(source.placement.placementKind, target.placementKind),
            kind: "FlowPort",
        } :
        cursor ? { x: cursor.x, y: cursor.y, kind: "FlowCursor" } : undefined;

    if (!to) {
        return;
    }

    return {
        id: "linking",
        to,
        from,
        dashed: true,
        color: linkingFlowColor(state),
        strokeWidth: 2,
    }
}

const linkingFlowColor = (state: DraggingState): LinkingFlowColor => {
    if (state.kind === DraggingStateKind.FLOW) {
        const { target } = state.value;

        if (target && target.placementId) {
            switch (target.targetStatus) {
                case "bad":
                    return "red";
                case "good":
                    return "green";
            }
        }
    }

    return "black";
}
