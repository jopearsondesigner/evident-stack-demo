plugins {
    id("java-platform")
}

group = "evident.platform"

dependencies {
    constraints {
        api("com.android.tools.build:gradle:7.3.1")
        api("org.jetbrains.kotlin.android:org.jetbrains.kotlin.android.gradle.plugin:1.7.22")
        api("org.jetbrains.kotlin.jvm:org.jetbrains.kotlin.jvm.gradle.plugin:1.7.22")
        api("org.jetbrains.kotlin.multiplatform:org.jetbrains.kotlin.multiplatform.gradle.plugin:1.7.22")
        api("org.jetbrains.kotlin.js:org.jetbrains.kotlin.js.gradle.plugin:1.7.22")
    }
}
