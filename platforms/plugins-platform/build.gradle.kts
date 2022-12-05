plugins {
    id("java-platform")
}

group = "evident.platform"

// allow the definition of dependencies to other platforms like the JUnit 5 BOM
javaPlatform.allowDependencies()

dependencies {
    api(platform("io.kotest:kotest-bom:5.5.4"))
    constraints {
        api("org.jetbrains.kotlin.jvm:org.jetbrains.kotlin.jvm.gradle.plugin:1.7.22")
        api("org.jetbrains.kotlin.js:org.jetbrains.kotlin.js.gradle.plugin:1.7.22")
        api("org.jetbrains.kotlin.multiplatform:org.jetbrains.kotlin.multiplatform.gradle.plugin:1.7.22")
        api("org.jetbrains.kotlin:kotlin-serialization:1.7.22")
        api("io.kotest:kotest-framework-multiplatform-plugin-gradle:5.5.4")
    }
}
