plugins {
    id("evident.platform.commons")
    id("com.google.protobuf")
}

dependencies {
    implementation("io.micronaut.grpc:micronaut-grpc-server-runtime:3.4.0")

    implementation("io.grpc:grpc-kotlin-stub:$grpcKotlinVersion")
    compileOnly("io.grpc:grpc-stub:$grpcVersion")
}


sourceSets {
    main {
        java {
            srcDirs 'build/generated/source/proto/main/grpc'
            srcDirs 'build/generated/source/proto/main/grpckt'
            srcDirs 'build/generated/source/proto/main/java'
        }
    }
}

protobuf {
    protoc { artifact = "com.google.protobuf:protoc:${protobufJavaVersion}" }
    plugins {
        grpc { artifact = "io.grpc:protoc-gen-grpc-java:${grpcVersion}" }
        grpckt { artifact = "io.grpc:protoc-gen-grpc-kotlin:${grpcKotlinVersion}:jdk8@jar" }
    }
    generateProtoTasks {
        all()*.plugins {
            grpc {}
            grpckt {}
        }
    }
}