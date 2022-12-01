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

sealed interface StreamCommand: EventModelCommand

@Serializable
data class AddStream(
    val name: Name,
    val index: Int,
): StreamCommand

@Serializable
data class ReorderStream(
    val streamId: StreamId,
    val index: Int
): StreamCommand

@Serializable
data class RenameStream(
    val streamId: StreamId,
    val name: Name
): StreamCommand

@Serializable
data class RemoveStream(
    val streamId: StreamId,
    val confirmed: Boolean = false,
): StreamCommand

// Events

sealed interface StreamEvent: EventModelEvent

@Serializable
data class StreamAdded(
    val id: StreamId,
    val name: Name,
    val index: Int,
): StreamEvent

@Serializable
data class StreamReordered(
    val streamId: StreamId,
    val index: Int
): StreamEvent

@Serializable
data class StreamRenamed(
    val streamId: StreamId,
    val name: Name
): StreamEvent

@Serializable
data class StreamRemoved(
    val streamId: StreamId
): StreamEvent

// API

fun decideOnStreamCommand(
    command: StreamCommand,
    state: EventModel
): Either<StreamError, Flow<EventModelEvent>> =
    when(command) {
        is AddStream -> state.streams.find { it.name == command.name }
            ?.let { DuplicateStreamNameError(it.name).left() }
            ?: flowOf(StreamAdded(
                StreamId.generateUUID(),
                command.name,
                command.index
            )).right()
        is RenameStream -> state.getStream(command.streamId)
            ?.let { stream ->
                state.streams.find { it.name == command.name }
                    ?.let { DuplicateStreamNameError(it.name).left() }
                    ?: flowOf(StreamRenamed(stream.id, command.name)).right()
            } ?: StreamNotFoundError(command.streamId).left()
        is ReorderStream -> state.getStream(command.streamId)
            ?.let { flowOf(StreamReordered(it.id, command.index)).right() }
            ?: StreamNotFoundError(command.streamId).left()
        is RemoveStream -> state.getStream(command.streamId)
            ?.let {
                if (command.confirmed) {
                    val events = mutableListOf<EventModelEvent>(StreamRemoved(it.id))
                    state.eventPlacementsInStream(it.id).forEach {(_, placement) ->
                        events + PlacementRemoved(placement.id)
                    }
                    events.asFlow().right()
                } else {
                    StreamRemovalConfirmationError(it.id).left()
                }
            }
            ?: StreamNotFoundError(command.streamId).left()
    }

fun evolveOnStreamEvent(state: EventModel, event: StreamEvent): EventModel {
    val builder = state.builder()
    when (event) {
        is StreamAdded -> builder.plusStream(Stream(event.id, event.name), event.index)
        is StreamRemoved -> builder.minusStream(event.streamId)
        is StreamRenamed -> builder
            .minusStream(event.streamId)
            .plusStream(
                state.getStream(event.streamId)!!
                    .copy(name = event.name),
                state.streams.indexOfFirst { it.id == event.streamId }
            )
        is StreamReordered -> builder
            .minusStream(event.streamId)
            .plusStream(
                state.getStream(event.streamId)!!,
                event.index
            )
    }
    return builder.build()
}