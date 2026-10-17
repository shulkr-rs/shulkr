use rust_embed::Embed;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

#[derive(Embed)]
#[folder = "assets/datapack/"]
struct Assets;

const NAMESPACE: &str = "minecraft";

pub const DATAPACK_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/datapack/minecraft");

pub fn has_json(registry: &str) -> bool {
    let prefix = format!("{NAMESPACE}/{registry}/");
    Assets::iter().any(|path| path.starts_with(&prefix) && path.ends_with(".json"))
}

pub fn load_json<T: DeserializeOwned>(registry: &str) -> BTreeMap<String, T> {
    let prefix = format!("{NAMESPACE}/{registry}/");

    let entries: BTreeMap<String, T> = Assets::iter()
        .filter_map(|path| {
            let key = path
                .strip_prefix(&prefix)?
                .strip_suffix(".json")?
                .to_owned();

            let file = Assets::get(&path).expect("the iterated asset must exist");
            let entry = serde_json::from_slice(&file.data)
                .unwrap_or_else(|error| panic!("failed to parse `{path}`: {error}"));

            Some((key, entry))
        })
        .collect();

    assert!(!entries.is_empty(), "missing build assets for `{registry}`");

    entries
}
