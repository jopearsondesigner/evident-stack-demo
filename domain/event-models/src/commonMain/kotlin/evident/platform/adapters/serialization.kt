package evident.platform.adapters

import evident.platform.domain.event_model.ImmutableEventModel
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.cbor.Cbor
import kotlinx.serialization.encodeToByteArray
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

fun ImmutableEventModel.toJson(): String =
    Json.encodeToString(this)

@OptIn(ExperimentalSerializationApi::class)
fun ImmutableEventModel.toCbor(): ByteArray =
    Cbor.encodeToByteArray(this)
