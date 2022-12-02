plugins {
    `kotlin-dsl`
}

dependencies {
    implementation(platform("evident.platform:plugins-platform"))

    implementation("org.jetbrains.kotlin.js:org.jetbrains.kotlin.js.gradle.plugin")
}
