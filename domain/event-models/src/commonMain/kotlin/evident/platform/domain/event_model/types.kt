package evident.platform.domain.event_model

import arrow.core.Either
import arrow.core.filterMap
import arrow.core.left
import arrow.core.right
import kotlinx.serialization.Serializable
import kotlinx.uuid.UUID
import kotlinx.uuid.generateUUID

typealias EntityId = UUID

sealed interface Entity {
    val id: EntityId
}

typealias Name = String
typealias Description = String

sealed interface NamedEntity : Entity {
    val name: Name
}

interface DescribedEntity : NamedEntity {
    val description: Description?
}

// Interfaces

typealias InterfaceId = EntityId
typealias InterfaceType = String
typealias InterfaceDimension = Double

sealed interface Interface : NamedEntity {
    override val id: InterfaceId
    val type: InterfaceType
        get() = this::class.simpleName!!

    val width: InterfaceDimension?
    val height: InterfaceDimension?

    companion object {
        fun create(
            type: InterfaceType,
            id: InterfaceId,
            name: Name,
            description: Description? = null,
            width: InterfaceDimension? = null,
            height: InterfaceDimension? = null
        ): Interface = TODO()
    }
}

sealed interface InterfaceElement : NamedEntity

sealed interface InterfaceWithElements : Interface {
    val elements: Iterable<InterfaceElement>
        get() = listOf()
}

// Commands

typealias CommandId = EntityId

@Serializable
data class Command(
    override val id: CommandId,
    override val name: Name,
    override val description: Description? = null,
    private val configAssignments: Map<CommandConfigRole, ConfigId> = mapOf(),
) : DescribedEntity,
    HasConfigsAssignedByRole<CommandConfigRole, Command> {
    override fun withConfigByRole(
        role: CommandConfigRole,
        configId: ConfigId
    ): Command {
        val newConfigAssignments = configAssignments.toMutableMap()
        newConfigAssignments[role] = configId
        return copy(configAssignments = newConfigAssignments.toMap())
    }

    override fun withoutConfigRole(role: CommandConfigRole): Command {
        val newConfigAssignments = configAssignments.toMutableMap()
        newConfigAssignments.remove(role)
        return copy(configAssignments = newConfigAssignments.toMap())
    }

    override fun getConfigByRole(model: EventModel, role: CommandConfigRole) =
        configAssignments[role]?.let { model.configs[it] }
}

// Events

typealias EventId = EntityId

@Serializable
data class Event(
    override val id: EventId,
    override val name: Name,
    override val description: Description? = null,
    private val configAssignments: Map<ConfigRole, ConfigId> = mapOf(),
) : DescribedEntity, HasConfigsAssignedByRole<EventConfigRole, Event> {
    override fun withConfigByRole(
        role: EventConfigRole,
        configId: ConfigId
    ): Event {
        val newConfigAssignments = configAssignments.toMutableMap()
        newConfigAssignments[role] = configId
        return copy(configAssignments = newConfigAssignments.toMap())
    }

    override fun withoutConfigRole(role: EventConfigRole): Event {
        val newConfigAssignments = configAssignments.toMutableMap()
        newConfigAssignments.remove(role)
        return copy(configAssignments = newConfigAssignments.toMap())
    }

    override fun getConfigByRole(model: EventModel, role: EventConfigRole) =
        configAssignments[role]?.let { model.configs[it] }
}

// Read Models

typealias ReadModelId = EntityId

@Serializable
data class ReadModel(
    override val id: ReadModelId,
    override val name: Name,
    override val description: Description? = null,
    private val configAssignments: Map<ConfigRole, ConfigId> = mapOf(),
) : DescribedEntity, HasConfigsAssignedByRole<ReadModelConfigRole, ReadModel> {
    override fun withConfigByRole(
        role: ReadModelConfigRole,
        configId: ConfigId
    ): ReadModel {
        val newConfigAssignments = configAssignments.toMutableMap()
        newConfigAssignments[role] = configId
        return copy(configAssignments = newConfigAssignments.toMap())
    }

    override fun withoutConfigRole(role: ReadModelConfigRole): ReadModel {
        val newConfigAssignments = configAssignments.toMutableMap()
        newConfigAssignments.remove(role)
        return copy(configAssignments = newConfigAssignments.toMap())
    }

    override fun getConfigByRole(model: EventModel, role: ReadModelConfigRole) =
        configAssignments[role]?.let { model.configs[it] }
}

