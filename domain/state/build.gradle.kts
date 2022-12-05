plugins {
    id("evident.platform.kotlin-multiplatform-library")
}

group = "evident.platform.domain"
version = "0.1.0-SNAPSHOT"

dependencies {
    commonMainImplementation(libs.kotlinx.coroutines)
    commonMainImplementation(libs.arrow.core)
}
