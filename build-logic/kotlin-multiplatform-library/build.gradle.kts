plugins {
    `kotlin-dsl`
}

dependencies {
    implementation(platform("evident.platform:plugins-platform"))

    implementation("org.jetbrains.kotlin.multiplatform:org.jetbrains.kotlin.multiplatform.gradle.plugin")
    implementation("org.jetbrains.kotlin:kotlin-serialization")
    implementation("io.kotest:kotest-framework-multiplatform-plugin-gradle")
}
