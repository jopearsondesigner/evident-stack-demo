import com.onote.adapters.svelte.WriteableFlowStore
import com.onote.adapters.svelte.WriteableStore

@OptIn(ExperimentalJsExport::class)
@JsExport
val buttonLabel: WriteableStore<String> = WriteableFlowStore("value-0")
