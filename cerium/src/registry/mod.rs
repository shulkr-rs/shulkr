use crate::{
    entity::meta::{
        CatSoundVariant, CatVariant, ChickenSoundVariant, ChickenVariant, CowSoundVariant,
        CowVariant, FrogVariant, PaintingVariant, PigSoundVariant, PigVariant, WolfSoundVariant,
        WolfVariant, ZombieNautilusVariant,
    },
    util::Identifier,
    world::{
        DimensionType, biome::Biome, block::BlockRegistry, clock::WorldClock, timeline::Timeline,
    },
};
use indexmap::IndexMap;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    borrow::Cow,
    cell::UnsafeCell,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::MaybeUninit,
    ops::Deref,
};

mod banner_pattern;
mod damage_type;
mod generated;
mod instrument;
mod jukebox_song;
mod tag;
mod trim_material;

pub use banner_pattern::*;
pub use damage_type::*;
pub use instrument::*;
pub use jukebox_song::*;
pub use tag::*;
pub use trim_material::*;

#[derive(Debug, Clone)]
pub struct DynamicRegistry<T>
where
    T: Serialize + DeserializeOwned,
{
    registry_id: Identifier,
    entries: IndexMap<RegistryKey<T>, T>,
}

impl<T> DynamicRegistry<T>
where
    T: Serialize + DeserializeOwned,
    RegistryKey<T>: Eq + Hash,
{
    pub fn create(name: String, data: String) -> Self {
        let mut entries: IndexMap<String, T> = serde_json::from_str(&data).unwrap();

        let mut this = Self {
            registry_id: Identifier::of(name),
            entries: IndexMap::new(),
        };

        let key = "plains";
        if let Some(value) = entries.swap_remove(key) {
            let old_entries = std::mem::take(&mut entries);
            entries.insert(key.to_string(), value);
            entries.extend(old_entries);
        }

        for (key, value) in entries {
            this.register(RegistryKey::new(key), value);
        }

        this
    }

    pub fn registry_id(&self) -> &Identifier {
        &self.registry_id
    }

    pub fn register(&mut self, key: RegistryKey<T>, object: T) {
        self.entries.insert(key, object);
    }

    pub fn get(&self, key: &RegistryKey<T>) -> Option<&T> {
        self.entries.get(key)
    }

    pub fn get_id(&self, key: &RegistryKey<T>) -> Option<usize> {
        self.entries.keys().position(|k| k == key)
    }

    pub fn entries(&self) -> &IndexMap<RegistryKey<T>, T> {
        &self.entries
    }
}

pub struct Registries {
    pub biome: DynamicRegistry<Biome>,
    pub cat_variant: DynamicRegistry<CatVariant>,
    pub cat_sound_variant: DynamicRegistry<CatSoundVariant>,
    pub chicken_variant: DynamicRegistry<ChickenVariant>,
    pub chicken_sound_variant: DynamicRegistry<ChickenSoundVariant>,
    pub cow_variant: DynamicRegistry<CowVariant>,
    pub cow_sound_variant: DynamicRegistry<CowSoundVariant>,
    pub damage_type: DynamicRegistry<DamageType>,
    pub dimension_type: DynamicRegistry<DimensionType>,
    pub frog_variant: DynamicRegistry<FrogVariant>,
    pub painting_variant: DynamicRegistry<PaintingVariant>,
    pub pig_variant: DynamicRegistry<PigVariant>,
    pub pig_sound_variant: DynamicRegistry<PigSoundVariant>,
    pub wolf_variant: DynamicRegistry<WolfVariant>,
    pub wolf_sound_variant: DynamicRegistry<WolfSoundVariant>,
    pub timeline: DynamicRegistry<Timeline>,
    pub world_clock: DynamicRegistry<WorldClock>,
    pub zombie_nautilus_variant: DynamicRegistry<ZombieNautilusVariant>,
    pub trim_material: DynamicRegistry<TrimMaterial>,
    pub jukebox_song: DynamicRegistry<JukeBoxSong>,
    pub banner_pattern: DynamicRegistry<BannerPattern>,
    pub instrument: DynamicRegistry<Instrument>,

    // Static
    pub block: &'static BlockRegistry,
}

pub fn load<T>(id: Identifier, data: &str) -> DynamicRegistry<T>
where
    T: Serialize + DeserializeOwned,
{
    DynamicRegistry::create(id.to_string(), data.to_owned())
}

