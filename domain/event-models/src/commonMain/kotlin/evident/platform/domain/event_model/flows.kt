package evident.platform.domain.event_model

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flowOf
import kotlinx.serialization.Serializable

// Commands

sealed interface FlowCommand: EventModelCommand

@Serializable
data class ConnectFlow(
    val from: FlowPort,
    val to: FlowPort
): FlowCommand

@Serializable
data class DisconnectFlow(
    val from: PlacementId,
    val to: PlacementId,
    val confirmed: Boolean = false
): FlowCommand

// Events

sealed interface FlowEvent: EventModelEvent

@Serializable
data class FlowConnected(
    val from: FlowPort,
    val to: FlowPort
): FlowEvent

@Serializable
data class FlowDisconnected(
    val from: PlacementId,
    val to: PlacementId
): FlowEvent

// API

fun decideOnFlowCommand(
    command: FlowCommand,
    state: EventModel
): Either<FlowError, Flow<EventModelEvent>> =
    when(command) {
        is ConnectFlow -> FlowArrow.of(state, command.from, command.to)
            .map { flowOf(FlowConnected(command.from, command.to)) }
        is DisconnectFlow -> if (command.confirmed) {
            state.getFlow(command.from, command.to)
                ?.let { flowOf(FlowDisconnected(command.from, command.to)).right() }
                ?: FlowNotFoundError(command.from, command.to).left()
        } else {
            FlowRemovalConfirmationError(command.from, command.to).left()
        }
    }

fun evolveOnFlowEvent(state: EventModel, event: FlowEvent): EventModel {
    val builder = state.builder()
    return when (event) {
        is FlowConnected -> builder.plusFlow(FlowArrow.build(state, event.from, event.to)).build()
        is FlowDisconnected -> builder.minusFlow(event.from, event.to).build()
    }
}