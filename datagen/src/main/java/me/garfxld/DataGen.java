package me.garfxld;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonElement;
import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;

import java.io.BufferedWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;

public class DataGen {

    public static final Gson GSON = new GsonBuilder().create();

    static void main() throws IOException {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();

        write(Path.of("generated/block.json"), new BlockGenerator().generate());
        write(Path.of("generated/block_entity_type.json"), new BlockEntityTypeGenerator().generate());
    }

    static void write(Path path, Object data) throws IOException {
        if (!Files.exists(path)) {
            Files.createDirectories(path.getParent());
        }

        try (BufferedWriter writer = Files.newBufferedWriter(path, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING)) {
            try {
                if (data instanceof JsonElement) {
                    GSON.toJson(data, writer);
                } else {
                    writer.write(data.toString());
                }
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        }
    }

}
