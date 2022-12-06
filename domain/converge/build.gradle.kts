plugins {
    id("evident.platform.kotlin-multiplatform-library")
}

group = "evident.platform.domain"
version = "0.1.0-SNAPSHOT"

kotlin {
    sourceSets {
        val commonMain by getting {
            dependencies {
                implementation(libs.kotlinx.coroutines)
                implementation(libs.kotlinx.uuid.core)
                implementation(project(":state"))
            }
        }
        val commonTest by getting
        val jvmMain by getting
        val jvmTest by getting
        val jsMain by getting
        val jsTest by getting
        val nativeMain by getting
        val nativeTest by getting
    }
}