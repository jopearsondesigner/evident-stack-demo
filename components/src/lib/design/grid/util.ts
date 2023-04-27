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
    PLACEMENT_DRAG_DROP
}

export type DragCommand =
    | { kind: DraggingCommandKind.LANE_DRAG_START,
        value: {
            laneId: string,
            laneType: Lane,
        } }
    | { kind: DraggingCommandKind.LANE_DRAG_ENTER,
        value: { laneIndex: number, laneType: Lane } }
    | { kind: DraggingCommandKind.LANE_DRAG_LEAVE, }
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
        }}
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_LEAVE }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_DROP }

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
                    return {
                        ...state,
                        value: {
                            ...state.value,
                            target: undefined
                        }
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
                    return {
                        ...state,
                        value: {
                            ...state.value,
                            target: undefined
                        }
                    }
                }
                case DraggingStateKind.PLACEMENT: {
                    return {
                        ...state,
                        value: {
                            ...state.value,
                            target: undefined
                        }
                    }
                }
                case DraggingStateKind.NONE:
                    return state;
            }
        case DraggingCommandKind.PLACEMENT_DRAG_DROP:
            return { kind: DraggingStateKind.NONE };
    }
}
