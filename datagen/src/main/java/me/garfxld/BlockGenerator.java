package me.garfxld;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import net.minecraft.core.Holder;
import net.minecraft.core.Registry;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.item.Item;
import net.minecraft.world.level.block.AirBlock;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockBehaviour;
import net.minecraft.world.level.block.state.properties.BlockStateProperties;

import java.lang.reflect.Field;

public class BlockGenerator implements DataGenerator {

    @Override
    public String generate() {
        var registry = BuiltInRegistries.BLOCK;

        var blocksJson = new JsonObject();
        var currStateId = 0;

        for (var block : registry) {
            final var key = registry.getKey(block);
            final var defaultBlockState = block.defaultBlockState();

            var blockJson = new JsonObject();
            blockJson.addProperty("id", registry.getId(block));
            blockJson.addProperty("defaultStateId", Block.BLOCK_STATE_REGISTRY.getId(defaultBlockState));
            blockJson.addProperty("minStateId", currStateId);

            if (currStateId != Block.BLOCK_STATE_REGISTRY.getId(defaultBlockState)) {
                IO.println("Different " + block.getDescriptionId());
            }

            currStateId += block.getStateDefinition().getPossibleStates().size();



            var propsJson = new JsonArray();
            for (var property : block.getStateDefinition().getProperties()) {
                propsJson.add(getConstantName(BlockStateProperties.class, property));
            }
            if (!propsJson.isEmpty()) {
                blockJson.add("properties", propsJson);
            }

            blocksJson.add(key.toString(),  blockJson);
        }

        return blocksJson.toString();
    }

    public static String getConstantName(Class<?> clazz, Object value) {
        for (Field field : clazz.getFields()) {
            try {
                if (field.get(null) == value) {
                    return field.getName();
                }
            } catch (IllegalAccessException ignored) {
            }
        }
        return null;
    }

}
