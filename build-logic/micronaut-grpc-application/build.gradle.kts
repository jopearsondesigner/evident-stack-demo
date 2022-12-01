plugins {
    `kotlin-dsl`
}

dependencies {
    implementation(platform("evident.platform:plugins-platform"))
    implementation(project(":commons"))
}
