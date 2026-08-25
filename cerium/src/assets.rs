use rust_embed::Embed;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

#[derive(Embed)]
#[folder = "build_assets/datapack/"]
pub(crate) struct Assets;

const NAMESPACE: &str = "minecraft";

pub(crate) fn load_json<T: DeserializeOwned>(registry: &str) -> BTreeMap<String, T> {
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
