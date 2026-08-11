plugins {
    id("java")
    alias(libs.plugins.loom)
}

group = "me.garfxld"
version = "1.0-SNAPSHOT"

loom {
    serverOnlyMinecraftJar()
}

repositories {
    mavenCentral()
}

dependencies {
    minecraft("com.mojang:minecraft:${libs.versions.minecraft.get()}")
}

