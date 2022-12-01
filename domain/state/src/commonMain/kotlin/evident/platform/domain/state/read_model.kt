package evident.platform.domain.state

interface IReadModel<in Si, out So, in E> {
    val evolve: (Si, E) -> So
    val initialState: So
}

typealias ISimpleReadModel<S, E> = IReadModel<S, S, E>
typealias SimpleReadModel<S, E> = ReadModel<S, S, E>

data class ReadModel<in Si, out So, in E>(
    override val evolve: (Si, E) -> So,
    override val initialState: So
): IReadModel<Si, So, E>