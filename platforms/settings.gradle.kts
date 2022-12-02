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
    }
}

// == Define the inner structure of this component ==
rootProject.name = "platforms"

include("plugins-platform")
include("product-platform")
include("test-platform")
