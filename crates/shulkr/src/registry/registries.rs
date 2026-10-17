use super::*;
use std::sync::Once;

pub fn register_all() {
    static REGISTERED: Once = Once::new();
    REGISTERED.call_once(|| {
        crate::world::block::register_all(&BLOCK);
        crate::world::block::block_entity::register_all(&BLOCK_ENTITY_TYPE);
        crate::item::material::register_all(&MATERIAL);
        crate::entity::entity_type::register_all(&ENTITY_TYPE);
        crate::world::attribute::register_all(&ENVIRONMENT_ATTRIBUTE);
        crate::world::fluid::register_all(&FLUID);
        crate::sound::register_all(&SOUND_EVENT);

        load_json(&BIOME);
        load_json(&DAMAGE_TYPE);
        load_json(&BANNER_PATTERN);
        load_json(&INSTRUMENT);
        load_json(&JUKEBOX_SONG);
        load_json(&TRIM_MATERIAL);
        load_json(&DIMENSION_TYPE);
        load_json(&TIMELINE);
        load_json(&WORLD_CLOCK);
        load_json(&CAT_VARIANT);
        load_json(&CAT_SOUND_VARIANT);
        load_json(&CHICKEN_VARIANT);
        load_json(&CHICKEN_SOUND_VARIANT);
        load_json(&COW_VARIANT);
        load_json(&COW_SOUND_VARIANT);
        load_json(&FROG_VARIANT);
        load_json(&PAINTING_VARIANT);
        load_json(&PIG_VARIANT);
        load_json(&PIG_SOUND_VARIANT);
        load_json(&WOLF_VARIANT);
        load_json(&WOLF_SOUND_VARIANT);
        load_json(&ZOMBIE_NAUTILUS_VARIANT);
        load_json(&BLOCK_TRANSFORMER);
        load_json(&CONTEXT_FLOAT_PROVIDER);
        load_json(&CONTEXT_INT_PROVIDER);
        load_json(&DECORATED_POT_PATTERN);
        load_json(&PREDICATE);
    });
}

pub static BLOCK: Registry<Block> = Registry::new("minecraft:block");
pub static BLOCK_ENTITY_TYPE: Registry<BlockEntityType> =
    Registry::new("minecraft:block_entity_type");
pub static MATERIAL: Registry<Material> = Registry::new("minecraft:material");
pub static ENTITY_TYPE: Registry<EntityType> = Registry::new("minecraft:entity_type");
pub static ENVIRONMENT_ATTRIBUTE: Registry<EnvironmentAttribute> =
    Registry::new("minecraft:environment_attribute");
pub static FLUID: Registry<Fluid> = Registry::new("minecraft:fluid");
pub static SOUND_EVENT: Registry<SoundEvent> = Registry::new("minecraft:sound_event");

pub static BIOME: Registry<Biome> = Registry::new("minecraft:worldgen/biome");
pub static DAMAGE_TYPE: Registry<DamageType> = Registry::new("minecraft:damage_type");
pub static BANNER_PATTERN: Registry<BannerPattern> = Registry::new("minecraft:banner_pattern");
pub static INSTRUMENT: Registry<Instrument> = Registry::new("minecraft:instrument");
pub static JUKEBOX_SONG: Registry<JukeboxSong> = Registry::new("minecraft:jukebox_song");
pub static TRIM_MATERIAL: Registry<TrimMaterial> = Registry::new("minecraft:trim_material");
pub static DIMENSION_TYPE: Registry<DimensionType> = Registry::new("minecraft:dimension_type");
pub static TIMELINE: Registry<Timeline> = Registry::new("minecraft:timeline");
pub static WORLD_CLOCK: Registry<WorldClock> = Registry::new("minecraft:world_clock");
pub static CAT_VARIANT: Registry<CatVariant> = Registry::new("minecraft:cat_variant");
pub static CAT_SOUND_VARIANT: Registry<CatSoundVariant> =
    Registry::new("minecraft:cat_sound_variant");
pub static CHICKEN_VARIANT: Registry<ChickenVariant> = Registry::new("minecraft:chicken_variant");
pub static CHICKEN_SOUND_VARIANT: Registry<ChickenSoundVariant> =
    Registry::new("minecraft:chicken_sound_variant");
pub static COW_VARIANT: Registry<CowVariant> = Registry::new("minecraft:cow_variant");
pub static COW_SOUND_VARIANT: Registry<CowSoundVariant> =
    Registry::new("minecraft:cow_sound_variant");
pub static FROG_VARIANT: Registry<FrogVariant> = Registry::new("minecraft:frog_variant");
pub static PAINTING_VARIANT: Registry<PaintingVariant> =
    Registry::new("minecraft:painting_variant");
pub static PIG_VARIANT: Registry<PigVariant> = Registry::new("minecraft:pig_variant");
pub static PIG_SOUND_VARIANT: Registry<PigSoundVariant> =
    Registry::new("minecraft:pig_sound_variant");
pub static WOLF_VARIANT: Registry<WolfVariant> = Registry::new("minecraft:wolf_variant");
pub static WOLF_SOUND_VARIANT: Registry<WolfSoundVariant> =
    Registry::new("minecraft:wolf_sound_variant");
pub static ZOMBIE_NAUTILUS_VARIANT: Registry<ZombieNautilusVariant> =
    Registry::new("minecraft:zombie_nautilus_variant");
pub static BLOCK_TRANSFORMER: Registry<Vec<BlockTransformer>> =
    Registry::new("minecraft:block_transformer");
pub static CONTEXT_FLOAT_PROVIDER: Registry<ContextFloatProvider> =
    Registry::new("minecraft:context_float_provider");
pub static CONTEXT_INT_PROVIDER: Registry<ContextIntProvider> =
    Registry::new("minecraft:context_int_provider");
pub static DECORATED_POT_PATTERN: Registry<DecoratedPotPattern> =
    Registry::new("minecraft:decorated_pot_pattern");
pub static PREDICATE: Registry<Predicate> = Registry::new("minecraft:predicate");
