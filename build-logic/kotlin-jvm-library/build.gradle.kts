import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    `kotlin-dsl`
}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(11))
    }
}

tasks.withType<KotlinCompile>().configureEach {
    kotlinOptions {
        languageVersion = "1.7"
        apiVersion = "1.7"
    }
}

dependencies {
    implementation(platform("evident.platform:plugins-platform"))

    implementation("org.jetbrains.kotlin.jvm:org.jetbrains.kotlin.jvm.gradle.plugin")
}
