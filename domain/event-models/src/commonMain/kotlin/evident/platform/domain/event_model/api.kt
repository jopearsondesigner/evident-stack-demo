package evident.platform.domain.event_model

import arrow.core.Either
import arrow.core.right
import evident.platform.domain.state.SimpleDecider
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flowOf

// Commands and Events

sealed interface EventModelCommand
sealed interface EventModelEvent

// Decider

fun decideOnEventModelCommand(
    command: EventModelCommand,
    state: EventModelLifecycle
): Either<Error, Flow<EventModelEvent>> =
    when(state) {
        is EventModelCreationContext -> when (command) {
            is LifecycleCommand -> decideOnLifecycleCommand(command, state)
            else -> flowOf<EventModelEvent>().right()
        }
        is EventModel -> when (command) {
            is LifecycleCommand -> decideOnLifecycleCommand(command, state)
            is AudienceCommand -> decideOnAudienceCommand(command, state)
            is StreamCommand -> decideOnStreamCommand(command, state)
            is PlacementCommand -> decideOnPlacementCommand(command, state)
            is FlowCommand -> decideOnFlowCommand(command, state)
        }
        is DeletedEventModel -> flowOf<EventModelEvent>().right()
    }

fun evolveOnEventModelEvent(
    state: EventModelLifecycle,
    event: EventModelEvent
): EventModelLifecycle =
    when(state) {
        is EventModelCreationContext -> when (event) {
            is LifecycleEvent -> evolveOnLifecycleEvent(state, event)
            else -> state
        }
        is EventModel -> when (event) {
            is LifecycleEvent -> evolveOnLifecycleEvent(state, event)
            is AudienceEvent -> evolveOnAudienceEvent(state, event)
            is StreamEvent -> evolveOnStreamEvent(state, event)
            is ComponentEvent -> evolveOnComponentEvent(state, event)
            is PlacementEvent -> evolveOnPlacementEvent(state, event)
            is FlowEvent -> evolveOnFlowEvent(state, event)
        }
        is DeletedEventModel -> state
    }

fun decider(initialState: EventModelLifecycle) =
    SimpleDecider(
        decide = ::decideOnEventModelCommand,
        evolve = ::evolveOnEventModelEvent,
        initialState = initialState
    )