// Configs

typealias Config = String

sealed interface CommandConfigRole : ConfigRole
sealed interface EventConfigRole : ConfigRole
sealed interface ReadModelConfigRole : ConfigRole

sealed interface ConfigRole {
    object CommandSchema : CommandConfigRole
    object ResultSchema : CommandConfigRole

    object EventBodySchema : EventConfigRole

    object QuerySchema : ReadModelConfigRole
    object ReadModelSchema : ReadModelConfigRole

    object ErrorSchema : CommandConfigRole, ReadModelConfigRole
}

sealed interface HasConfigsAssignedByRole<
        in S : ConfigRole,
        out T : HasConfigsAssignedByRole<S, T>> {
    fun withConfigByRole(role: S, config: Config): T
    fun withoutConfigRole(role: S): T
    fun getConfigByRole(role: S): Config?
}

// Lanes

typealias AudienceId = EntityId

@Serializable
data class Audience(
    override val id: AudienceId,
    override val name: Name,
) : NamedEntity

typealias StreamId = EntityId

@Serializable
data class Stream(
    override val id: StreamId,
    override val name: Name,
) : NamedEntity

// Placements

typealias PlacementId = EntityId
typealias PlacementIndex = Int // TODO: Zero + Positive

sealed interface Placement : Entity {
    override val id: PlacementId
    val index: PlacementIndex
}

sealed interface TimelinePlacement : Placement

@Serializable
data class InterfacePlacement(
    override val id: PlacementId,
    override val index: PlacementIndex,
    val interfaceId: InterfaceId,
    val audienceId: AudienceId? = null,
) : Placement {
    fun `interface`(model: EventModel): Interface =
        model.interfaces[interfaceId]!!

    fun audience(model: EventModel): Audience? =
        audienceId?.let { model.getAudience(it) }
}

@Serializable
data class CommandPlacement(
    override val id: PlacementId,
    override val index: PlacementIndex,
    val commandId: CommandId,
) : TimelinePlacement {
    fun command(model: EventModel): Command =
        model.commands[commandId]!!
}

@Serializable
data class EventPlacement(
    override val id: PlacementId,
    override val index: PlacementIndex,
    val eventId: EventId,
    val streamId: StreamId? = null,
) : Placement {
    fun event(model: EventModel): Event =
        model.events[eventId]!!

    fun stream(model: EventModel): Stream? =
        streamId?.let { model.getStream(it)!! }
}

@Serializable
data class ReadModelPlacement(
    override val id: PlacementId,
    override val index: PlacementIndex,
    val readModelId: ReadModelId,
) : TimelinePlacement {
    fun readModel(model: EventModel): ReadModel =
        model.readModels[readModelId]!!
}

// Flows

typealias FlowId = EntityId

sealed interface Anchor {
    object TOP : Anchor
    object LEFT : Anchor
    object BOTTOM : Anchor
    object RIGHT : Anchor
}

@Serializable
data class FlowPort(
    val placementId: PlacementId,
    val anchor: Anchor? = null,
    val element: InterfaceElement? = null,
)

