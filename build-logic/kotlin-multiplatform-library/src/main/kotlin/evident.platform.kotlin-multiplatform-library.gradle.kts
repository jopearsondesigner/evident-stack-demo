plugins {
    id("evident.platform.commons")
    kotlin("multiplatform")
    kotlin("plugin.serialization")
    id("io.kotest.multiplatform")
}

dependencies {
    commonMainImplementation(platform("evident.platform:product-platform"))
    commonTestImplementation(platform("evident.platform:test-platform"))
    commonTestImplementation("io.kotest:kotest-assertions-core")
    commonTestImplementation("io.kotest:kotest-framework-engine")
    commonTestImplementation("io.kotest:kotest-framework-datatest")
    commonTestImplementation(kotlin("test-common"))
    commonTestImplementation(kotlin("test-annotations-common"))
}

kotlin {
    jvmToolchain {
        languageVersion.set(JavaLanguageVersion.of(18))
    }
    jvm {
        withJava()
    }
    js(IR) {
        binaries.library()
        browser {
            testTask {
                useKarma {
                    useFirefox()
                }
            }
        }
        // nodejs() // TODO: enable NodeJS build for Event Model SDK
    }
    val hostOs = System.getProperty("os.name")
    val isMingwX64 = hostOs.startsWith("Windows")
    val nativeTarget = when {
        hostOs == "Mac OS X" -> macosX64("native")
        hostOs == "Linux" -> linuxX64("native")
        isMingwX64 -> mingwX64("native")
        else -> throw GradleException("Host OS is not supported in Kotlin/Native.")
    }
    sourceSets {
        val jvmTest by getting {
            dependencies {
                implementation("io.kotest:kotest-runner-junit5")
            }
        }
    }
}

tasks.named<Test>("jvmTest") {
    useJUnitPlatform()
    filter {
        isFailOnNoMatchingTests = false
    }
    testLogging {
        showExceptions = true
        showStandardStreams = true
        events = setOf(
            org.gradle.api.tasks.testing.logging.TestLogEvent.FAILED,
            org.gradle.api.tasks.testing.logging.TestLogEvent.PASSED
        )
        exceptionFormat = org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL
    }
}
