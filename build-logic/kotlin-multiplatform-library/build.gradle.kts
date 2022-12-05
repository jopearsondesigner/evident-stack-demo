plugins {
    `kotlin-dsl`
}

kotlin {
    this.coreLibrariesVersion = "${properties["kotlinVersion"]}"
}

dependencies {
    implementation(platform("evident.platform:plugins-platform"))

    implementation(project(":commons"))
    implementation("org.jetbrains.kotlin:kotlin-serialization")
    implementation("org.jetbrains.kotlin.multiplatform:org.jetbrains.kotlin.multiplatform.gradle.plugin")
    implementation("io.kotest:kotest-framework-multiplatform-plugin-gradle")
}