@Serializable
data class FlowArrow internal constructor(
    val from: FlowPort,
    val to: FlowPort,
) : Entity {
    override val id: FlowId
        get() = flowId(from.placementId, to.placementId)

    companion object {
        fun flowId(fromPlacementId: PlacementId, toPlacementId: PlacementId): FlowId =
            FlowId.generateUUID(fromPlacementId, toPlacementId.toString())

        fun build(
            model: EventModel,
            from: PlacementId,
            to: PlacementId
        ): FlowArrow =
            build(model, FlowPort(from), FlowPort(to))

        fun build(
            model: EventModel,
            from: FlowPort,
            to: FlowPort
        ): FlowArrow {
            val fromPlacement = model.placements[from.placementId]
                ?: throw IllegalArgumentException("From placement doesn't exist")
            val toPlacement = model.placements[to.placementId]
                ?: throw IllegalArgumentException("To placement doesn't exist")
            if (toPlacement.index < fromPlacement.index)
                throw IllegalArgumentException("Flows cannot go backward")
            if (model.getFlow(from.placementId, to.placementId) != null)
                throw IllegalArgumentException("Flow already connects these two placements")
            return when (fromPlacement) {
                is InterfacePlacement -> when (toPlacement) {
                    is CommandPlacement -> FlowArrow(from, to)
                    else -> throw IllegalArgumentException("Interfaces can only flow to Commands")
                }

                is CommandPlacement -> when (toPlacement) {
                    is EventPlacement -> FlowArrow(from, to)
                    else -> throw IllegalArgumentException("Commands can only flow to Events")
                }

                is EventPlacement -> when (toPlacement) {
                    is CommandPlacement, is ReadModelPlacement -> FlowArrow(from, to)
                    else -> throw IllegalArgumentException("Events can only flow to Commands or Read Models")
                }

                is ReadModelPlacement -> when (toPlacement) {
                    is InterfacePlacement -> FlowArrow(from, to)
                    else -> throw IllegalArgumentException("Read Models can only flow to Interfaces")
                }
            }
        }

        fun of(
            model: EventModel,
            from: FlowPort,
            to: FlowPort
        ): Either<IllegalFlowError, FlowArrow> =
            try {
                build(model, from, to).right()
            } catch (e: IllegalArgumentException) {
                IllegalFlowError(from, to, e.message ?: "Illegal Flow").left()
            }

        fun of(
            model: EventModel,
            from: Placement,
            to: Placement
        ): Either<IllegalFlowError, FlowArrow> =
            try {
                build(model, from.id, to.id).right()
            } catch (e: IllegalArgumentException) {
                IllegalFlowError(
                    FlowPort(from.id),
                    FlowPort(to.id),
                    e.message ?: "Illegal Flow"
                ).left()
            }
    }
}

// Main Event Model API

sealed interface GridLane

@Serializable
data class DefaultGridAudience(
    val placements: Map<PlacementIndex, InterfacePlacement>
) : GridLane

@Serializable
data class GridAudience(
    val audience: Audience,
    val placements: Map<PlacementIndex, InterfacePlacement>
) : GridLane

@Serializable
data class GridTimeline(
    val placements: Map<PlacementIndex, TimelinePlacement>
) : GridLane

@Serializable
data class GridStream(
    val stream: Stream,
    val placements: Map<PlacementIndex, EventPlacement>
) : GridLane

@Serializable
data class DefaultGridStream(
    val placements: Map<PlacementIndex, EventPlacement>
) : GridLane

sealed interface EventModelLifecycle

interface EventModel : DescribedEntity, EventModelLifecycle {
    val interfaces: Map<InterfaceId, Interface>
    val commands: Map<CommandId, Command>
    val events: Map<EventId, Event>
    val readModels: Map<ReadModelId, ReadModel>
    val audiences: List<Audience>
    val streams: List<Stream>
    val placements: Map<PlacementId, Placement>
    val flows: Map<FlowId, FlowArrow>
    val configs: Map<ConfigId, Config>

    fun builder(): EventModelBuilder<EventModel>

    fun isEmpty() = Companion.isEmpty(this)

    fun getAudience(audienceId: AudienceId): Audience? =
        audiences.find { it.id == audienceId }

    fun getStream(streamId: StreamId): Stream? =
        streams.find { it.id == streamId }

    fun getFlow(from: PlacementId, to: PlacementId): FlowArrow? =
        flows[FlowArrow.flowId(from, to)]

    fun interfacePlacementsInAudience(audienceId: AudienceId): Map<PlacementId, InterfacePlacement> =
        placements
            .filterMap {
                when (it) {
                    is InterfacePlacement ->
                        if (it.audienceId == audienceId)
                            it
                        else
                            null

                    else -> null
                }
            }

    fun eventPlacementsInStream(streamId: StreamId): Map<PlacementId, EventPlacement> =
        placements
            .filterMap {
                when (it) {
                    is EventPlacement ->
                        if (it.streamId == streamId)
                            it
                        else
                            null

                    else -> null
                }
            }

    fun placementsOfInterface(interfaceId: InterfaceId): Map<PlacementId, InterfacePlacement> =
        placements.filterMap { placement ->
            when (placement) {
                is InterfacePlacement ->
                    if (placement.interfaceId == interfaceId)
                        placement
                    else
                        null

                else -> null
            }
        }

    fun placementsOfCommand(commandId: CommandId): Map<PlacementId, CommandPlacement> =
        placements.filterMap { placement ->
            when (placement) {
                is CommandPlacement ->
                    if (placement.commandId == commandId)
                        placement
                    else
                        null

                else -> null
            }
        }

