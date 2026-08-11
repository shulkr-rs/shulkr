package me.garfxld;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.entity.EntityDimensions;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;

public class EntityTypeGenerator implements DataGenerator {

    @Override
    public String generate() {
        var registry = BuiltInRegistries.ENTITY_TYPE;

        var entityTypesJson = new JsonObject();
        for (var entityType : registry) {
            final var key = registry.getKey(entityType);

            var entityTypeJson = new JsonObject();

            EntityDimensions dimensions = entityType.getDimensions();
            entityTypeJson.addProperty("width", dimensions.width());
            entityTypeJson.addProperty("height", dimensions.height());
            entityTypeJson.addProperty("eyeHeight", dimensions.eyeHeight());

            entityTypesJson.add(key.toString(),  entityTypeJson);
        }

        return "";
    }

}
