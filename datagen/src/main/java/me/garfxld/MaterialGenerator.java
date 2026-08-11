package me.garfxld;

import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.item.Item;
import net.minecraft.world.level.block.Block;

public class MaterialGenerator implements DataGenerator {

    @Override
    public String generate() {

        var registry = BuiltInRegistries.ITEM;

        var builder = new StringBuilder();
        builder.append("pub struct Material {");
        builder.append("    ");
        builder.append("}");

        Block.byItem()
        for (Item item : registry) {

        }


        // define_objects! {
        //     Material,
        //     ACACIA_BOAT: Material = "minecraft:acacia_boat";
        //
        // }

        // pub const ACACIA_BOAT: Object<Material> =

        return builder.toString();
    }

}
