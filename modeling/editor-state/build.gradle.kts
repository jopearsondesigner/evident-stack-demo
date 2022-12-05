plugins {
    id("evident.platform.kotlin-js-library")
}

group = "$group.modeling"
version = "0.1.0-SNAPSHOT"

dependencies {
    implementation("evident.platform.domain:event-models")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core")
}
