package evident.platform.domain.event_model

import arrow.core.Either
import com.twolambdas.domain.ISimpleDecider
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.collections.shouldContainExactly
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.fold
import kotlinx.coroutines.flow.toList

private fun <C, S, E, Err> ISimpleDecider<C, S, E, Err>.givenEvents(
    events: Iterable<E>, command: () -> C
): Either<Err, Flow<E>> {
    val currentState = events.fold(initialState) { s, e -> evolve(s, e) }
    return decide(command(), currentState)
}

private suspend fun <C, S, E, Err> ISimpleDecider<C, S, E, Err>.givenState(
    state: S?, command: () -> C
): Either<Err, S> {
    val currentState = state ?: initialState
    val events = decide(command(), currentState)
    return events.map { es -> es.fold(currentState) { s, e -> evolve(s, e) } }
}

@Suppress("unused")
private fun <C, S, E, Err> ISimpleDecider<C, S, E, Err>.whenCommand(command: C): C = command

private suspend infix fun <E, Err> Either<Err, Flow<E>>.thenEvents(expected: Iterable<E>) = map { it.toList() shouldContainExactly (expected) }
private infix fun <S, U : S, Err> Either<Err, S>.thenState(expected: U?) = tap { it.shouldBe(expected) }
private infix fun <S, U : Err, Err> Either<Err, S>.thenError(expected: U?) = tapLeft { it.shouldBe(expected) }

class TestCreationContext(models: List<EventModel> = listOf()): EventModelCreationContext {
    private val models: MutableMap<Name, EventModel> = models
        .associateBy { it.name }
        .toMutableMap()

    override fun isNameUnique(name: Name): Boolean =
        !models.containsKey(name)

    override fun create(
        id: EntityId,
        name: Name,
        description: Description?
    ): EventModel {
        val model = ImmutableEventModel.empty(EntityId.NIL, name, description)
        models[name] = model
        return model
    }

}

class ApiTest : FunSpec({
    test("State-based Event Model creation") {
        val context = TestCreationContext()
        with(decider(context)) {
            givenState(context) {
                whenCommand(CreateEventModel("foo", "bar baz"))
            } thenState ImmutableEventModel.empty(EntityId.NIL, "foo", "bar baz")
        }
    }

    test("State-based Event Model creation fails on duplicate name") {
        val context = TestCreationContext(
            listOf(ImmutableEventModel.empty(EntityId.NIL, "foo", "baz quux"))
        )
        with(decider(context)) {
            givenState(context) {
                whenCommand(CreateEventModel("foo", "bar baz"))
            } thenError DuplicateEventModelNameError("foo")
        }
    }
})