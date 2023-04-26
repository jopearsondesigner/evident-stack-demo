import { Lane, PlacementType } from "../Grid";

enum DraggingStateKind {
    LANE,
    INTERFACE,
    READ_MODEL,
    COMMAND,
    EVENT,
    NONE
}

type DraggingState =
    | { kind: DraggingStateKind.LANE,
        value: {
            source: {
                laneId: string,
                laneType: Lane,
            },
            target?: {
                laneIndex: number,
                laneType: Lane, 
            }} }
    | { kind: DraggingStateKind.INTERFACE,
        value: {
            source: { interfaceId: string },
            target?: { column: number, audienceId: string } } }
    | { kind: DraggingStateKind.READ_MODEL
        value: {
            source: { readModelId: string },
            target?: { column: number } } }
    | { kind: DraggingStateKind.COMMAND,
        value: {
            source: { commandId: string },
            target?: { column: number }} }
    | { kind: DraggingStateKind.EVENT,
        value: {
            source: { eventId: string },
            target?: { column: number, streamId: string } }} 
    | { kind: DraggingStateKind.NONE };

enum DraggingCommandKind {
    LANE_DRAG_START,
    LANE_DRAG_ENTER,
    LANE_DRAG_LEAVE,
    LANE_DRAG_DROP,
    PLACEMENT_DRAG_START,
    PLACEMENT_DRAG_ENTER,
    PLACEMENT_DRAG_LEAVE,
    PLACEMENT_DRAG_DROP
}

type DragCommand =
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
            placementKind?: PlacementType
        }}
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_ENTER,
        value: {
            column: number
            laneIndex: number,
            laneType: Lane,
            laneId: string,
        }}
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_LEAVE }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_DROP }

function decide(state: DraggingState, command: DragCommand): DraggingState {
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
                                laneType
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
            return state;
        case DraggingCommandKind.PLACEMENT_DRAG_START:
            return state;
        case DraggingCommandKind.PLACEMENT_DRAG_ENTER: {
            switch (state.kind) {
                default:
                    return state;
            }
            // case DraggingStateKind.LANE:
            // case DraggingStateKind.INTERFACE:
            // case DraggingStateKind.READ_MODEL:
            // case DraggingStateKind.COMMAND:
            // case DraggingStateKind.EVENT:
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
                case DraggingStateKind.INTERFACE: {
                    return {
                        ...state,
                        value: {
                            ...state.value,
                            target: undefined
                        }
                    }
                }
                case DraggingStateKind.READ_MODEL: {
                    return {
                        ...state,
                        value: {
                            ...state.value,
                            target: undefined
                        }
                    }
                }
                case DraggingStateKind.COMMAND: {
                    return {
                        ...state,
                        value: {
                            ...state.value,
                            target: undefined
                        }
                    }
                }
                case DraggingStateKind.EVENT: {
                    return {
                        ...state,
                        value: {
                            ...state.value,
                            target: undefined
                        }
                    }
                }
                case DraggingStateKind.NONE:
            }
        case DraggingCommandKind.PLACEMENT_DRAG_DROP:
            return state;
    }

    // switch(state.kind) {
    //     case DraggingStateKind.AUDIENCE:
    //         return state;
    //     case DraggingStateKind.STREAM:
    //         return state;
    //     case DraggingStateKind.INTERFACE:
    //         return state;
    //     case DraggingStateKind.COMMAND:
    //         return state;
    //     case DraggingStateKind.EVENT:
    //         return state;
    //     default:
    //         return state;
    // }
}