impl Registries {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self {
            damage_type: load("minecraft:damage_type".into(), include_str!("../../data/damage_type.json")),
            banner_pattern: load("minecraft:banner_pattern".into(), include_str!("../../data/banner_pattern.json")),
            instrument: load("minecraft:instrument".into(), include_str!("../../data/instrument.json")),
            jukebox_song: load("minecraft:jukebox_song".into(), include_str!("../../data/jukebox_song.json")),
            trim_material: load("minecraft:trim_material".into(), include_str!("../../data/trim_material.json")),

            // World
            biome:                      load("minecraft:worldgen/biome".into(), include_str!("../../data/worldgen/biome.json")),
            dimension_type:             load("minecraft:dimension_type".into(), include_str!("../../data/dimension_type.json")),
            timeline:                   load("minecraft:timeline".into(), include_str!("../../data/timeline.json")),
            world_clock:                load("minecraft:world_clock".into(), include_str!("../../data/world_clock.json")),

            // Entities
            cat_variant:                load("minecraft:cat_variant".into(), include_str!("../../data/cat_variant.json")),
            cat_sound_variant:          load("minecraft:cat_sound_variant".into(), include_str!("../../data/cat_sound_variant.json")),
            chicken_variant:            load("minecraft:chicken_variant".into(), include_str!("../../data/chicken_variant.json")),
            chicken_sound_variant:      load("minecraft:chicken_sound_variant".into(), include_str!("../../data/chicken_sound_variant.json")),
            cow_variant:                load("minecraft:cow_variant".into(), include_str!("../../data/cow_variant.json")),
            cow_sound_variant:          load("minecraft:cow_sound_variant".into(), include_str!("../../data/cow_sound_variant.json")),
            frog_variant:               load("minecraft:frog_variant".into(), include_str!("../../data/frog_variant.json")),
            painting_variant:           load("minecraft:painting_variant".into(), include_str!("../../data/painting_variant.json")),
            pig_variant:                load("minecraft:pig_variant".into(), include_str!("../../data/pig_variant.json")),
            pig_sound_variant:          load("minecraft:pig_sound_variant".into(), include_str!("../../data/pig_sound_variant.json")),
            wolf_variant:               load("minecraft:wolf_variant".into(), include_str!("../../data/wolf_variant.json")),
            wolf_sound_variant:         load("minecraft:wolf_sound_variant".into(), include_str!("../../data/wolf_sound_variant.json")),
            zombie_nautilus_variant:    load("minecraft:zombie_nautilus_variant".into(), include_str!("../../data/zombie_nautilus_variant.json")),

            block: BlockRegistry::load()
        }
    }
}

/// Represents a static registry object. This works similar to rust's [OnceLock].
pub struct RegistryHolder<T> {
    key: &'static str,
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> RegistryHolder<T> {
    pub const fn new(key: &'static str) -> Self {
        Self {
            key,
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn load(&self, registry: DynamicRegistry<T>)
    where
        T: Serialize + DeserializeOwned + Clone,
    {
        let value = registry.get(&RegistryKey::of(self.key())).unwrap().clone();
        self.set(value);
    }

    pub const fn set(&self, value: T) {
        unsafe { (&mut *self.value.get()).write(value) };
    }

    pub const fn key(&self) -> &'static str {
        self.key
    }
}

unsafe impl<T: Sync + Send> Sync for RegistryHolder<T> {}
unsafe impl<T: Send> Send for RegistryHolder<T> {}

impl<T> const Deref for RegistryHolder<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { (&*self.value.get()).assume_init_ref() }
    }
}

#[derive(Debug, Clone)]
pub struct RegistryKey<T> {
    key: Cow<'static, str>,
    _phantom: PhantomData<T>,
}

impl<T> RegistryKey<T> {
    pub const fn of(key: &'static str) -> Self {
        Self {
            key: Cow::Borrowed(key),
            _phantom: PhantomData,
        }
    }

    pub const fn new(key: String) -> Self {
        Self {
            key: Cow::Owned(key),
            _phantom: PhantomData,
        }
    }

    pub fn to_key(&self) -> Identifier {
        Identifier::of(self.key.clone())
    }
}

impl<T> PartialEq for RegistryKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for RegistryKey<T> {}

impl<T> Hash for RegistryKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T> From<RegistryKey<T>> for Identifier {
    fn from(value: RegistryKey<T>) -> Self {
        value.to_key()
    }
}
