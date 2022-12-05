plugins {
    id("evident.platform.kotlin-multiplatform-library")
}

group = "evident.platform.domain"
version = "0.1.0-SNAPSHOT"

dependencies {
    // Language & Platform
    commonMainImplementation(libs.kotlinx.collections.immutable)
    commonMainImplementation(libs.kotlinx.coroutines)
    commonMainImplementation(libs.kotlinx.uuid.core)
    commonMainImplementation(libs.kotlinx.serialization.json)
    commonMainImplementation(libs.kotlinx.serialization.cbor)
    commonMainImplementation(libs.arrow.core)

    // State
    commonMainImplementation(project(":state"))
    commonMainImplementation(project(":converge"))
}