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

sealed interface AudienceCommand: EventModelCommand

@Serializable
data class AddAudience(
    val name: Name,
    val index: Int,
): AudienceCommand

@Serializable
data class ReorderAudience(
    val audienceId: AudienceId,
    val index: Int
): AudienceCommand

@Serializable
data class RenameAudience(
    val audienceId: AudienceId,
    val name: Name
): AudienceCommand

@Serializable
data class RemoveAudience(
    val audienceId: AudienceId,
    val confirmed: Boolean = false
): AudienceCommand

// Events

sealed interface AudienceEvent: EventModelEvent

// TODO: change `index` here to `after` w/ Op Id
@Serializable
data class AudienceAdded(
    val id: AudienceId,
    val name: Name,
    val index: Int,
): AudienceEvent

// TODO: is this an Assign Op?
@Serializable
data class AudienceReordered(
    val audienceId: AudienceId,
    val index: Int
): AudienceEvent

@Serializable
data class AudienceRenamed(
    val audienceId: AudienceId,
    val name: Name
): AudienceEvent

@Serializable
data class AudienceRemoved(
    val audienceId: AudienceId
): AudienceEvent

// API

fun decideOnAudienceCommand(
    command: AudienceCommand,
    state: EventModel
): Either<AudienceError, Flow<EventModelEvent>> =
    when(command) {
        is AddAudience -> state.audiences.find { it.name == command.name }
            ?.let { DuplicateAudienceNameError(it.name).left() }
            ?: flowOf(AudienceAdded(
                AudienceId.generateUUID(),
                command.name,
                command.index
            )).right()
        is RenameAudience -> state.getAudience(command.audienceId)
            ?.let { audience ->
                state.audiences.find { it.name == command.name }
                    ?.let { DuplicateAudienceNameError(it.name).left() }
                    ?: flowOf(AudienceRenamed(audience.id, command.name)).right()
            } ?: AudienceNotFoundError(command.audienceId).left()
        is ReorderAudience -> state.getAudience(command.audienceId)
            ?.let { flowOf(AudienceReordered(it.id, command.index)).right() }
            ?: AudienceNotFoundError(command.audienceId).left()
        is RemoveAudience -> state.getAudience(command.audienceId)
            ?.let {
                if (command.confirmed) {
                    val events = mutableListOf<EventModelEvent>(AudienceRemoved(it.id))
                    state.interfacePlacementsInAudience(it.id).forEach {(_, placement) ->
                        events + PlacementRemoved(placement.id)
                    }
                    events.asFlow().right()
                } else {
                    AudienceRemovalConfirmationError(it.id).left()
                }
            }
            ?: AudienceNotFoundError(command.audienceId).left()
    }

fun evolveOnAudienceEvent(state: EventModel, event: AudienceEvent): EventModel {
    val builder = state.builder()
    when (event) {
        is AudienceAdded -> builder.plusAudience(Audience(event.id, event.name), event.index)
        is AudienceRemoved -> builder.minusAudience(event.audienceId)
        is AudienceRenamed -> builder
            .minusAudience(event.audienceId)
            .plusAudience(
                state.getAudience(event.audienceId)!!
                    .copy(name = event.name),
                state.audiences.indexOfFirst { it.id == event.audienceId }
            )
        is AudienceReordered -> builder
            .minusAudience(event.audienceId)
            .plusAudience(
                state.getAudience(event.audienceId)!!,
                event.index
            )
    }
    return builder.build()
}