    fun placementsOfEvent(eventId: EventId): Map<PlacementId, EventPlacement> =
        placements.filterMap { placement ->
            when (placement) {
                is EventPlacement ->
                    if (placement.eventId == eventId)
                        placement
                    else
                        null

                else -> null
            }
        }

    fun placementsOfReadModel(readModelId: ReadModelId): Map<PlacementId, ReadModelPlacement> =
        placements.filterMap { placement ->
            when (placement) {
                is ReadModelPlacement ->
                    if (placement.readModelId == readModelId)
                        placement
                    else
                        null

                else -> null
            }
        }

    fun interfaceByName(name: Name): Interface? =
        interfaces.values.first { it.name == name }

    fun commandByName(name: Name): Command? =
        commands.values.first { it.name == name }

    fun eventByName(name: Name): Event? =
        events.values.first { it.name == name }

    fun readModelByName(name: Name): ReadModel? =
        readModels.values.first { it.name == name }

    // TODO: Allow for a sorted set of placements (on placement ID) at each (index, lane)
    //  coordinate, rather than just a single placement
    fun grid(): List<GridLane> {
        val lanes = mutableListOf<GridLane>()
        val groupedPlacements = placements
            .values
            .groupBy { placement ->
                when (placement) {
                    is InterfacePlacement -> InterfacePlacement
                    is CommandPlacement -> CommandPlacement
                    is EventPlacement -> EventPlacement
                    is ReadModelPlacement -> ReadModelPlacement
                }
            }
        groupedPlacements[InterfacePlacement]?.let { interfacePlacements ->
            val groupedInterfacePlacements = interfacePlacements
                .map { it as InterfacePlacement }
                .groupBy { it.audienceId ?: UUID.NIL }
            lanes.add(
                DefaultGridAudience(
                    groupedInterfacePlacements[UUID.NIL]
                        ?.associateBy { it.index }
                        ?: mapOf()
                )
            )
            for (audience in audiences.reversed()) {
                lanes.add(
                    GridAudience(
                        audience,
                        groupedInterfacePlacements[audience.id]
                            ?.associateBy { it.index }
                            ?: mapOf()
                    )
                )
            }
        }

        val timelinePlacements = mutableListOf<TimelinePlacement>()
        timelinePlacements
            .addAll(groupedPlacements[CommandPlacement]
                ?.map { it as CommandPlacement }
                ?: listOf())
        timelinePlacements
            .addAll(groupedPlacements[ReadModelPlacement]
                ?.map { it as ReadModelPlacement }
                ?: listOf())
        lanes.add(
            GridTimeline(timelinePlacements.associateBy { it.index })
        )

        groupedPlacements[EventPlacement]?.let { eventPlacements ->
            val groupedEventPlacements = eventPlacements
                .map { it as EventPlacement }
                .groupBy { it.streamId ?: UUID.NIL }
            for (stream in streams) {
                lanes.add(
                    GridStream(
                        stream,
                        groupedEventPlacements[stream.id]
                            ?.associateBy { it.index }
                            ?: mapOf()
                    )
                )
            }
            lanes.add(
                DefaultGridStream(
                    groupedEventPlacements[UUID.NIL]
                        ?.associateBy { it.index }
                        ?: mapOf()
                )
            )
        }

        return lanes.toList()
    }

    companion object {
        fun isEmpty(eventModel: EventModel) = eventModel.interfaces.isEmpty() &&
                eventModel.commands.isEmpty() &&
                eventModel.events.isEmpty() &&
                eventModel.readModels.isEmpty() &&
                eventModel.audiences.isEmpty() &&
                eventModel.streams.isEmpty() &&
                eventModel.placements.isEmpty() &&
                eventModel.flows.isEmpty() &&
                eventModel.configs.isEmpty()
    }
}

interface EventModelBuilder<out T : EventModel> {
    val eventModel: T

    fun name(name: Name): EventModelBuilder<T>
    fun description(description: Description): EventModelBuilder<T>

    fun plusInterface(`interface`: Interface): EventModelBuilder<T>
    fun minusInterface(interfaceId: InterfaceId): EventModelBuilder<T>

    fun plusCommand(command: Command): EventModelBuilder<T>
    fun minusCommand(commandId: CommandId): EventModelBuilder<T>

    fun plusEvent(event: Event): EventModelBuilder<T>
    fun minusEvent(eventId: EventId): EventModelBuilder<T>

    fun plusReadModel(readModel: ReadModel): EventModelBuilder<T>
    fun minusReadModel(readModelId: ReadModelId): EventModelBuilder<T>

