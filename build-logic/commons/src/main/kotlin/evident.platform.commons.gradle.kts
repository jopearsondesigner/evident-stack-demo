import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.jetbrains.kotlin.gradle.targets.jvm.tasks.KotlinJvmTest

tasks.withType<KotlinCompile>().configureEach {
    kotlinOptions {
        languageVersion = "1.7"
        apiVersion = "1.7"
    }
}

tasks.withType<KotlinJvmTest>().configureEach {
    useJUnitPlatform()
}