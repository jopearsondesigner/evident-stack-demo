package evident.platform.adapters.svelte

import kotlinx.coroutines.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

@OptIn(ExperimentalJsExport::class)
@JsExport
interface ReadableStore<T> {
    val value: T
    fun subscribe(subscription: (value: T) -> Unit): () -> Unit
}

@OptIn(ExperimentalJsExport::class)
@JsExport
interface WriteableStore<T>: ReadableStore<T> {
    fun set(value: T)
    fun compareAndSet(expected: T, update: T)
}

interface FlowStore<T> {
    val stateFlow: StateFlow<T>
}

class WriteableFlowStore<T>(initialState: T): WriteableStore<T>, FlowStore<T> {
    private val mutableStateFlow = MutableStateFlow(initialState)
    private val scope = CoroutineScope(Dispatchers.Default)
    override val stateFlow = mutableStateFlow.asStateFlow()
    override val value: T
        get() = stateFlow.value

    override fun subscribe(subscription: (value: T) -> Unit): () -> Unit {
        val job = scope.launch {
            stateFlow.collect(subscription)
        }
        return { job.cancel() }
    }

    override fun set(value: T) {
        mutableStateFlow.value = value
    }

    override fun compareAndSet(expected: T, update: T) {
        mutableStateFlow.compareAndSet(expected, update)
    }

    fun close() {
        scope.cancel()
    }
}

class DerivedFlowStore<T>(
    override val stateFlow: StateFlow<T>,
): ReadableStore<T>, FlowStore<T> {
    private val scope = CoroutineScope(Dispatchers.Default)
    override val value: T
        get() = stateFlow.value

    override fun subscribe(subscription: (value: T) -> Unit): () -> Unit {
        val job = scope.launch {
            stateFlow.collect(subscription)
        }
        return { job.cancel() }
    }

    fun close() {
        scope.cancel()
    }

    companion object {
        fun <T> from(flowStore: WriteableFlowStore<T>) = DerivedFlowStore(flowStore.stateFlow)
    }
}
