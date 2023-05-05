import type { Decider, DropTargetStatus, LaneKind, PlacementType } from "../Grid";

export enum DraggingStateKind {
    LANE,
    PLACEMENT,
    NONE
}

export type DraggingState =
    | { kind: DraggingStateKind.LANE,
        value: {
            source: {
                laneId: string,
                laneKind: LaneKind,
            },
            target?: {
                laneIndex: number,
                laneKind: LaneKind, 
            }} }
    | { kind: DraggingStateKind.PLACEMENT,
        value: {
            source: {
                placementId: string,
                placementKind: PlacementType
            },
            target?: {
                column: number,
                laneId: string
            } } }
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
                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        laneIndex,
                                        laneKind: laneType
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
                            if (laneTargetFromState(state)?.targetStatus === "good" && state.value.target) {
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
                        case DraggingStateKind.LANE:
                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        laneIndex,
                                        laneKind,
                                    }
                                }
                            }
                        case DraggingStateKind.PLACEMENT:
                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: {
                                        column,
                                        laneId,
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
                            if (laneTargetFromState(state)?.targetStatus === "good" && state.value.target) {
                                const { source, target } = state.value;
                                const { laneId, laneKind: laneType } = source;
                                const { laneIndex } = target;
                                reactionDecider.reorder_lane(laneType, laneId, laneIndex);
                            }
                            return { kind: DraggingStateKind.NONE }
                        }
                        case DraggingStateKind.PLACEMENT: {
                            if (placementTargetFromState(state)?.targetStatus === "good" && state.value.target) {
                                const { source, target } = state.value;
                                const { placementId, placementKind } = source;
                                const { column, laneId } = target
                                switch (placementKind) {
                                    case "command":
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
                            return { kind: DraggingStateKind.NONE }
                        }
                    }

                    return { kind: DraggingStateKind.NONE };
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

export function laneTargetFromState(state: DraggingState): { index: number, kind: LaneKind, targetStatus: DropTargetStatus } | void {
    switch (state.kind) {
        case DraggingStateKind.LANE: {
            if (!state.value.target) {
                return undefined;
            }

            let sourceKind = state.value.source.laneKind;
            let targetKind = state.value.target?.laneKind;

            return {
                index: state.value.target.laneIndex,
                kind: targetKind,
                targetStatus: (sourceKind == targetKind) ? "good" : "bad"
            };
        }
        default:
            return undefined;
    }
}

export function placementTargetFromState(state: DraggingState): { column: number, laneId: string, targetStatus: DropTargetStatus } | void {
    switch (state.kind) {
        case DraggingStateKind.PLACEMENT: {
            if (!state.value.target) {
                return undefined;
            }

            const { column, laneId } = state.value.target;

            return {
                column,
                laneId,
                targetStatus: "good"
            }
        }
        default:
            return undefined;
    }
}
