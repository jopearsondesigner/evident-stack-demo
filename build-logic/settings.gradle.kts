dependencyResolutionManagement {
    repositories {
        mavenCentral()
    }
}
includeBuild("../platforms")

rootProject.name = "build-logic"
include("commons")
include("kotlin-library")
include("kotlin-multiplatform-library")
include("kotlin-js-library")
include("micronaut-application")