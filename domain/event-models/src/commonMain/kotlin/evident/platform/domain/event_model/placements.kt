package evident.platform.domain.event_model

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.flowOf
import kotlinx.serialization.Serializable
import kotlinx.uuid.generateUUID

// Commands

sealed interface PlacementCommand : EventModelCommand
sealed interface DefineAndPlaceComponent : PlacementCommand

@Serializable
data class DefineAndPlaceInterface(
    val index: PlacementIndex,
    val type: InterfaceType,
    val name: Name,
    val description: Description? = null,
    val audienceId: AudienceId? = null,
    val width: InterfaceDimension? = null,
    val height: InterfaceDimension? = null
) : DefineAndPlaceComponent

@Serializable
data class DefineAndPlaceCommand(
    val index: PlacementIndex,
    val name: Name,
    val description: Description? = null
) : DefineAndPlaceComponent

@Serializable
data class DefineAndPlaceEvent(
    val index: PlacementIndex,
    val name: Name,
    val description: Description? = null,
    val streamId: StreamId? = null,
) : DefineAndPlaceComponent

@Serializable
data class DefineAndPlaceReadModel(
    val index: PlacementIndex,
    val name: Name,
    val description: Description? = null
) : DefineAndPlaceComponent

@Serializable
sealed interface PlaceComponent : PlacementCommand

@Serializable
data class PlaceInterface(
    val index: PlacementIndex,
    val interfaceId: InterfaceId,
    val audienceId: AudienceId? = null,
) : PlaceComponent

@Serializable
data class PlaceCommand(
    val index: PlacementIndex,
    val commandId: CommandId,
) : PlaceComponent

@Serializable
data class PlaceEvent(
    val index: PlacementIndex,
    val eventId: EventId,
    val streamId: StreamId? = null,
) : PlaceComponent

@Serializable
data class PlaceReadModel(
    val index: PlacementIndex,
    val readModelId: ReadModelId,
) : PlaceComponent

sealed interface MovePlacement : PlacementCommand {
    val placementId: PlacementId
    val index: PlacementIndex
}

@Serializable
data class MoveInterfacePlacement(
    override val placementId: PlacementId,
    override val index: PlacementIndex,
    val audienceId: AudienceId? = null,
) : MovePlacement

@Serializable
data class MoveTimelinePlacement(
    override val placementId: PlacementId,
    override val index: PlacementIndex,
) : MovePlacement

@Serializable
data class MoveEventPlacement(
    override val placementId: PlacementId,
    override val index: PlacementIndex,
    val streamId: StreamId? = null,
) : MovePlacement

@Serializable
data class RemovePlacement(
    val placementId: PlacementId,
    val confirmed: Boolean = false
) : PlacementCommand

// Events

sealed interface PlacementEvent : EventModelEvent

sealed interface ComponentPlaced : PlacementEvent

@Serializable
data class InterfacePlaced(
    val id: PlacementId,
    val index: PlacementIndex,
    val interfaceId: InterfaceId,
    val audienceId: AudienceId? = null,
) : ComponentPlaced

@Serializable
data class CommandPlaced(
    val id: PlacementId,
    val index: PlacementIndex,
    val commandId: CommandId,
) : ComponentPlaced

@Serializable
data class EventPlaced(
    val id: PlacementId,
    val index: PlacementIndex,
    val eventId: EventId,
    val streamId: StreamId? = null,
) : ComponentPlaced

@Serializable
data class ReadModelPlaced(
    val id: PlacementId,
    val index: PlacementIndex,
    val readModelId: ReadModelId,
) : ComponentPlaced

@Serializable
sealed interface PlacementMoved : PlacementEvent {
    val placementId: PlacementId
    val index: PlacementIndex
}

@Serializable
data class InterfacePlacementMoved(
    override val placementId: PlacementId,
    override val index: PlacementIndex,
    val audienceId: AudienceId?
) : PlacementMoved

