package evident.platform.domain.converge

typealias Counter = Long
typealias Node = Long
data class Id(val counter: Long, val node: Node)

// Doesn't account for out-of-order patches, but that shouldn't
//  matter for our server-mediated use-case
typealias Clock = Map<Node, Counter>

sealed interface Op {
    val ordinal: Short
}

typealias Patch = Map<Id, Op>
