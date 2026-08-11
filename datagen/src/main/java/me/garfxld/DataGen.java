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

//TIP To <b>Run</b> code, press <shortcut actionId="Run"/> or
// click the <icon src="AllIcons.Actions.Execute"/> icon in the gutter.
public class DataGen {

    public static final Gson GSON = new GsonBuilder().create();

    static void main() throws IOException {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();

        Object result = new BlockGenerator().generate();

        var path = Path.of("generated/blocks.json");
        if (!Files.exists(path)) {
            Files.createDirectories(path.getParent());
        }

        try (BufferedWriter writer = Files.newBufferedWriter(path, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING)) {
            try {
                if (result instanceof JsonElement) {
                    GSON.toJson(result, writer);
                } else {
                    writer.write(result.toString());
                }
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        }
    }
}
