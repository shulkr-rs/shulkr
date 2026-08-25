use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use convert_case::{Case, Casing as _};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod object;

mod r#static {
    pub mod block;
    pub mod block_entity_type;
    pub mod entity_type;
    pub mod material;
}
pub use r#static::*;

const DATAPACK_REGISTRIES: &[&str] = &[
    "banner_pattern",
    "cat_sound_variant",
    "cat_variant",
    "chicken_sound_variant",
    "chicken_variant",
    "cow_sound_variant",
    "cow_variant",
    "damage_type",
    "dimension_type",
    "frog_variant",
    "instrument",
    "jukebox_song",
    "painting_variant",
    "pig_sound_variant",
    "pig_variant",
    "timeline",
    "trim_material",
    "wolf_sound_variant",
    "wolf_variant",
    "world_clock",
    "worldgen/biome",
    "zombie_nautilus_variant",
];

const TAG_REGISTRIES: &[&str] = &[
    "banner_pattern",
    "block",
    "damage_type",
    "instrument",
    "timeline",
];

fn main() {
    ensure_assets();

    for registry in DATAPACK_REGISTRIES {
        generate_datapack(registry);
    }
    for registry in TAG_REGISTRIES {
        watch(&datapack_dir(&format!("tags/{registry}")));
    }

    block::generate();
    block_entity_type::generate();
    entity_type::generate();
    material::generate();
}

const DATAGEN_REPO: &str = "https://github.com/garfxld/shulkr-datagen";

fn ensure_assets() {
    assert!(
        Path::new("build_assets/datapack").exists(),
        "build assets are missing from `cerium/build_assets/`. They are committed to this \
         repository; regenerate them with `{DATAGEN_REPO}`.\n"
    );
}

pub fn read_asset(name: &str) -> String {
    let path = Path::new("build_assets").join(name);
    watch(&path);

    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "failed to read `{}`: {e}\nthe build assets in `cerium/build_assets/` look incomplete; regenerate them with `{DATAGEN_REPO}`",
            path.display()
        )
    })
}

fn datapack_dir(registry: &str) -> PathBuf {
    Path::new("build_assets/datapack/minecraft").join(registry)
}

fn read_registry(registry: &str) -> BTreeMap<String, serde_json::Value> {
    let dir = datapack_dir(registry);
    watch(&dir);

    let mut entries = BTreeMap::new();
    read_entries(&dir, &dir, &mut entries);

    assert!(
        !entries.is_empty(),
        "`{}` holds no entries; the build assets in `cerium/build_assets/` look incomplete, \
         regenerate them with `{DATAGEN_REPO}`",
        dir.display()
    );

    entries
}

fn read_entries(root: &Path, dir: &Path, entries: &mut BTreeMap<String, serde_json::Value>) {
    let children = fs::read_dir(dir).unwrap_or_else(|e| {
        panic!(
            "failed to read `{}`: {e}\nthe build assets in `cerium/build_assets/` look \
             incomplete; regenerate them with `{DATAGEN_REPO}`",
            dir.display()
        )
    });

    for child in children {
        let path = child.expect("failed to read a directory entry").path();

        if path.is_dir() {
            read_entries(root, &path, entries);
            continue;
        }
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }

        let key = path
            .strip_prefix(root)
            .expect("entry must live below the registry")
            .with_extension("")
            .to_string_lossy()
            .replace('\\', "/");

        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read `{}`: {e}", path.display()));
        let entry = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("failed to parse `{}`: {e}", path.display()));

        entries.insert(key, entry);
    }
}

fn watch(path: &Path) {
    println!("cargo::rerun-if-changed={}", path.display());
}

pub fn write_file(content: &TokenStream, dst: &str) {
    let path = Path::new("src/registry/generated").join(dst);

    fs::create_dir_all(path.parent().unwrap()).expect("failed to create the output directory");
    fs::write(&path, content.to_string())
        .unwrap_or_else(|e| panic!("failed to write `{}`: {e}", path.display()));

    let _ = Command::new("rustfmt").arg(&path).output();
}

fn generate_datapack(registry: &str) {
    let name = registry
        .rsplit('/')
        .next()
        .expect("registry path must not be empty");
    let strct = format_ident!("{}", name.to_case(Case::UpperCamel));

    let keys: TokenStream = read_registry(registry)
        .into_keys()
        .map(|key| {
            let ident = format_ident!("{}", const_name(&key));

            quote! { pub const #ident: RegistryKey<#strct> = RegistryKey::const_vanilla(#key); }
        })
        .collect();

    let out = quote! {
        use crate::registry::{RegistryKey, #strct};

        #[allow(unused)]
        impl #strct {
            #keys
        }
    };

    write_file(&out, &format!("{name}s.rs"));
}

fn const_name(name: &str) -> String {
    let name = match name {
        "5" => "FIVE",
        "11" => "ELEVEN",
        "13" => "THIRTEEN",
        _ => &name.to_uppercase(),
    };

    assert!(
        !name.starts_with(|char: char| char.is_ascii_digit()),
        "`{name}` is not a valid identifier, add it to `const_name`"
    );

    name.to_owned()
}
