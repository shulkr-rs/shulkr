use std::{
    collections::BTreeMap,
    fs,
    io::Write as _,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use convert_case::{Case, Casing as _};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod object;
mod version;

mod r#static {
    pub mod block;
    pub mod block_entity_type;
    pub mod entity_type;
    pub mod material;
}
pub use r#static::*;

const HEADER: &str = "// This file was auto-generated. Do not edit it manually.\n";
const MAX_WIDTH: usize = 50000;

const ASSETS: &str = "../assets";
const OUT_DIR: &str = "../crates/cerium/generated";

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

fn main() {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))
        .expect("failed to enter the manifest directory");

    ensure_assets();

    version::generate();

    for registry in DATAPACK_REGISTRIES {
        generate_datapack(registry);
    }

    block::generate();
    block_entity_type::generate();
    entity_type::generate();
    material::generate();
}

fn ensure_assets() {
    assert!(
        asset_path("datapack").exists(),
        "build assets are missing from `assets/`."
    );
}

fn asset_path(name: &str) -> PathBuf {
    Path::new(ASSETS).join(name)
}

pub fn read_asset(name: &str) -> String {
    let path = asset_path(name);

    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "failed to read `{}`: {e}\nthe build assets in `assets/`.",
            path.display()
        )
    })
}

fn datapack_dir(registry: &str) -> PathBuf {
    asset_path("datapack/minecraft").join(registry)
}

fn read_registry(registry: &str) -> BTreeMap<String, serde_json::Value> {
    let dir = datapack_dir(registry);

    let mut entries = BTreeMap::new();
    read_entries(&dir, &dir, &mut entries);

    entries
}

fn read_entries(root: &Path, dir: &Path, entries: &mut BTreeMap<String, serde_json::Value>) {
    let children = fs::read_dir(dir).unwrap_or_else(|e| {
        panic!(
            "failed to read `{}`: {e}\nthe build assets in `assets/`.",
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

pub fn write_file(content: &TokenStream, dst: &str) {
    write(content, dst, None);
}

pub fn write_wide_file(content: &TokenStream, dst: &str) {
    write(content, dst, Some(MAX_WIDTH));
}

fn write(content: &TokenStream, dst: &str, width: Option<usize>) {
    let path = Path::new(OUT_DIR).join(dst);

    let content = format_code(&content.to_string(), width).unwrap_or_else(|| content.to_string());
    let content = separate_imports(&content);
    let content = if width.is_some() {
        skip_formatting(&content)
    } else {
        content
    };
    let content = format!("{HEADER}\n{content}");

    if fs::read_to_string(&path).is_ok_and(|existing| existing == content) {
        return;
    }

    fs::create_dir_all(path.parent().unwrap()).expect("failed to create the output directory");
    fs::write(&path, content)
        .unwrap_or_else(|e| panic!("failed to write `{}`: {e}", path.display()));

    println!("generated {}", path.display());
}

fn separate_imports(code: &str) -> String {
    let mut lines = code.lines().peekable();

    let mut imports = String::new();
    while lines.peek().is_some_and(|line| line.starts_with("use ")) {
        imports.push_str(lines.next().expect("peeked at a line"));
        imports.push('\n');
    }

    let rest: String = lines.map(|line| format!("{line}\n")).collect();
    if imports.is_empty() || rest.is_empty() {
        return code.to_owned();
    }

    format!("{imports}\n{rest}")
}

fn skip_formatting(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    let mut skipped = false;

    for line in code.lines() {
        let starts_item = !skipped
            && !line.is_empty()
            && !line.starts_with(char::is_whitespace)
            && !line.starts_with("use ")
            && !line.starts_with("//");

        if starts_item {
            out.push_str("#[rustfmt::skip]\n");
            skipped = true;
        }

        out.push_str(line);
        out.push('\n');
    }

    out
}

fn format_code(code: &str, width: Option<usize>) -> Option<String> {
    let width = width.map(|width| format!("max_width={width}"));

    let mut child = Command::new("rustfmt")
        .args(width.iter().flat_map(|width| ["--config", width]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    child.stdin.take()?.write_all(code.as_bytes()).ok()?;

    let output = child.wait_with_output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8(output.stdout).ok())
        .flatten()
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

        impl #strct {
            #keys
        }
    };

    write_wide_file(&out, &format!("{name}s.rs"));
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
