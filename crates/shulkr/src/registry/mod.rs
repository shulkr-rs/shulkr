pub(crate) mod generated;

mod banner_pattern;
mod block_sound_set;
mod block_transformer;
mod context_float_provider;
mod context_int_provider;
mod damage_type;
mod decorated_pot_pattern;
mod instrument;
mod jukebox_song;
mod predicate;
mod reference;
pub mod registries;
mod registry_key;
mod tag;

pub use banner_pattern::*;
pub use block_sound_set::*;
pub use block_transformer::*;
pub use context_float_provider::*;
pub use context_int_provider::*;
pub use damage_type::*;
pub use decorated_pot_pattern::*;
pub use instrument::*;
pub use jukebox_song::*;
pub use predicate::*;
pub use reference::*;
pub use registry_key::*;
pub use tag::*;

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
    sound::SoundEvent,
    util::{HashMap, Key, Mutex},
    world::{
        DimensionType,
        attribute::EnvironmentAttribute,
        biome::Biome,
        block::{Block, BlockEntityType},
        clock::WorldClock,
        fluid::Fluid,
        timeline::Timeline,
    },
};
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use serde::de::DeserializeOwned;
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicBool, Ordering},
};

pub type Id = u16;

static FROZEN: AtomicBool = AtomicBool::new(false);

pub fn freeze() {
    FROZEN.store(true, Ordering::SeqCst);
}

pub fn is_frozen() -> bool {
    FROZEN.load(Ordering::SeqCst)
}

#[derive(thiserror::Error, Debug)]
#[error(
    "registry {registry} is frozen; entries can only be registered before shulkr::registry::freeze()"
)]
pub struct RegistryFrozen {
    pub registry: Key,
}

struct Entries<T: 'static> {
    refs: Vec<&'static Ref<T>>,
    by_key: HashMap<Key, Id>,
    tags: Option<Arc<Tags>>,
}

impl<T: 'static> Entries<T> {
    const fn new() -> Self {
        Self {
            refs: Vec::new(),
            by_key: HashMap::with_hasher(FxBuildHasher),
            tags: None,
        }
    }
}

pub struct Registry<T: 'static> {
    key: &'static str,
    loading: Mutex<Entries<T>>,
    frozen: OnceLock<Entries<T>>,
}

