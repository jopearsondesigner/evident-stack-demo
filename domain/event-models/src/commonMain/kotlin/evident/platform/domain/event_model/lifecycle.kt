package evident.platform.domain.event_model

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flowOf
import kotlinx.serialization.Serializable
import kotlinx.uuid.generateUUID

// Commands

sealed interface LifecycleCommand : EventModelCommand

@Serializable
data class CreateEventModel(
    val name: Name,
    val description: Description?,
): LifecycleCommand

@Serializable
data class DeleteEventModel(
    val id: EntityId
): LifecycleCommand

// Events

sealed interface LifecycleEvent : EventModelEvent

@Serializable
data class EventModelCreated(
    val id: EntityId,
    val name: Name,
    val description: Description?,
): LifecycleEvent

@Serializable
data class EventModelDeleted(
    val id: EntityId
): LifecycleEvent

// Read Models

interface EventModelCreationContext: EventModelLifecycle {
    fun isNameUnique(name: Name): Boolean
    fun create(id: EntityId, name: Name, description: Description? = null): EventModel
}
interface DeletedEventModel: EventModelLifecycle

// API

fun decideOnLifecycleCommand(
    command: LifecycleCommand,
    state: EventModelLifecycle
): Either<EventModelError, Flow<EventModelEvent>> =
    when(command) {
        is CreateEventModel -> when (state) {
            is EventModelCreationContext ->
                if (state.isNameUnique(command.name)) {
                    flowOf(
                        EventModelCreated(EntityId.generateUUID(), command.name, command.description)
                    ).right()
                } else {
                    DuplicateEventModelNameError(command.name).left()
                }
            is EventModel -> flowOf<EventModelEvent>().right()
            is DeletedEventModel -> flowOf<EventModelEvent>().right()
        }
        is DeleteEventModel -> when (state) {
            is DeletedEventModel -> flowOf<EventModelEvent>().right()
            is EventModel -> flowOf(EventModelDeleted(command.id)).right()
            is EventModelCreationContext -> flowOf<EventModelEvent>().right()
        }
    }

fun evolveOnLifecycleEvent(
    state: EventModelLifecycle,
    event: LifecycleEvent
): EventModelLifecycle =
    when (event) {
        is EventModelCreated -> when (state) {
            is EventModelCreationContext -> state.create(event.id, event.name, event.description)
            is EventModel -> state
            is DeletedEventModel -> state
        }
        is EventModelDeleted -> when (state) {
            is EventModelCreationContext -> state
            is EventModel -> state
            is DeletedEventModel -> state
        }
    }
