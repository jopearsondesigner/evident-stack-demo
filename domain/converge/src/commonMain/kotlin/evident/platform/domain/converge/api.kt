package evident.platform.domain.converge

import evident.platform.domain.state.SimpleReadModel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.fold

interface OpSet {
    // Reads
    suspend fun maxCounter(): Counter
    suspend fun clock(): Clock =
        sortedOps().fold(mutableMapOf<Node, Counter>())
        { acc, (id, _) -> acc.apply { acc[id.node] = id.counter } }
            .toMap()

    fun sortedOps(): Flow<Pair<Id, Op>>

    // Writes
    suspend fun applyPatch(patch: Patch): Clock
}

interface Interpretation {
    val basis: Clock

    fun builder(): Builder

    interface Builder {
        fun build(): Interpretation
    }
}

fun interpreterReadModel(
    initialState: Interpretation.Builder,
    evolve: (Interpretation.Builder, Pair<Id, Op>) -> Interpretation.Builder
) = SimpleReadModel(evolve, initialState)

interface ClientServerSyncProtocol {
    suspend fun getServerCounter(node: Node): Counter
    suspend fun syncClientPatch(clientPatch: Patch, clock: Clock): Patch
}
