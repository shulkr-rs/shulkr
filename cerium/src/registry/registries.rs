use crate::item::trim_material::TrimMaterial;

use super::*;

pub struct Registries {
    pub biome: Registry<Biome>,
    pub cat_variant: Registry<CatVariant>,
    pub cat_sound_variant: Registry<CatSoundVariant>,
    pub chicken_variant: Registry<ChickenVariant>,
    pub chicken_sound_variant: Registry<ChickenSoundVariant>,
    pub cow_variant: Registry<CowVariant>,
    pub cow_sound_variant: Registry<CowSoundVariant>,
    pub damage_type: Registry<DamageType>,
    pub dimension_type: Registry<DimensionType>,
    pub frog_variant: Registry<FrogVariant>,
    pub painting_variant: Registry<PaintingVariant>,
    pub pig_variant: Registry<PigVariant>,
    pub pig_sound_variant: Registry<PigSoundVariant>,
    pub wolf_variant: Registry<WolfVariant>,
    pub wolf_sound_variant: Registry<WolfSoundVariant>,
    pub timeline: Registry<Timeline>,
    pub world_clock: Registry<WorldClock>,
    pub zombie_nautilus_variant: Registry<ZombieNautilusVariant>,
    pub trim_material: Registry<TrimMaterial>,
    pub jukebox_song: Registry<JukeBoxSong>,
    pub banner_pattern: Registry<BannerPattern>,
    pub instrument: Registry<Instrument>,
}

pub fn load<T>(id: &'static str, data: &str) -> Registry<T>
where
    T: Serialize + DeserializeOwned,
{
    let mut entries: IndexMap<String, T> = serde_json::from_str(&data).unwrap();

    let key = "plains";
    if let Some(value) = entries.swap_remove(key) {
        let old_entries = std::mem::take(&mut entries);
        entries.insert(key.to_string(), value);
        entries.extend(old_entries);
    }

    let mut registry = Registry::<T>::new(RegistryKey::const_vanilla(id));
    for (key, value) in entries {
        Registry::register(&mut registry, key.into(), value);
    }

    registry
}

macro_rules! registry {
    ($path:path, $ty:ty, $expr:expr) => {{
        static VALUE: LazyLock<Registry<$ty>> = LazyLock::new(|| {
            let mut registry = Registry::new($expr);
            $path(&mut registry);
            registry
        });
        &VALUE
    }};
}

impl Registries {
    pub const BLOCK: &LazyLock<Registry<Block>> = registry!(
        crate::world::block::register_all,
        Block,
        RegistryKeys::BLOCK
    );
    pub const MATERIAL: &LazyLock<Registry<Material>> = registry!(
        crate::item::material::register_all,
        Material,
        RegistryKeys::MATERIAL
    );
    pub const ENTITY_TYPE: &LazyLock<Registry<EntityType>> = registry!(
        crate::entity::entity_type::register_all,
        EntityType,
        RegistryKeys::ENTITY_TYPE
    );

    #[rustfmt::skip]
    pub fn new() -> Self {
        Self {
            damage_type: load("damage_type", include_str!("../../build_assets/damage_type.json")),
            banner_pattern: load("banner_pattern", include_str!("../../build_assets/banner_pattern.json")),
            instrument: load("instrument", include_str!("../../build_assets/instrument.json")),
            jukebox_song: load("jukebox_song", include_str!("../../build_assets/jukebox_song.json")),
            trim_material: load("trim_material", include_str!("../../build_assets/trim_material.json")),

            // World
            biome:                      load("worldgen/biome", include_str!("../../build_assets/worldgen/biome.json")),
            dimension_type:             load("dimension_type", include_str!("../../build_assets/dimension_type.json")),
            timeline:                   load("timeline", include_str!("../../build_assets/timeline.json")),
            world_clock:                load("world_clock", include_str!("../../build_assets/world_clock.json")),

            // Entities
            cat_variant:                load("cat_variant", include_str!("../../build_assets/cat_variant.json")),
            cat_sound_variant:          load("cat_sound_variant", include_str!("../../build_assets/cat_sound_variant.json")),
            chicken_variant:            load("chicken_variant", include_str!("../../build_assets/chicken_variant.json")),
            chicken_sound_variant:      load("chicken_sound_variant", include_str!("../../build_assets/chicken_sound_variant.json")),
            cow_variant:                load("cow_variant", include_str!("../../build_assets/cow_variant.json")),
            cow_sound_variant:          load("cow_sound_variant", include_str!("../../build_assets/cow_sound_variant.json")),
            frog_variant:               load("frog_variant", include_str!("../../build_assets/frog_variant.json")),
            painting_variant:           load("painting_variant", include_str!("../../build_assets/painting_variant.json")),
            pig_variant:                load("pig_variant", include_str!("../../build_assets/pig_variant.json")),
            pig_sound_variant:          load("pig_sound_variant", include_str!("../../build_assets/pig_sound_variant.json")),
            wolf_variant:               load("wolf_variant", include_str!("../../build_assets/wolf_variant.json")),
            wolf_sound_variant:         load("wolf_sound_variant", include_str!("../../build_assets/wolf_sound_variant.json")),
            zombie_nautilus_variant:    load("zombie_nautilus_variant", include_str!("../../build_assets/zombie_nautilus_variant.json")),
        }
    }
}
