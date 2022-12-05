plugins {
    `kotlin-dsl`
}

kotlin {
    this.coreLibrariesVersion = "${properties["kotlinVersion"]}"
}

dependencies {
    implementation(platform("evident.platform:plugins-platform"))

    implementation("org.jetbrains.kotlin.jvm:org.jetbrains.kotlin.jvm.gradle.plugin")
}
