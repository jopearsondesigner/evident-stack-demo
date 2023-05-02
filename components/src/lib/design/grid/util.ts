import type { Lane, PlacementType } from "../Grid";

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
                laneType: Lane,
            },
            target?: {
                laneIndex: number,
                laneKind: Lane, 
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
    LANE_DRAG_LEAVE,
    LANE_DRAG_DROP,
    PLACEMENT_DRAG_START,
    PLACEMENT_DRAG_ENTER,
    PLACEMENT_DRAG_LEAVE,
    PLACEMENT_DRAG_DROP,
    OUT_OF_BOUNDS,
    RESET,
}

export type DragCommand =
    | { kind: DraggingCommandKind.LANE_DRAG_START,
        value: {
            laneId: string,
            laneType: Lane,
        } }
    | { kind: DraggingCommandKind.LANE_DRAG_ENTER,
        value: { laneIndex: number, laneType: Lane } }
    | { kind: DraggingCommandKind.LANE_DRAG_LEAVE,
        value: { laneIndex: number, laneType: Lane }}
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
            laneKind: Lane,
            laneId: string,
            placementId?: string,
        }}
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_LEAVE,
        value: {
            column: number
            laneIndex: number,
            laneKind: Lane,
            laneId: string,
            placementId?: string,
        }}
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_DROP }
    | { kind:  DraggingCommandKind.OUT_OF_BOUNDS }
    | { kind:  DraggingCommandKind.RESET }

export function evolveDraggingState(state: DraggingState, command: DragCommand): DraggingState {
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
        case DraggingCommandKind.LANE_DRAG_LEAVE:
            switch (state.kind) {
                case DraggingStateKind.LANE:
                    let { laneIndex, laneType } = command.value;

                    if (state.value.target && state.value.target.laneIndex == laneIndex && state.value.target.laneKind == laneType) {
                        console.warn("LANE DRAG CLEARING TARGET");
                        return {
                            ...state,
                            value: {
                                ...state.value,
                                target: undefined
                            }
                        }
                    } else {
                        console.info("Lane drag exiting - target already updated");
                        return state;
                    }
                    
                default:
                    return state;
            }
        case DraggingCommandKind.LANE_DRAG_DROP:
            return { kind: DraggingStateKind.NONE };
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
        case DraggingCommandKind.PLACEMENT_DRAG_LEAVE:
            switch (state.kind) {
                case DraggingStateKind.LANE: {
                    const { laneIndex, laneKind } = command.value;

                    if (state.value.target && state.value.target.laneIndex === laneIndex && state.value.target.laneKind == laneKind) {
                        return {
                        ...state,
                        value: {
                            ...state.value,
                            target: undefined
                        }
                    }
                    } else {
                        return state;
                    }
                    
                }
                case DraggingStateKind.PLACEMENT: {
                    const { column, laneId } = command.value;
                    if (state.value.target && state.value.target.column === column && state.value.target.laneId == laneId) {
                        console.warn("PLACEMENT DRAG CLEARING TARGET");
                        return {
                            ...state,
                            value: {
                                ...state.value,
                                target: undefined
                            }
                        }
                    } else {
                        console.info("Placement drag exiting - target already updated");
                        return state;
                    }
                    
                }
                case DraggingStateKind.NONE:
                    return state;
            }
        case DraggingCommandKind.PLACEMENT_DRAG_DROP:
            return { kind: DraggingStateKind.NONE };
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

export function laneTargetFromState(state: DraggingState): { index: number, kind: Lane, target_type: "good" | "bad" } | void {
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
