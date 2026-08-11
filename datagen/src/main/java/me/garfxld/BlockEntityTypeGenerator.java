package me.garfxld;


import com.google.gson.JsonObject;
import net.minecraft.core.registries.BuiltInRegistries;

public class BlockEntityTypeGenerator implements DataGenerator {

    @Override
    public String generate() {
        var registry = BuiltInRegistries.BLOCK_ENTITY_TYPE;

        var blockEntityTypesJson = new JsonObject();

        for (var blockEntityType : registry) {
            final var key = registry.getKey(blockEntityType);

            var blockEntityTypeJson = new JsonObject();
            blockEntityTypesJson.add(key.toString(),  blockEntityTypeJson);
        }

        return blockEntityTypesJson.toString();
    }

}
