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
                laneType: LaneKind,
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
    PLACEMENT_DRAG_ENTER,
    PLACEMENT_DRAG_DROP,
    OUT_OF_BOUNDS,
    RESET,
}

export type DragCommand =
    | { kind: DraggingCommandKind.LANE_DRAG_START,
        value: {
            laneId: string,
            laneType: LaneKind,
        } }
    | { kind: DraggingCommandKind.LANE_DRAG_ENTER,
        value: { laneIndex: number, laneType: LaneKind } }
    | { kind: DraggingCommandKind.LANE_DRAG_DROP }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_START,
        value: {
            placementId: string,
            placementKind: PlacementType
        }}
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_ENTER,
        value: {
            column: number
            laneIndex: number,
            laneKind: LaneKind,
            laneId: string,
            placementId?: string,
        }}
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_DROP }
    | { kind:  DraggingCommandKind.OUT_OF_BOUNDS }
    | { kind:  DraggingCommandKind.RESET }

export function buildEvolveAndReact(reactionDecider: Decider): (s: DraggingState, c: DragCommand) => DraggingState {
    return (state: DraggingState, command: DragCommand) => {
        switch (command.kind) {
                case DraggingCommandKind.LANE_DRAG_START: {
                    let { laneId, laneType } = command.value

                    return {
                        kind: DraggingStateKind.LANE,
                        value: {
                            source: { laneId, laneType } ,
                        }
                    };
                }
                    
                case DraggingCommandKind.LANE_DRAG_ENTER: {
                    let { laneIndex, laneType } = command.value;

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
                            if (laneTargetFromState(state)?.target_type === "good" && state.value.target) {
                                const { source, target } = state.value;
                                const { laneId, laneType } = source;
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
                case DraggingCommandKind.PLACEMENT_DRAG_ENTER: {
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
                case DraggingCommandKind.PLACEMENT_DRAG_DROP: {
                    switch (state.kind) {
                        case DraggingStateKind.LANE:
                            if (laneTargetFromState(state)?.target_type === "good" && state.value.target) {
                                const { source, target } = state.value;
                                const { laneId, laneType } = source;
                                const { laneIndex } = target;
                                reactionDecider.reorder_lane(laneType, laneId, laneIndex);
                            }
                        case DraggingStateKind.PLACEMENT:
                    }

                    return { kind: DraggingStateKind.NONE };
                }
                case DraggingCommandKind.OUT_OF_BOUNDS: {
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
                case DraggingCommandKind.RESET: {
                    return {
                        kind: DraggingStateKind.NONE
                    };
                }
            }
    }
}

export function laneTargetFromState(state: DraggingState): { index: number, kind: LaneKind, target_type: DropTargetStatus } | void {
    switch (state.kind) {
        case DraggingStateKind.LANE: {
            if (!state.value.target) {
                return undefined;
            }

            let sourceKind = state.value.source.laneType;
            let targetKind = state.value.target?.laneKind;

            return {
                index: state.value.target.laneIndex,
                kind: targetKind,
                target_type: (sourceKind == targetKind) ? "good" : "bad"
            };
        }
        default:
            return undefined;
    }
}