    fun plusAudience(audience: Audience, index: Int): EventModelBuilder<T>
    fun minusAudience(audienceId: AudienceId): EventModelBuilder<T>

    fun plusStream(stream: Stream, index: Int): EventModelBuilder<T>
    fun minusStream(streamId: StreamId): EventModelBuilder<T>

    fun plusPlacement(placement: Placement): EventModelBuilder<T>
    fun minusPlacement(placementId: PlacementId): EventModelBuilder<T>

    fun plusFlow(flowArrow: FlowArrow): EventModelBuilder<T>
    fun minusFlow(from: PlacementId, to: PlacementId): EventModelBuilder<T> =
        minusFlow(FlowArrow.flowId(from, to))

    fun minusFlow(flowId: FlowId): EventModelBuilder<T>

    fun plusConfig(config: Config): EventModelBuilder<T>
    fun minusConfig(configId: ConfigId): EventModelBuilder<T>

    fun build(): T
}

@Serializable
data class ImmutableEventModel internal constructor(
    override val id: EntityId,
    override val name: Name,
    override val description: Description? = null,
    override val interfaces: Map<InterfaceId, Interface> = mapOf(),
    override val commands: Map<CommandId, Command> = mapOf(),
    override val events: Map<EventId, Event> = mapOf(),
    override val readModels: Map<ReadModelId, ReadModel> = mapOf(),
    override val audiences: List<Audience> = listOf(),
    override val streams: List<Stream> = listOf(),
    override val placements: Map<PlacementId, Placement> = mapOf(),
    override val flows: Map<FlowId, FlowArrow> = mapOf(),
    override val configs: Map<ConfigId, Config> = mapOf(),
) : EventModel {
    override fun builder() = builder(this)

    companion object {
        fun empty(id: EntityId, name: Name, description: Description? = null) =
            ImmutableEventModel(id, name, description)

        fun builder(eventModel: ImmutableEventModel) = Builder(eventModel)
    }

    data class Builder(
        override val eventModel: ImmutableEventModel
    ) : EventModelBuilder<ImmutableEventModel> {
        private var name = eventModel.name
        private var description = eventModel.description
        private var interfaces = eventModel.interfaces
        private var commands = eventModel.commands
        private var events = eventModel.events
        private var readModels = eventModel.readModels
        private var audiences = eventModel.audiences
        private var streams = eventModel.streams
        private var placements = eventModel.placements
        private var flows = eventModel.flows
        private var configs = eventModel.configs

        override fun name(name: Name): EventModelBuilder<ImmutableEventModel> =
            apply { this.name = name }

        override fun description(description: Description): EventModelBuilder<ImmutableEventModel> =
            apply { this.description = description }

        override fun plusInterface(`interface`: Interface): EventModelBuilder<ImmutableEventModel> =
            apply { this.interfaces = interfaces + Pair(`interface`.id, `interface`) }

        override fun minusInterface(interfaceId: InterfaceId): EventModelBuilder<ImmutableEventModel> =
            apply { this.interfaces = interfaces - interfaceId }

        override fun plusCommand(command: Command): EventModelBuilder<ImmutableEventModel> =
            apply { this.commands = commands + Pair(command.id, command) }

        override fun minusCommand(commandId: CommandId): EventModelBuilder<ImmutableEventModel> =
            apply { this.commands = commands - commandId }

        override fun plusEvent(event: Event): EventModelBuilder<ImmutableEventModel> =
            apply { this.events = events + Pair(event.id, event) }

        override fun minusEvent(eventId: EventId): EventModelBuilder<ImmutableEventModel> =
            apply { this.events = events - eventId }

        override fun plusReadModel(readModel: ReadModel): EventModelBuilder<ImmutableEventModel> =
            apply { this.readModels = readModels + Pair(readModel.id, readModel) }

        override fun minusReadModel(readModelId: ReadModelId): EventModelBuilder<ImmutableEventModel> =
            apply { this.readModels = readModels - readModelId }

        override fun plusAudience(audience: Audience, index: Int): EventModelBuilder<ImmutableEventModel> =
            apply {
                this.audiences = audiences.subList(0, index - 1) +
                        audience +
                        audiences.subList(index, audiences.lastIndex)
            }

        override fun minusAudience(audienceId: AudienceId): EventModelBuilder<ImmutableEventModel> =
            apply {
                val index = audiences.indexOfFirst { it.id == audienceId }
                this.audiences = audiences.subList(0, index - 1) +
                        audiences.subList(index + 1, audiences.lastIndex)
            }

        override fun plusStream(stream: Stream, index: Int): EventModelBuilder<ImmutableEventModel> =
            apply {
                this.streams = streams.subList(0, index - 1) +
                        stream +
                        streams.subList(index, streams.lastIndex)
            }

        override fun minusStream(streamId: StreamId): EventModelBuilder<ImmutableEventModel> =
            apply {
                val index = streams.indexOfFirst { it.id == streamId }
                this.streams = streams.subList(0, index - 1) +
                        streams.subList(index + 1, streams.lastIndex)
            }

        override fun plusPlacement(placement: Placement): EventModelBuilder<ImmutableEventModel> =
            apply { this.placements = placements + Pair(placement.id, placement) }

        override fun minusPlacement(placementId: PlacementId): EventModelBuilder<ImmutableEventModel> =
            apply { this.placements = placements - placementId }

        override fun plusFlow(flowArrow: FlowArrow): EventModelBuilder<ImmutableEventModel> =
            apply { this.flows = flows + Pair(flowArrow.id, flowArrow) }

        override fun minusFlow(flowId: FlowId): EventModelBuilder<ImmutableEventModel> =
            apply { this.flows = flows - flowId }

        override fun plusConfig(config: Config): EventModelBuilder<ImmutableEventModel> =
            apply { this.configs = configs + Pair(config.id, config) }

        override fun minusConfig(configId: ConfigId): EventModelBuilder<ImmutableEventModel> =
            apply { this.configs = configs - configId }

        override fun build(): ImmutableEventModel =
            ImmutableEventModel(
                eventModel.id, name, description,
                interfaces, commands, events, readModels,
                audiences, streams,
                placements, flows, configs
            )
    }
}

