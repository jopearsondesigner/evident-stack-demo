plugins {
    id("evident.platform.kotlin-js-lib")
}

group = "$group.modeling"
version = "0.1.0-SNAPSHOT"

dependencies {
    //platform()
    implementation("evident.platform.domain:event-models")
    implementation(libs.kotlinx.coroutines)
    testImplementation(kotlin("test"))
}

kotlin {
    js(IR) {
        binaries.library()
        browser {
            testTask {
                useKarma {
                    useFirefox()
                }
            }
        }
        compilations["main"].packageJson {
            customField("private", true)
        }
    }
}