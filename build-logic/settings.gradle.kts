// == Define locations for build logic ==
pluginManagement {
    repositories {
        gradlePluginPortal()
    }
}

// == Define locations for components ==
dependencyResolutionManagement {
    repositories {
        mavenCentral()
        gradlePluginPortal()
    }
}
includeBuild("../platforms")

rootProject.name = "build-logic"
include("commons")
include("kotlin-library")
include("kotlin-multiplatform-library")
include("kotlin-js-library")
include("micronaut-application")