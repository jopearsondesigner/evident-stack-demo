plugins {
    id("java")
}

group = "evident.platform"

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

dependencies {
    implementation(platform("evident.platform:product-platform"))

    testImplementation(platform("evident.platform:test-platform"))
    testImplementation("org.junit.jupiter:junit-jupiter")
}

tasks.test {
    useJUnitPlatform()
}

