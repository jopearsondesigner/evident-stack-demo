plugins {
    id("org.jetbrains.kotlin.jvm")
    id("java-library")
}

kotlin {
    jvmToolchain {
        languageVersion.set(JavaLanguageVersion.of("18")) // "8"
    }
}

dependencies {
    implementation(kotlin("stdlib"))
}
