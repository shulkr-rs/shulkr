use serde::{Deserialize, Serialize};

use crate::util::{deserialize_color, deserialize_optional_color};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeEffects {
    #[serde(deserialize_with = "deserialize_color")]
    water_color: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_color"
    )]
    foliage_color: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_color"
    )]
    dry_foliage_color: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_color"
    )]
    grass_color: Option<String>,
    #[serde(default)]
    grass_color_modifier: GrassColorModifier,
}

pub struct BiomeEffectsBuilder {
    water_color: Option<String>,
    foliage_color: Option<String>,
    dry_foliage_color: Option<String>,
    grass_color: Option<String>,
    grass_color_modifier: GrassColorModifier,
}

impl BiomeEffectsBuilder {
    pub fn new() -> Self {
        Self {
            water_color: None,
            foliage_color: None,
            dry_foliage_color: None,
            grass_color: None,
            grass_color_modifier: GrassColorModifier::None,
        }
    }

    pub fn water_color(mut self, water_color: impl Into<String>) -> Self {
        self.water_color = Some(water_color.into());
        self
    }

    pub fn foliage_color(mut self, foliage_color: impl Into<String>) -> Self {
        self.foliage_color = Some(foliage_color.into());
        self
    }

    pub fn dry_foliage_color(mut self, dry_foliage_color: impl Into<String>) -> Self {
        self.dry_foliage_color = Some(dry_foliage_color.into());
        self
    }

    pub fn grass_color(mut self, grass_color: impl Into<String>) -> Self {
        self.grass_color = Some(grass_color.into());
        self
    }

    pub fn grass_color_modifier(mut self, grass_color_modifier: GrassColorModifier) -> Self {
        self.grass_color_modifier = grass_color_modifier;
        self
    }

    pub fn build(self) -> BiomeEffects {
        BiomeEffects {
            water_color: self.water_color.expect("missing 'water color'."),
            foliage_color: self.foliage_color,
            dry_foliage_color: self.dry_foliage_color,
            grass_color: self.grass_color,
            grass_color_modifier: self.grass_color_modifier,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrassColorModifier {
    #[default]
    None,
    DarkForest,
    Swamp,
}
