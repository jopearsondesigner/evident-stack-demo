package evident.platform.domain.event_model

import kotlinx.serialization.Serializable

// Events

sealed interface ComponentEvent: EventModelEvent

sealed interface ComponentDefined: ComponentEvent
sealed interface ComponentRemoved: ComponentEvent

@Serializable
data class InterfaceDefined(
    val type: InterfaceType,
    val id: InterfaceId,
    val name: Name,
    val description: Description?,
    val width: InterfaceDimension?,
    val height: InterfaceDimension?
): ComponentDefined

@Serializable
data class InterfaceRemoved(
    val id: InterfaceId
): ComponentRemoved

@Serializable
data class CommandDefined(
    val id: CommandId,
    val name: Name,
    val description: Description?,
): ComponentDefined

@Serializable
data class CommandRemoved(
    val id: CommandId
): ComponentRemoved

@Serializable
data class EventDefined(
    val id: EventId,
    val name: Name,
    val description: Description?,
): ComponentDefined

@Serializable
data class EventRemoved(
    val id: EventId
): ComponentRemoved

@Serializable
data class ReadModelDefined(
    val id: ReadModelId,
    val name: Name,
    val description: Description?,
): ComponentDefined

@Serializable
data class ReadModelRemoved(
    val id: ReadModelId
): ComponentRemoved

fun evolveOnComponentEvent(state: EventModel, event: ComponentEvent): EventModel {
    val builder = state.builder()
    when (event) {
        is InterfaceDefined -> builder.plusInterface(
            Interface.create(event.type, event.id, event.name, event.description, event.width, event.height)
        )
        is InterfaceRemoved -> builder.minusInterface(event.id)
        is CommandDefined -> builder.plusCommand(Command(event.id, event.name, event.description))
        is CommandRemoved -> builder.minusCommand(event.id)
        is EventDefined -> builder.plusEvent(Event(event.id, event.name, event.description))
        is EventRemoved -> builder.minusEvent(event.id)
        is ReadModelDefined -> builder.plusReadModel(ReadModel(event.id, event.name, event.description))
        is ReadModelRemoved -> builder.minusReadModel(event.id)
    }
    return builder.build()
}