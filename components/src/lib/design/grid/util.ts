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

export enum GridStateKind {
    LANE,
    PLACEMENT,
    FLOW,
    CONTEXT,
    NONE
}

export type GridState =
    | { kind: GridStateKind.LANE,
        value: {
            source: LaneSource,
            target?: RowTarget & WithDropTargetStatus} }
    | { kind: GridStateKind.PLACEMENT,
        value: {
            source: PlacementSource & WithSourceEffect,
            target?: CellTarget & WithDropTargetStatus } }
    | { kind: GridStateKind.FLOW,
        value: {
            source: FlowPortSource,
            cursor?: CursorTarget,
            target?: (CellTarget & WithDropTargetStatus)
        }}
    | { kind: GridStateKind.CONTEXT
        source: CellTarget & CursorTarget
     }
    | { kind: GridStateKind.NONE };

export enum GridCommandKind {
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
    OPEN_CONTEXT_MENU,
    CLOSE_CONTEXT_MENU,
    CONTEXT_COMMAND
}

export type GridCommand =
    | { kind: GridCommandKind.LANE_DRAG_START,
        value: LaneSource }
    | { kind: GridCommandKind.LANE_DRAG_ENTER,
        value: RowTarget }
    | { kind: GridCommandKind.LANE_DRAG_DROP }
    | { kind: GridCommandKind.PLACEMENT_DRAG_START,
        value: PlacementSource & WithSourceEffect }
    | { kind: GridCommandKind.CELL_DRAG_ENTER,
        value: CellTarget }
    | { kind: GridCommandKind.CELL_DRAG_DROP }
    | { kind: GridCommandKind.FLOW_PORT_DRAG_START,
        value: FlowPortSource }
    | { kind:  GridCommandKind.OUT_OF_BOUNDS_DRAG_ENTER }
    | { kind:  GridCommandKind.OUT_OF_BOUNDS_DRAG_END }
    | { kind: GridCommandKind.CURSOR_MOVE,
        value: CursorTarget }
    | { kind: GridCommandKind.OPEN_CONTEXT_MENU,
        value: CellTarget & CursorTarget }
    | { kind: GridCommandKind.CLOSE_CONTEXT_MENU }
    | { kind: GridCommandKind.CONTEXT_COMMAND,
        value: ContextMenuCommand }

export enum ContextMenuCommandKind {
    InsertColumnLeft,
    InsertColumnRight,
    InsertLaneAbove,
    InsertLaneBelow,
    DeletePlacement
}

export enum ContextMenuCommand {
    InsertColumnLeft,
    InsertColumnRight,
    InsertLaneAbove,
    InsertLaneBelow,
    DeletePlacement
}