// Errors

sealed interface Error
typealias ErrorMessage = String

sealed interface EventModelError: Error
sealed interface AudienceError : Error
sealed interface StreamError : Error
sealed interface PlacementError : Error
sealed interface FlowError : Error

@Serializable
data class IllegalEventModelError(
    val name: Name,
    val description: Description?
) : EventModelError

@Serializable
data class IllegalPlacementError(
    val id: PlacementId,
    val message: ErrorMessage
) : PlacementError

@Serializable
data class IllegalFlowError(
    val from: FlowPort,
    val to: FlowPort,
    val message: ErrorMessage
) : FlowError

sealed interface NotFoundError : Error

@Serializable
data class AudienceNotFoundError(
    val id: AudienceId,
) : AudienceError, NotFoundError

@Serializable
data class StreamNotFoundError(
    val id: StreamId,
) : StreamError, NotFoundError

@Serializable
data class PlacementNotFoundError(
    val id: PlacementId,
) : PlacementError, NotFoundError

@Serializable
data class FlowNotFoundError(
    val from: PlacementId,
    val to: PlacementId,
) : FlowError, NotFoundError

sealed interface DuplicateNameError : Error

@Serializable
data class DuplicateEventModelNameError(
    val name: Name
) : EventModelError, DuplicateNameError

@Serializable
data class DuplicateAudienceNameError(
    val name: Name
) : AudienceError, DuplicateNameError

@Serializable
data class DuplicateStreamNameError(
    val name: Name
) : StreamError, DuplicateNameError

@Serializable
data class DuplicateInterfaceNameError(
    val name: Name
) : PlacementError, DuplicateNameError

@Serializable
data class DuplicateCommandNameError(
    val name: Name
) : PlacementError, DuplicateNameError

@Serializable
data class DuplicateEventNameError(
    val name: Name
) : PlacementError, DuplicateNameError

@Serializable
data class DuplicateReadModelNameError(
    val name: Name
) : PlacementError, DuplicateNameError

sealed interface RemovalConfirmationError : Error

@Serializable
data class AudienceRemovalConfirmationError(
    val id: AudienceId
) : AudienceError, RemovalConfirmationError

@Serializable
data class StreamRemovalConfirmationError(
    val id: StreamId
) : StreamError, RemovalConfirmationError

@Serializable
data class PlacementRemovalConfirmationError(
    val id: PlacementId
) : PlacementError, RemovalConfirmationError

@Serializable
data class FlowRemovalConfirmationError(
    val from: PlacementId,
    val to: PlacementId
) : FlowError, RemovalConfirmationError