@Serializable
data class TimelinePlacementMoved(
    override val placementId: PlacementId,
    override val index: PlacementIndex,
) : PlacementMoved

@Serializable
data class EventPlacementMoved(
    override val placementId: PlacementId,
    override val index: PlacementIndex,
    val streamId: StreamId? = null,
) : PlacementMoved

@Serializable
data class PlacementRemoved(
    val id: PlacementId
) : PlacementEvent

// API

fun decideOnPlacementCommand(
    command: PlacementCommand,
    state: EventModel
): Either<Error, Flow<EventModelEvent>> =
    when (command) {
        is DefineAndPlaceComponent -> when (command) {
            is DefineAndPlaceInterface -> {
                val interfaceId = InterfaceId.generateUUID()
                state.interfaceByName(command.name)
                    ?.let { DuplicateInterfaceNameError(it.name).left() }
                    ?: flowOf(
                        InterfaceDefined(
                            command.type,
                            interfaceId,
                            command.name,
                            command.description,
                            command.width,
                            command.height
                        ),
                        InterfacePlaced(
                            PlacementId.generateUUID(),
                            command.index,
                            interfaceId
                        )
                    ).right()

            }

            is DefineAndPlaceCommand -> {
                val commandId = CommandId.generateUUID()
                state.commandByName(command.name)
                    ?.let { DuplicateCommandNameError(it.name).left() }
                    ?: flowOf(
                        CommandDefined(
                            commandId,
                            command.name,
                            command.description
                        ),
                        CommandPlaced(
                            PlacementId.generateUUID(),
                            command.index,
                            commandId
                        )
                    ).right()
            }

            is DefineAndPlaceEvent -> {
                val eventId = EventId.generateUUID()
                state.eventByName(command.name)
                    ?.let { DuplicateEventNameError(it.name).left() }
                    ?: flowOf(
                        EventDefined(
                            eventId,
                            command.name,
                            command.description
                        ),
                        EventPlaced(
                            PlacementId.generateUUID(),
                            command.index,
                            eventId
                        )
                    ).right()
            }

            is DefineAndPlaceReadModel -> {
                val readModelId = ReadModelId.generateUUID()
                state.readModelByName(command.name)
                    ?.let { DuplicateReadModelNameError(it.name).left() }
                    ?: flowOf(
                        ReadModelDefined(
                            readModelId,
                            command.name,
                            command.description
                        ),
                        ReadModelPlaced(
                            PlacementId.generateUUID(),
                            command.index,
                            readModelId
                        )
                    ).right()
            }
        }

        is PlaceComponent -> when (command) {
            is PlaceInterface -> flowOf(
                InterfacePlaced(
                    PlacementId.generateUUID(),
                    command.index,
                    command.interfaceId,
                    command.audienceId
                )
            ).right()

            is PlaceCommand -> flowOf(
                CommandPlaced(
                    PlacementId.generateUUID(),
                    command.index,
                    command.commandId,
                )
            ).right()

            is PlaceEvent -> flowOf(
                EventPlaced(
                    PlacementId.generateUUID(),
                    command.index,
                    command.eventId,
                    command.streamId
                )
            ).right()

            is PlaceReadModel -> flowOf(
                ReadModelPlaced(
                    PlacementId.generateUUID(),
                    command.index,
                    command.readModelId,
                )
            ).right()
        }

        is MovePlacement -> {
            state.placements[command.placementId]?.let { placement ->
                when (command) {
                    is MoveInterfacePlacement ->
                        if (placement is InterfacePlacement) {
                            flowOf(
                                InterfacePlacementMoved(placement.id, command.index, command.audienceId)
                            ).right()
                        } else {
                            IllegalPlacementError(
                                command.placementId,
                                "Trying to move a non-interface placement to an interface area"
                            ).left()
                        }

                    is MoveTimelinePlacement -> when (placement) {
                        is CommandPlacement, is ReadModelPlacement -> {
                            flowOf(
                                TimelinePlacementMoved(placement.id, command.index)
                            ).right()
                        }

                        else -> {
                            IllegalPlacementError(
                                command.placementId,
                                "Trying to move a non-timeline placement to the timeline"
                            ).left()
                        }
                    }

                    is MoveEventPlacement ->
                        if (placement is EventPlacement) {
                            flowOf(
                                EventPlacementMoved(placement.id, command.index, command.streamId)
                            ).right()
                        } else {
                            IllegalPlacementError(
                                command.placementId,
                                "Trying to move a non-event placement to an event area"
                            ).left()
                        }
                }
            } ?: PlacementNotFoundError(command.placementId).left()
        }

        is RemovePlacement -> {
            if (command.confirmed) {
                state.placements[command.placementId]?.let { placement ->
                    val events = mutableListOf<EventModelEvent>(PlacementRemoved(placement.id))
                    // If this is the only placement of this component, also remove the component
                    when (placement) {
                        is InterfacePlacement ->
                            if (state.placementsOfInterface(placement.interfaceId).size == 1) {
                                events + InterfaceRemoved(placement.interfaceId)
                            }

                        is CommandPlacement ->
                            if (state.placementsOfCommand(placement.commandId).size == 1) {
                                events + CommandRemoved(placement.commandId)
                            }

                        is EventPlacement ->
                            if (state.placementsOfEvent(placement.eventId).size == 1) {
                                events + EventRemoved(placement.eventId)
                            }

                        is ReadModelPlacement ->
                            if (state.placementsOfReadModel(placement.readModelId).size == 1) {
                                events + ReadModelRemoved(placement.readModelId)
                            }
                    }
                    events.asFlow().right()
                } ?: PlacementNotFoundError(command.placementId).left()
            } else {
                PlacementRemovalConfirmationError(command.placementId).left()
            }
        }
    }

