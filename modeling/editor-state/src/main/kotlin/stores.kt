import evident.platform.adapters.svelte.WriteableFlowStore
import evident.platform.adapters.svelte.WriteableStore

@OptIn(ExperimentalJsExport::class)
@JsExport
val buttonLabel: WriteableStore<String> = WriteableFlowStore("value-0")