impl<T: Send + Sync + 'static> Registry<T> {
    pub const fn new(key: &'static str) -> Self {
        Self {
            key,
            loading: Mutex::new(Entries::new()),
            frozen: OnceLock::new(),
        }
    }

    pub fn key(&self) -> RegistryKey<Registry<T>> {
        RegistryKey::of(self.key.to_owned())
    }

    fn with<R>(&self, f: impl FnOnce(&Entries<T>) -> R) -> R {
        if let Some(entries) = self.frozen.get() {
            return f(entries);
        }
        if !is_frozen() {
            return f(&self.loading.lock());
        }

        f(self.frozen.get_or_init(|| {
            let mut entries = self.loading.lock();
            if entries.tags.is_none() {
                entries.tags = Some(Arc::new(self.load_bundled_tags(&entries)));
            }
            std::mem::replace(&mut *entries, Entries::new())
        }))
    }

    fn write<R>(&self, f: impl FnOnce(&mut Entries<T>) -> R) -> Result<R, RegistryFrozen> {
        let mut entries = self.loading.lock();
        if is_frozen() {
            return Err(RegistryFrozen {
                registry: self.key().to_key(),
            });
        }
        Ok(f(&mut entries))
    }

    pub fn register(&self, key: Key, value: T) -> Result<Id, RegistryFrozen> {
        self.write(|entries| {
            let id = match entries.by_key.get(&key) {
                Some(&id) => id,
                None => Id::try_from(entries.refs.len()).expect("registry id out of range"),
            };
            let entry: &'static Ref<T> = Box::leak(Box::new(Ref::new(key.clone(), id, value)));

            if (id as usize) < entries.refs.len() {
                entries.refs[id as usize] = entry;
            } else {
                entries.refs.push(entry);
                entries.by_key.insert(key, id);
            }
            id
        })
    }

    pub fn len(&self) -> usize {
        self.with(|entries| entries.refs.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn by_id(&self, id: Id) -> Option<&'static Ref<T>> {
        self.with(|entries| entries.refs.get(id as usize).copied())
    }

    pub fn by_key(&self, key: &Key) -> Option<&'static Ref<T>> {
        self.with(|entries| entries.by_key.get(key).map(|&id| entries.refs[id as usize]))
    }

    pub fn key_of(&self, id: Id) -> Option<&'static Key> {
        self.by_id(id).map(Ref::key)
    }

    pub fn get_id(&self, key: &Key) -> Option<Id> {
        self.with(|entries| entries.by_key.get(key).copied())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static Key, &'static T)> + use<T> {
        self.refs().map(|entry| (entry.key(), entry.value()))
    }

    pub fn values(&self) -> impl Iterator<Item = &'static T> + use<T> {
        self.refs().map(Ref::value)
    }

    fn refs(&self) -> impl Iterator<Item = &'static Ref<T>> + use<T> {
        self.with(|entries| entries.refs.clone()).into_iter()
    }

    pub fn tags(&self) -> Arc<Tags> {
        self.with_tags(|_, tags| tags.clone())
    }

    fn with_tags<R>(&self, f: impl FnOnce(&Entries<T>, &Arc<Tags>) -> R) -> R {
        if self.frozen.get().is_none() && !is_frozen() {
            let mut entries = self.loading.lock();
            if entries.tags.is_none() {
                entries.tags = Some(Arc::new(self.load_bundled_tags(&entries)));
            }
            let tags = entries.tags.clone().expect("tags were just loaded");
            return f(&entries, &tags);
        }
        self.with(|entries| {
            f(
                entries,
                entries.tags.as_ref().expect("frozen registries have tags"),
            )
        })
    }

    fn load_bundled_tags(&self, entries: &Entries<T>) -> Tags {
        let directory = format!("tags/{}", self.key().path());
        if !shulkr_data::has_json(&directory) {
            return Tags::default();
        }

        let mut loader = TagLoader::new();
        for (path, file) in shulkr_data::load_json::<TagFile>(&directory) {
            loader.add(Key::vanilla(path), file);
        }
        loader.build(|key| entries.by_key.contains_key(key))
    }

    pub fn set_tags(&self, tags: Tags) -> Result<(), RegistryFrozen> {
        self.write(|entries| entries.tags = Some(Arc::new(tags)))
    }

    pub fn has_tag(&self, key: &Key, tag: &Key) -> bool {
        self.with_tags(|_, tags| tags.contains(tag, key))
    }

    pub fn iter_tag(&self, tag: &Key) -> impl Iterator<Item = (&'static Key, &'static T)> + use<T> {
        let refs: Vec<_> = self.with_tags(|entries, tags| {
            tags.get(tag)
                .map(|tag| {
                    tag.keys()
                        .iter()
                        .filter_map(|key| entries.by_key.get(key))
                        .map(|&id| entries.refs[id as usize])
                        .collect()
                })
                .unwrap_or_default()
        });
        refs.into_iter().map(|entry| (entry.key(), entry.value()))
    }
}

const FIRST_KEY: Key = Key::const_vanilla("plains");

pub(crate) fn load_json<T: DeserializeOwned + Send + Sync>(registry: &Registry<T>) {
    let mut entries: IndexMap<Key, T> = shulkr_data::load_json(registry.key().path())
        .into_iter()
        .map(|(name, value)| (Key::of(name), value))
        .collect();
    if let Some(index) = entries.get_index_of(&FIRST_KEY) {
        entries.move_index(index, 0);
    }

    for (key, value) in entries {
        registry
            .register(key, value)
            .expect("vanilla registries are filled before freezing");
    }
}
