plugins {
    id("java-platform")
}

group = "evident.platform"

dependencies {
    constraints {
        api("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.6.4")
    }
}
