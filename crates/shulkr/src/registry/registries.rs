use super::*;
use crate::{
    util::{RwLock, RwLockReadGuard},
    world::block::BlockEntityType,
};
use std::path::{Path, PathBuf};

pub struct Registries {
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
    pub jukebox_song: Registry<JukeboxSong>,
    pub banner_pattern: Registry<BannerPattern>,
    pub instrument: Registry<Instrument>,
}

const FIRST_KEY: Key = Key::const_vanilla("plains");

fn move_first<T>(entries: &mut IndexMap<Key, T>) {
    if let Some(value) = entries.swap_remove(&FIRST_KEY) {
        let old_entries = std::mem::take(entries);
        entries.insert(FIRST_KEY, value);
        entries.extend(old_entries);
    }
}

pub fn load_datapack<T>(id: &'static str) -> Registry<T>
where
    T: Serialize + DeserializeOwned,
{
    let mut registry = Registry::<T>::new(RegistryKey::const_vanilla(id));

    let dir = id.strip_prefix("minecraft:").unwrap_or(id);
    let mut entries: IndexMap<Key, T> = shulkr_data::load_json(dir)
        .into_iter()
        .map(|(name, value)| (Key::of(name), value))
        .collect();
    move_first(&mut entries);

    for (key, value) in entries {
        Registry::register(&mut registry, key, value);
    }

    registry
}

#[derive(thiserror::Error, Debug)]
pub enum RegistryLoadError {
    #[error("failed to read {path}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse {path}: {source}")]
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },
}

fn collect_json_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), RegistryLoadError> {
    let entries = std::fs::read_dir(dir).map_err(|source| RegistryLoadError::Read {
        path: dir.to_path_buf(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| RegistryLoadError::Read {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, files)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            files.push(path);
        }
    }

    Ok(())
}

fn path_to_key(namespace: &str, root: &Path, path: &Path) -> Key {
    let relative = path.strip_prefix(root).unwrap_or(path).with_extension("");
    let path = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    Key::new(namespace.to_owned(), path)
}

pub fn register_biome(key: impl Into<Key>, biome: Biome) -> Id {
    Registry::register(&mut Registries::BIOME.write(), key.into(), biome)
}

pub fn load_biomes(namespace: &str, dir: impl AsRef<Path>) -> Result<usize, RegistryLoadError> {
    let dir = dir.as_ref();
    let mut files = Vec::new();
    collect_json_files(dir, &mut files)?;
    files.sort();

    for path in &files {
        let data = std::fs::read_to_string(path).map_err(|source| RegistryLoadError::Read {
            path: path.clone(),
            source,
        })?;
        let biome: Biome =
            serde_json::from_str(&data).map_err(|source| RegistryLoadError::Parse {
                path: path.clone(),
                source,
            })?;

        register_biome(path_to_key(namespace, dir, path), biome);
    }

    Ok(files.len())
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

macro_rules! mutable_registry {
    ($path:path, $ty:ty, $expr:expr) => {{
        static VALUE: LazyLock<RwLock<Registry<$ty>>> = LazyLock::new(|| {
            let mut registry = Registry::new($expr);
            $path(&mut registry);
            RwLock::new(registry)
        });
        &VALUE
    }};
}

fn register_biomes(registry: &mut Registry<Biome>) {
    *registry = load_datapack("worldgen/biome");
}

impl Default for Registries {
    fn default() -> Self {
        Self::new()
    }
}

impl Registries {
    pub const BLOCK: &LazyLock<Registry<Block>> = registry!(
        crate::world::block::register_all,
        Block,
        RegistryKeys::BLOCK
    );
    pub const BLOCK_ENTITY_TYPE: &LazyLock<Registry<BlockEntityType>> = registry!(
        crate::world::block::block_entity::register_all,
        BlockEntityType,
        RegistryKeys::BLOCK_ENTITY_TYPE
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
    pub const ENVIRONMENT_ATTRIBUTE: &LazyLock<Registry<EnvironmentAttribute>> = registry!(
        crate::world::attribute::register_all,
        EnvironmentAttribute,
        RegistryKeys::ENVIRONMENT_ATTRIBUTE
    );
    pub const BIOME: &LazyLock<RwLock<Registry<Biome>>> =
        mutable_registry!(register_biomes, Biome, RegistryKeys::BIOME);

    pub fn biomes() -> RwLockReadGuard<'static, Registry<Biome>> {
        Self::BIOME.read()
    }

    #[rustfmt::skip]
    pub fn new() -> Self {

        Self {
            damage_type:                load_datapack("damage_type"),
            banner_pattern:             load_datapack("banner_pattern"),
            instrument:                 load_datapack("instrument"),
            jukebox_song:               load_datapack("jukebox_song"),
            trim_material:              load_datapack("trim_material"),

            // World
            dimension_type:             load_datapack("dimension_type"),
            timeline:                   load_datapack("timeline"),
            world_clock:                load_datapack("world_clock"),

            // Entities
            cat_variant:                load_datapack("cat_variant"),
            cat_sound_variant:          load_datapack("cat_sound_variant"),
            chicken_variant:            load_datapack("chicken_variant"),
            chicken_sound_variant:      load_datapack("chicken_sound_variant"),
            cow_variant:                load_datapack("cow_variant"),
            cow_sound_variant:          load_datapack("cow_sound_variant"),
            frog_variant:               load_datapack("frog_variant"),
            painting_variant:           load_datapack("painting_variant"),
            pig_variant:                load_datapack("pig_variant"),
            pig_sound_variant:          load_datapack("pig_sound_variant"),
            wolf_variant:               load_datapack("wolf_variant"),
            wolf_sound_variant:         load_datapack("wolf_sound_variant"),
            zombie_nautilus_variant:    load_datapack("zombie_nautilus_variant"),
        }
    }
}
