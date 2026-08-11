use std::{fs::File, io::Write as _, path::Path, process::Command};

use indexmap::IndexMap;
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

fn main() {
    // Dynamic Registries
    generate(
        "Biome",
        "biomes.rs",
        include_str!("../build_assets/worldgen/biome.json"),
    );
    generate(
        "CatVariant",
        "cat_variants.rs",
        include_str!("../build_assets/cat_variant.json"),
    );
    generate(
        "ChickenVariant",
        "chicken_variants.rs",
        include_str!("../build_assets/chicken_variant.json"),
    );
    generate(
        "CowVariant",
        "cow_variants.rs",
        include_str!("../build_assets/cow_variant.json"),
    );
    generate(
        "DamageType",
        "damage_types.rs",
        include_str!("../build_assets/damage_type.json"),
    );
    generate(
        "FrogVariant",
        "frog_variants.rs",
        include_str!("../build_assets/frog_variant.json"),
    );
    generate(
        "PaintingVariant",
        "painting_variants.rs",
        include_str!("../build_assets/painting_variant.json"),
    );
    generate(
        "PigVariant",
        "pig_variants.rs",
        include_str!("../build_assets/pig_variant.json"),
    );
    generate(
        "WolfSoundVariant",
        "wolf_sound_variants.rs",
        include_str!("../build_assets/wolf_sound_variant.json"),
    );
    generate(
        "WolfVariant",
        "wolf_variants.rs",
        include_str!("../build_assets/wolf_variant.json"),
    );

    // Static Registries
    block::generate();
    block_entity_type::generate();
    entity_type::generate();
    material::generate();
}

pub fn write_file(content: &TokenStream, dst: &str) {
    let path = Path::new("src/registry/generated").join(dst);
    if !path.parent().unwrap().exists() {
        std::fs::create_dir(&path.parent().unwrap()).unwrap();
    }

    let mut file = File::create(&path).unwrap();
    if let Err(e) = file.write_all(content.to_string().as_bytes()) {
        println!("cargo::error={e}");
    }

    let _ = Command::new("rustfmt").arg(&path).output();
}

pub fn generate(strct: &str, dst: &str, content: &str) {
    let entries: IndexMap<String, serde_json::Value> = serde_json::from_str(content).unwrap();

    let strct = format_ident!("{}", strct);

    let keys: TokenStream = entries
        .keys()
        .map(|key| {
            let path = key.split_once(':').map_or(key.as_str(), |(_, path)| path);
            let ident = format_ident!("{}", path.to_uppercase());

            quote! {
                pub const #ident: RegistryKey<#strct> = RegistryKey::const_vanilla(#path);
            }
        })
        .collect();

    let out = quote! {
        use crate::registry::{RegistryKey, #strct};

        #[allow(unused)]
        impl #strct {
            #keys
        }

    };

    write_file(&out, dst);
}
