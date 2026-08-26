// This file was auto-generated. Do not edit it manually.

use crate::registry::Registry;
use cerium_macros::StaticObject;
use cerium_macros::UnitEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StaticObject, UnitEnum)]
#[repr(u16)]
pub enum BlockEntityType {
    Furnace,
    Chest,
    TrappedChest,
    EnderChest,
    Jukebox,
    Dispenser,
    Dropper,
    Sign,
    HangingSign,
    MobSpawner,
    CreakingHeart,
    Piston,
    BrewingStand,
    EnchantingTable,
    EndPortal,
    Beacon,
    Skull,
    DaylightDetector,
    Hopper,
    Comparator,
    Banner,
    StructureBlock,
    EndGateway,
    CommandBlock,
    ShulkerBox,
    Conduit,
    Barrel,
    Smoker,
    BlastFurnace,
    Lectern,
    Bell,
    Jigsaw,
    Campfire,
    Beehive,
    SculkSensor,
    CalibratedSculkSensor,
    SculkCatalyst,
    SculkShrieker,
    ChiseledBookshelf,
    Shelf,
    BrushableBlock,
    DecoratedPot,
    Crafter,
    TrialSpawner,
    Vault,
    TestBlock,
    TestInstanceBlock,
    CopperGolemStatue,
    PotentSulfur,
}
pub static FURNACE: BlockEntityTypeData = BlockEntityTypeData;
pub static CHEST: BlockEntityTypeData = BlockEntityTypeData;
pub static TRAPPED_CHEST: BlockEntityTypeData = BlockEntityTypeData;
pub static ENDER_CHEST: BlockEntityTypeData = BlockEntityTypeData;
pub static JUKEBOX: BlockEntityTypeData = BlockEntityTypeData;
pub static DISPENSER: BlockEntityTypeData = BlockEntityTypeData;
pub static DROPPER: BlockEntityTypeData = BlockEntityTypeData;
pub static SIGN: BlockEntityTypeData = BlockEntityTypeData;
pub static HANGING_SIGN: BlockEntityTypeData = BlockEntityTypeData;
pub static MOB_SPAWNER: BlockEntityTypeData = BlockEntityTypeData;
pub static CREAKING_HEART: BlockEntityTypeData = BlockEntityTypeData;
pub static PISTON: BlockEntityTypeData = BlockEntityTypeData;
pub static BREWING_STAND: BlockEntityTypeData = BlockEntityTypeData;
pub static ENCHANTING_TABLE: BlockEntityTypeData = BlockEntityTypeData;
pub static END_PORTAL: BlockEntityTypeData = BlockEntityTypeData;
pub static BEACON: BlockEntityTypeData = BlockEntityTypeData;
pub static SKULL: BlockEntityTypeData = BlockEntityTypeData;
pub static DAYLIGHT_DETECTOR: BlockEntityTypeData = BlockEntityTypeData;
pub static HOPPER: BlockEntityTypeData = BlockEntityTypeData;
pub static COMPARATOR: BlockEntityTypeData = BlockEntityTypeData;
pub static BANNER: BlockEntityTypeData = BlockEntityTypeData;
pub static STRUCTURE_BLOCK: BlockEntityTypeData = BlockEntityTypeData;
pub static END_GATEWAY: BlockEntityTypeData = BlockEntityTypeData;
pub static COMMAND_BLOCK: BlockEntityTypeData = BlockEntityTypeData;
pub static SHULKER_BOX: BlockEntityTypeData = BlockEntityTypeData;
pub static CONDUIT: BlockEntityTypeData = BlockEntityTypeData;
pub static BARREL: BlockEntityTypeData = BlockEntityTypeData;
pub static SMOKER: BlockEntityTypeData = BlockEntityTypeData;
pub static BLAST_FURNACE: BlockEntityTypeData = BlockEntityTypeData;
pub static LECTERN: BlockEntityTypeData = BlockEntityTypeData;
pub static BELL: BlockEntityTypeData = BlockEntityTypeData;
pub static JIGSAW: BlockEntityTypeData = BlockEntityTypeData;
pub static CAMPFIRE: BlockEntityTypeData = BlockEntityTypeData;
pub static BEEHIVE: BlockEntityTypeData = BlockEntityTypeData;
pub static SCULK_SENSOR: BlockEntityTypeData = BlockEntityTypeData;
pub static CALIBRATED_SCULK_SENSOR: BlockEntityTypeData = BlockEntityTypeData;
pub static SCULK_CATALYST: BlockEntityTypeData = BlockEntityTypeData;
pub static SCULK_SHRIEKER: BlockEntityTypeData = BlockEntityTypeData;
pub static CHISELED_BOOKSHELF: BlockEntityTypeData = BlockEntityTypeData;
pub static SHELF: BlockEntityTypeData = BlockEntityTypeData;
pub static BRUSHABLE_BLOCK: BlockEntityTypeData = BlockEntityTypeData;
pub static DECORATED_POT: BlockEntityTypeData = BlockEntityTypeData;
pub static CRAFTER: BlockEntityTypeData = BlockEntityTypeData;
pub static TRIAL_SPAWNER: BlockEntityTypeData = BlockEntityTypeData;
pub static VAULT: BlockEntityTypeData = BlockEntityTypeData;
pub static TEST_BLOCK: BlockEntityTypeData = BlockEntityTypeData;
pub static TEST_INSTANCE_BLOCK: BlockEntityTypeData = BlockEntityTypeData;
pub static COPPER_GOLEM_STATUE: BlockEntityTypeData = BlockEntityTypeData;
pub static POTENT_SULFUR: BlockEntityTypeData = BlockEntityTypeData;
pub(crate) fn register_all(registry: &mut Registry<BlockEntityType>) {
    let mut register = |key: &'static str, value: BlockEntityType| {
        Registry::register(registry, key.into(), value);
    };
    register("minecraft:furnace", BlockEntityType::Furnace);
    register("minecraft:chest", BlockEntityType::Chest);
    register("minecraft:trapped_chest", BlockEntityType::TrappedChest);
    register("minecraft:ender_chest", BlockEntityType::EnderChest);
    register("minecraft:jukebox", BlockEntityType::Jukebox);
    register("minecraft:dispenser", BlockEntityType::Dispenser);
    register("minecraft:dropper", BlockEntityType::Dropper);
    register("minecraft:sign", BlockEntityType::Sign);
    register("minecraft:hanging_sign", BlockEntityType::HangingSign);
    register("minecraft:mob_spawner", BlockEntityType::MobSpawner);
    register("minecraft:creaking_heart", BlockEntityType::CreakingHeart);
    register("minecraft:piston", BlockEntityType::Piston);
    register("minecraft:brewing_stand", BlockEntityType::BrewingStand);
    register(
        "minecraft:enchanting_table",
        BlockEntityType::EnchantingTable,
    );
    register("minecraft:end_portal", BlockEntityType::EndPortal);
    register("minecraft:beacon", BlockEntityType::Beacon);
    register("minecraft:skull", BlockEntityType::Skull);
    register(
        "minecraft:daylight_detector",
        BlockEntityType::DaylightDetector,
    );
    register("minecraft:hopper", BlockEntityType::Hopper);
    register("minecraft:comparator", BlockEntityType::Comparator);
    register("minecraft:banner", BlockEntityType::Banner);
    register("minecraft:structure_block", BlockEntityType::StructureBlock);
    register("minecraft:end_gateway", BlockEntityType::EndGateway);
    register("minecraft:command_block", BlockEntityType::CommandBlock);
    register("minecraft:shulker_box", BlockEntityType::ShulkerBox);
    register("minecraft:conduit", BlockEntityType::Conduit);
    register("minecraft:barrel", BlockEntityType::Barrel);
    register("minecraft:smoker", BlockEntityType::Smoker);
    register("minecraft:blast_furnace", BlockEntityType::BlastFurnace);
    register("minecraft:lectern", BlockEntityType::Lectern);
    register("minecraft:bell", BlockEntityType::Bell);
    register("minecraft:jigsaw", BlockEntityType::Jigsaw);
    register("minecraft:campfire", BlockEntityType::Campfire);
    register("minecraft:beehive", BlockEntityType::Beehive);
    register("minecraft:sculk_sensor", BlockEntityType::SculkSensor);
    register(
        "minecraft:calibrated_sculk_sensor",
        BlockEntityType::CalibratedSculkSensor,
    );
    register("minecraft:sculk_catalyst", BlockEntityType::SculkCatalyst);
    register("minecraft:sculk_shrieker", BlockEntityType::SculkShrieker);
    register(
        "minecraft:chiseled_bookshelf",
        BlockEntityType::ChiseledBookshelf,
    );
    register("minecraft:shelf", BlockEntityType::Shelf);
    register("minecraft:brushable_block", BlockEntityType::BrushableBlock);
    register("minecraft:decorated_pot", BlockEntityType::DecoratedPot);
    register("minecraft:crafter", BlockEntityType::Crafter);
    register("minecraft:trial_spawner", BlockEntityType::TrialSpawner);
    register("minecraft:vault", BlockEntityType::Vault);
    register("minecraft:test_block", BlockEntityType::TestBlock);
    register(
        "minecraft:test_instance_block",
        BlockEntityType::TestInstanceBlock,
    );
    register(
        "minecraft:copper_golem_statue",
        BlockEntityType::CopperGolemStatue,
    );
    register("minecraft:potent_sulfur", BlockEntityType::PotentSulfur);
}
