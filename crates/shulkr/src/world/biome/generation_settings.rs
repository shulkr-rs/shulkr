use crate::util::Key;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BiomeGenerationSettings {
    #[serde(deserialize_with = "deserialize_holder_set")]
    carvers: Vec<Key>,
    #[serde(deserialize_with = "deserialize_holder_sets")]
    features: Vec<Vec<Key>>,
}

impl BiomeGenerationSettings {
    pub fn carvers(&self) -> &[Key] {
        &self.carvers
    }

    pub fn features(&self) -> &[Vec<Key>] {
        &self.features
    }
}

pub struct BiomeGenerationSettingsBuilder {
    carvers: Vec<Key>,
    features: Vec<Vec<Key>>,
}

impl Default for BiomeGenerationSettingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BiomeGenerationSettingsBuilder {
    pub fn new() -> Self {
        Self {
            carvers: Vec::new(),
            features: Vec::new(),
        }
    }

    pub fn carver(mut self, carver: impl Into<Key>) -> Self {
        self.carvers.push(carver.into());
        self
    }

    pub fn feature(mut self, step: usize, feature: impl Into<Key>) -> Self {
        if self.features.len() <= step {
            self.features.resize_with(step + 1, Vec::new);
        }
        self.features[step].push(feature.into());
        self
    }

    pub fn build(self) -> BiomeGenerationSettings {
        BiomeGenerationSettings {
            carvers: self.carvers,
            features: self.features,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum HolderSet {
    One(Key),
    Many(Vec<Key>),
}

impl From<HolderSet> for Vec<Key> {
    fn from(set: HolderSet) -> Self {
        match set {
            HolderSet::One(key) => vec![key],
            HolderSet::Many(keys) => keys,
        }
    }
}

fn deserialize_holder_set<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Key>, D::Error> {
    HolderSet::deserialize(d).map(Into::into)
}

fn deserialize_holder_sets<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Vec<Key>>, D::Error> {
    Vec::<HolderSet>::deserialize(d).map(|sets| sets.into_iter().map(Into::into).collect())
}
