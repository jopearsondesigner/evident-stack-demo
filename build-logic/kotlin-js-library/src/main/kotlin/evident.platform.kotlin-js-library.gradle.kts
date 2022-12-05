plugins {
    kotlin("js")
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
        // nodejs() // TODO: enable NodeJS build for Event Model SDK
    }
}