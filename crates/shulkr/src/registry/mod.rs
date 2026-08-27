use crate::{
    entity::{
        EntityType,
        meta::{
            CatSoundVariant, CatVariant, ChickenSoundVariant, ChickenVariant, CowSoundVariant,
            CowVariant, FrogVariant, PaintingVariant, PigSoundVariant, PigVariant,
            WolfSoundVariant, WolfVariant, ZombieNautilusVariant,
        },
    },
    item::{Material, trim_material::TrimMaterial},
    util::{HashMap, Key},
    world::{
        DimensionType,
        attribute::EnvironmentAttribute,
        biome::Biome,
        block::{Block, BlockEntityType},
        clock::WorldClock,
        timeline::Timeline,
    },
};
use indexmap::IndexMap;
use serde::{Serialize, de::DeserializeOwned};
use std::sync::LazyLock;

mod banner_pattern;
mod damage_type;
pub(crate) mod generated;
mod instrument;
mod jukebox_song;
mod registries;
mod registry_key;
mod tag;

pub use banner_pattern::*;
pub use damage_type::*;
pub use instrument::*;
pub use jukebox_song::*;
pub use registries::*;
pub use registry_key::*;
pub use tag::*;

// Use this instead of u16. This makes it possible to swap the underlying type in the future if it is not big enough.
pub type Id = u16;

pub struct RegistryKeys;

type RegisteredKey<T> = RegistryKey<Registry<T>>;

#[rustfmt::skip]
impl RegistryKeys {
    pub const ROOT: Key = Key::const_vanilla("root");
    pub const BLOCK: RegisteredKey<Block>                                = RegistryKey::const_vanilla("block");
    pub const BLOCK_ENTITY_TYPE: RegisteredKey<BlockEntityType>          = RegistryKey::const_vanilla("block_entity_type");
    pub const MATERIAL: RegisteredKey<Material>                          = RegistryKey::const_vanilla("material");
    pub const ENTITY_TYPE: RegisteredKey<EntityType>                     = RegistryKey::const_vanilla("entity_type");
    pub const ENVIRONMENT_ATTRIBUTE: RegisteredKey<EnvironmentAttribute> = RegistryKey::const_vanilla("environment_attribute");
    pub const BIOME: RegisteredKey<Biome>                                = RegistryKey::const_vanilla("worldgen/biome");
}

pub struct Registry<T> {
    key: RegistryKey<Registry<T>>,

    values: Vec<T>,
    keys: Vec<Key>,
    by_key: HashMap<Key, usize>,
}

impl<T> Registry<T> {
    pub fn new(key: RegistryKey<Registry<T>>) -> Self {
        Self {
            key,
            values: Vec::new(),
            keys: Vec::new(),
            by_key: HashMap::default(),
        }
    }

    pub fn key(&self) -> &RegistryKey<Registry<T>> {
        &self.key
    }

    pub fn register(this: &mut Self, key: Key, value: T) -> Id {
        if let Some(&id) = this.by_key.get(&key) {
            this.values[id] = value;
            return id as Id;
        }

        let id = this.values.len();
        this.values.push(value);
        this.keys.push(key.clone());
        this.by_key.insert(key, id);
        id as Id
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn by_id(&self, id: Id) -> Option<&T> {
        self.values.get(id as usize)
    }

    pub fn by_key(&self, key: &Key) -> Option<&T> {
        let id = self.by_key.get(key)?;
        self.by_id(*id as Id)
    }

    pub fn key_of(&self, id: Id) -> Option<&Key> {
        self.keys.get(id as usize)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &T)> {
        self.keys.iter().zip(self.values.iter())
    }

    pub fn get_id<'a>(&self, value: impl Into<Holder<'a, T>>) -> Option<Id>
    where
        T: 'a,
    {
        match value.into() {
            Holder::Direct(t) => self
                .values
                .iter()
                .position(|v| std::ptr::eq(v, t))
                .map(|v| v as Id),
            Holder::Ref(key) => self.by_key.get(&key.to_key()).copied().map(|id| id as Id),
        }
    }

    pub fn get_key<'a>(&self, value: impl Into<Holder<'a, T>>) -> Option<&Key>
    where
        T: 'a,
    {
        match value.into() {
            Holder::Direct(t) => {
                let id = self.values.iter().position(|v| std::ptr::eq(v, t))?;
                self.keys.get(id)
            }
            Holder::Ref(key) => {
                let id = *self.by_key.get(&key.to_key())?;
                self.keys.get(id)
            }
        }
    }

    pub fn values(&self) -> &Vec<T> {
        &self.values
    }
}

pub enum Holder<'a, T> {
    Ref(RegistryKey<T>),
    Direct(&'a T),
}

impl<'a, T: 'a> From<&'a T> for Holder<'a, T> {
    fn from(value: &'a T) -> Self {
        Holder::Direct(value)
    }
}

impl<'a, T: 'a> From<RegistryKey<T>> for Holder<'a, T> {
    fn from(key: RegistryKey<T>) -> Self {
        Holder::Ref(key)
    }
}

impl<'a, T: 'a> From<&RegistryKey<T>> for Holder<'a, T> {
    fn from(key: &RegistryKey<T>) -> Self {
        Holder::Ref(key.clone())
    }
}
