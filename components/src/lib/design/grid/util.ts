import type { Decider, DropTargetStatus, LaneKind, PlacementType } from "../Grid";

// Types
interface WithDropTargetStatus { targetStatus: DropTargetStatus}

export type SourceEffect = "MOVE" | "DUPLICATE";
export type RowKind = LaneKind | "timeline";

export interface LaneSource {
    laneId: string,
    laneKind: LaneKind
}

export interface PlacementSource {
    placementId: string,
    placementKind: PlacementType,
    sourceEffect: SourceEffect,
}

export interface LaneTarget {
    laneIndex: number,
    laneKind: LaneKind, 
}

export type AudienceTarget = {
    audienceId: string,
    laneIndex: number,
    rowKind: "audience"
};

export type StreamTarget = {
    streamId: string,
    laneIndex: number,
    rowKind: "stream"
};

export type TimelineTarget = {
    rowKind: "timeline"
};

export type RowTarget = | AudienceTarget | StreamTarget | TimelineTarget;

export interface CellTarget {
    column: number,
    row: RowTarget,
}

export enum DraggingStateKind {
    LANE,
    PLACEMENT,
    NONE
}

export type DraggingState =
    | { kind: DraggingStateKind.LANE,
        value: {
            source: LaneSource,
            target?: RowTarget & WithDropTargetStatus} }
    | { kind: DraggingStateKind.PLACEMENT,
        value: {
            source: PlacementSource,
            target?: CellTarget & WithDropTargetStatus } }
    | { kind: DraggingStateKind.NONE };

export enum DraggingCommandKind {
    LANE_DRAG_START,
    LANE_DRAG_ENTER,
    LANE_DRAG_DROP,
    PLACEMENT_DRAG_START,
    CELL_DRAG_ENTER,
    CELL_DRAG_DROP,
    OUT_OF_BOUNDS_DRAG_ENTER,
    OUT_OF_BOUNDS_DRAG_END,
}

export type DragCommand =
    | { kind: DraggingCommandKind.LANE_DRAG_START,
        value: LaneSource }
    | { kind: DraggingCommandKind.LANE_DRAG_ENTER,
        value: RowTarget }
    | { kind: DraggingCommandKind.LANE_DRAG_DROP }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_START,
        value: PlacementSource }
    | { kind: DraggingCommandKind.CELL_DRAG_ENTER,
        value: CellTarget & RowTarget }
    | { kind: DraggingCommandKind.CELL_DRAG_DROP }
    | { kind:  DraggingCommandKind.OUT_OF_BOUNDS_DRAG_ENTER }
    | { kind:  DraggingCommandKind.OUT_OF_BOUNDS_DRAG_END }

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
                    const { placementId, placementKind, sourceEffect } = command.value;

                    return {
                        kind: DraggingStateKind.PLACEMENT,
                        value: {
                            source: {
                                placementId,
                                placementKind,
                                sourceEffect
                            }
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

                                const laneId = rowtargetToLaneId(row);

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
                                        }
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
                                        }
                                }
                            }
                        }
                        default:
                            return { kind: DraggingStateKind.NONE };
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
            }
    }
}

function laneTargetStatus(source: LaneSource, target: RowTarget): DropTargetStatus {
    return (source.laneKind == target.rowKind) ? "good" : "bad";
}

function placementTargetStatus({ placementKind }: PlacementSource, { row: { rowKind }}: CellTarget): DropTargetStatus {
    switch (rowKind) {
        case "audience":
            return ( placementKind == "interface" ) ? "good" : "bad";
        case "stream":
            return ( placementKind == "event" ) ? "good" : "bad";
        case "timeline":
            return (["readModel", "command"].includes(placementKind) ? "good" : "bad" )
    }
}

const rowtargetToLaneId = (row: RowTarget): string | undefined => {
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