export function buildEvolveAndReact(reactionDecider: Decider): (s: GridState, c: GridCommand) => GridState {
    return (state: GridState, command: GridCommand) => {
        switch (command.kind) {
                case GridCommandKind.LANE_DRAG_START: {
                    let { laneId, laneKind: laneType } = command.value

                    return {
                        kind: GridStateKind.LANE,
                        value: {
                            source: { laneId, laneKind: laneType } ,
                        }
                    };
                }
                    
                case GridCommandKind.LANE_DRAG_ENTER: {
                    const target = command.value;

                    switch (state.kind) {
                        case GridStateKind.LANE:
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
                case GridCommandKind.LANE_DRAG_DROP: {
                    switch (state.kind) {
                        case GridStateKind.LANE:
                            if (state.value.target?.targetStatus == "good" && state.value.target?.rowKind !== "timeline") {
                                const { source, target } = state.value;
                                const { laneId, laneKind: laneType } = source;
                                const { laneIndex } = target;
                                reactionDecider.reorder_lane(laneType, laneId, laneIndex);
                            }
                        default:
                    }

                    return { kind: GridStateKind.NONE }
                };
                case GridCommandKind.PLACEMENT_DRAG_START:
                    return {
                        kind: GridStateKind.PLACEMENT,
                        value: {
                            source: command.value
                        }
                    };
                case GridCommandKind.CELL_DRAG_ENTER: {
                    const { row } = command.value;

                    switch (state.kind) {
                        case GridStateKind.LANE: {
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
                        case GridStateKind.PLACEMENT: {
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
                        case GridStateKind.FLOW: {
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
                            
                        case GridStateKind.NONE:
                            return state;
                    }
                }
                case GridCommandKind.CELL_DRAG_DROP: {
                    switch (state.kind) {
                        case GridStateKind.LANE: {
                            if (state.value.target && state.value.target.rowKind !== "timeline" && state.value.target.targetStatus === "good") {
                                const { source, target } = state.value;
                                const { laneId, laneKind: laneType } = source;
                                const { laneIndex } = target;

                                reactionDecider.reorder_lane(laneType, laneId, laneIndex);
                            }
                            return { kind: GridStateKind.NONE }
                        }
                        case GridStateKind.PLACEMENT: {
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

                            return { kind: GridStateKind.NONE };
                        }
                        case GridStateKind.FLOW: {
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
                            return { kind: GridStateKind.NONE };
                        }
                        default:
                            return { kind: GridStateKind.NONE };
                    }
                }
                case GridCommandKind.FLOW_PORT_DRAG_START: {
                    return {
                        kind: GridStateKind.FLOW,
                        value: {
                            source: command.value
                        }
                    }
                }
                case GridCommandKind.OUT_OF_BOUNDS_DRAG_ENTER: {
                    switch (state.kind) {
                        case GridStateKind.LANE:
                            return {
                                ...state,
                                value: {
                                    ...state.value,
                                    target: undefined,
                                }
                            };
                        case GridStateKind.PLACEMENT:
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
                case GridCommandKind.OUT_OF_BOUNDS_DRAG_END: {
                    return {
                        kind: GridStateKind.NONE
                    };
                }
                case GridCommandKind.CURSOR_MOVE: {
                    switch (state.kind) {
                        case GridStateKind.FLOW: {
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
                case GridCommandKind.OPEN_CONTEXT_MENU:
                    return {
                        kind: GridStateKind.CONTEXT,
                        source: command.value
                    };
                case GridCommandKind.CLOSE_CONTEXT_MENU:
                    return { kind: GridStateKind.NONE };
                case GridCommandKind.CONTEXT_COMMAND:
                    switch (state.kind) {
                        case GridStateKind.CONTEXT:
                            contextMenuReact(state.source, command.value, reactionDecider)
                        default:
                            return state;
                    }
            }
    }
}

async function contextMenuReact(state: CellTarget & CursorTarget, command: ContextMenuCommand, reactionDecider: Decider) {
    switch (command) {
        case ContextMenuCommand.DeletePlacement:
            if (state.placementId) {
                await reactionDecider.remove_placement(state.placementId)
            }
            break;
        case ContextMenuCommand.InsertColumnLeft:
            console.warn("ContextMenuCommand.InsertColumnLeft");
            await reactionDecider.insert_columns(state.column, "LEFT", 1);
            break;
        case ContextMenuCommand.InsertColumnRight:
            console.warn("ContextMenuCommand.InsertColumnRight");
            await reactionDecider.insert_columns(state.column, "RIGHT", 1);
            break;
        case ContextMenuCommand.InsertLaneAbove:
            if (state.row) {
                await reactionDecider.add_lane(state.row.rowKind, state.row.rowIndex -1, "Placeholder Above") // TODO: two state insert lane with name menu
            }
            break;
        case ContextMenuCommand.InsertLaneBelow:
            if (state.row) {
                await reactionDecider.add_lane(state.row.rowKind, state.row.rowIndex +1, "Placeholder Below") // TODO: two state insert lane with name menu
            }
            break;
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

export const linkingFlowFromState = (state: GridState): Flow | void => {
    if (state.kind !== GridStateKind.FLOW) {
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

const linkingFlowColor = (state: GridState): LinkingFlowColor => {
    if (state.kind === GridStateKind.FLOW) {
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
