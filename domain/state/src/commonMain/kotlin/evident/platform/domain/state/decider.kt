package evident.platform.domain.state

import arrow.core.Either
import kotlinx.coroutines.flow.Flow

interface IDecider<in C, in Si, out So, in Ei, out Eo, out Err> {
    val decide: (C, Si) -> Either<Err, Flow<Eo>>
    val evolve: (Si, Ei) -> So
    val initialState: So
}

typealias ISimpleDecider<C, S, E, Err> = IDecider<C, S, S, E, E, Err>
typealias SimpleDecider<C, S, E, Err> = Decider<C, S, S, E, E, Err>

data class Decider<in C, in Si, out So, in Ei, out Eo, out Err>(
    override val decide: (C, Si) -> Either<Err, Flow<Eo>>,
    override val evolve: (Si, Ei) -> So,
    override val initialState: So
): IDecider<C, Si, So, Ei, Eo, Err>

