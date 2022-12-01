package evident.platform.domain.state

import kotlinx.coroutines.flow.Flow

interface ISaga<in AR, out A> {
    val react: (AR) -> Flow<A>
}

data class Saga<in AR, out A>(
    override val react: (AR) -> Flow<A>
) : ISaga<AR, A>
