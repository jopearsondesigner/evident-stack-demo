import type { Decider, DropTargetStatus, LaneKind, PlacementType } from "../Grid";

// Types
interface DropTargetWithStatus { targetStatus: DropTargetStatus}

interface LaneTarget {
    laneIndex: number,
    laneKind: LaneKind, 
}

interface PlacementTarget {
    column: number,
    laneId: string,
    laneKind: LaneKind,
}

interface LaneSource {
    laneId: string,
    laneKind: LaneKind
}

interface PlacementSource {
    placementId: string,
    placementKind: PlacementType
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
            target?: LaneTarget & DropTargetWithStatus} }
    | { kind: DraggingStateKind.PLACEMENT,
        value: {
            source: PlacementSource,
            target?: PlacementTarget & DropTargetWithStatus } }
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
        value: {
            laneId: string,
            laneKind: LaneKind,
        } }
    | { kind: DraggingCommandKind.LANE_DRAG_ENTER,
        value: { laneIndex: number, laneKind: LaneKind } }
    | { kind: DraggingCommandKind.LANE_DRAG_DROP }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_START,
        value: {
            placementId: string,
            placementKind: PlacementType
        }}
    | { kind: DraggingCommandKind.CELL_DRAG_ENTER,
        value: {
            column: number
            laneIndex: number,
            laneKind: LaneKind,
            laneId: string,
            placementId?: string,
        }}
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
                    let { laneIndex, laneKind: laneType } = command.value;

                    switch (state.kind) {
                        case DraggingStateKind.LANE:
                            const target = {
                                laneIndex,
                                laneKind: laneType,
                            };

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
                            if (state.value.target?.targetStatus == "good") {
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
                    const { placementId, placementKind } = command.value;
                    return {
                        kind: DraggingStateKind.PLACEMENT,
                        value: {
                            source: {
                                placementId,
                                placementKind
                            }
                        }
                    };
                case DraggingCommandKind.CELL_DRAG_ENTER: {
                    const { column, laneIndex, laneKind, laneId } = command.value;

                    switch (state.kind) {
                        case DraggingStateKind.LANE: {
                            const target: LaneTarget = {
                                laneIndex,
                                laneKind,
                            };

                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        ...target,
                                        targetStatus: laneTargetStatus(state.value.source, target)
                                    }
                                }
                            }
                        }
                        case DraggingStateKind.PLACEMENT: {
                            const target: PlacementTarget = {
                                column,
                                laneId,
                                laneKind,
                            };

                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        ...target,                                        
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
                            if (state.value.target && state.value.target.targetStatus === "good") {
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
                                const { placementId, placementKind } = source;
                                const { column, laneId } = target

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

export function laneTargetStatus(source: LaneSource, target: LaneTarget): DropTargetStatus {
    return (source.laneKind == target.laneKind) ? "good" : "bad";
}

export function placementTargetStatus({ placementKind }: PlacementSource, { laneKind }: PlacementTarget): DropTargetStatus {
    switch (laneKind) {
        case "audience":
            return ( placementKind == "interface" ) ? "good" : "bad";
        case "stream":
            return ( placementKind == "event" ) ? "good" : "bad";
    }
}
