use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AmbientSounds {
    #[serde(rename = "loop", skip_serializing_if = "Option::is_none")]
    pub sound_loop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<AmbientMood>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_additions"
    )]
    pub additions: Option<Vec<AmbientAdditions>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientMood {
    pub sound: String,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientAdditions {
    pub sound: String,
    pub tick_chance: f64,
}

fn deserialize_additions<'de, D>(deserializer: D) -> Result<Option<Vec<AmbientAdditions>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Additions {
        One(AmbientAdditions),
        Many(Vec<AmbientAdditions>),
    }

    Ok(match Option::<Additions>::deserialize(deserializer)? {
        Some(Additions::One(additions)) => Some(vec![additions]),
        Some(Additions::Many(additions)) => Some(additions),
        None => None,
    })
}