fun evolveOnPlacementEvent(state: EventModel, event: PlacementEvent): EventModel =
    when (event) {
        is ComponentPlaced -> {
            val builder = state.builder()
            when (event) {
                is InterfacePlaced -> builder.plusPlacement(
                    InterfacePlacement(
                        event.id,
                        event.index,
                        event.interfaceId,
                        event.audienceId
                    )
                )

                is CommandPlaced -> builder.plusPlacement(CommandPlacement(event.id, event.index, event.commandId))
                is EventPlaced -> builder.plusPlacement(
                    EventPlacement(
                        event.id,
                        event.index,
                        event.eventId,
                        event.streamId
                    )
                )

                is ReadModelPlaced -> builder.plusPlacement(
                    ReadModelPlacement(
                        event.id,
                        event.index,
                        event.readModelId
                    )
                )
            }
            builder.build()
        }

        is PlacementMoved -> {
            val builder = state.builder()
            when (event) {
                is EventPlacementMoved -> state.placements[event.placementId]!!
                    .let {
                        val placement = it as EventPlacement
                        builder
                            .minusPlacement(event.placementId)
                            .plusPlacement(placement.copy(index = event.index, streamId = event.streamId))
                    }

                is InterfacePlacementMoved -> state.placements[event.placementId]!!
                    .let {
                        val placement = it as InterfacePlacement
                        builder
                            .minusPlacement(event.placementId)
                            .plusPlacement(placement.copy(index = event.index, audienceId = event.audienceId))
                    }

                is TimelinePlacementMoved -> state.placements[event.placementId]!!
                    .let { placement ->
                        when (placement) {
                            is CommandPlacement ->
                                builder
                                    .minusPlacement(event.placementId)
                                    .plusPlacement(placement.copy(index = event.index))

                            is ReadModelPlacement ->
                                builder
                                    .minusPlacement(event.placementId)
                                    .plusPlacement(placement.copy(index = event.index))

                            else -> Unit
                        }
                    }
            }
            builder.build()
        }

        is PlacementRemoved -> state.builder().minusPlacement(event.id).build()
    }
