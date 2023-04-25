enum DraggingStateKind {
    AUDIENCE,
    STREAM,
    INTERFACE,
    READ_MODEL,
    COMMAND,
    EVENT,
    NONE
}

type DraggingState =
    | { kind: DraggingStateKind.AUDIENCE, value: { sourceId: string, targetIndex: number | undefined } }
    | { kind: DraggingStateKind.STREAM, value: { sourceId: string, targetIndex: number | undefined } }
    | { kind: DraggingStateKind.INTERFACE, value: { sourceId: string, target: { index: number, audienceId: string } | undefined } }
    | { kind: DraggingStateKind.READ_MODEL, value: { sourceId: string, targetIndex: number | undefined } }
    | { kind: DraggingStateKind.COMMAND, value: { sourceId: string, targetIndex: number | undefined } }
    | { kind: DraggingStateKind.EVENT, value: { sourceId: string, target: { index: number, streamId: string } | undefined }} 
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
    | { kind: DraggingCommandKind.LANE_DRAG_START, value: { laneId: string } }
    | { kind: DraggingCommandKind.LANE_DRAG_ENTER, value: { index: number } }
    | { kind: DraggingCommandKind.LANE_DRAG_LEAVE }
    | { kind: DraggingCommandKind.LANE_DRAG_DROP }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_START, value: { laneId: string } }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_ENTER, value: { index: number } }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_LEAVE }
    | { kind: DraggingCommandKind.PLACEMENT_DRAG_DROP }

function decide(state: DraggingState, command: DragCommand): DraggingState {
    switch(state.kind) {
        case DraggingStateKind.AUDIENCE:
            return state;
        case DraggingStateKind.STREAM:
            return state;
        case DraggingStateKind.INTERFACE:
            return state;
        case DraggingStateKind.COMMAND:
            return state;
        case DraggingStateKind.EVENT:
            return state;
        default:
            return state;
    }
}
