use serde::{Deserialize, Serialize};

use crate::world::attribute::EnvironmentAttributeMap;

mod effects;
pub use effects::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biome {
    has_precipitation: bool,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_modifier: Option<TemperatureModifier>,
    downfall: f32,
    #[serde(default, skip_serializing_if = "EnvironmentAttributeMap::is_empty")]
    attributes: EnvironmentAttributeMap,
    effects: BiomeEffects,
}

impl Biome {
    pub fn attributes(&self) -> &EnvironmentAttributeMap {
        &self.attributes
    }

    pub fn effects(&self) -> &BiomeEffects {
        &self.effects
    }
}

pub struct BiomeBuilder {
    has_precipitation: bool,
    temperature: Option<f32>,
    temperature_modifier: Option<TemperatureModifier>,
    downfall: Option<f32>,
    attributes: EnvironmentAttributeMap,
    effects: Option<BiomeEffects>,
}

impl Default for BiomeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BiomeBuilder {
    pub fn new() -> Self {
        Self {
            has_precipitation: true,
            temperature: None,
            temperature_modifier: None,
            downfall: None,
            attributes: EnvironmentAttributeMap::new(),
            effects: None,
        }
    }

    pub fn has_precipitation(mut self, has_precipitation: bool) -> Self {
        self.has_precipitation = has_precipitation;
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn temperature_modifier(mut self, temperature_modifier: TemperatureModifier) -> Self {
        self.temperature_modifier = Some(temperature_modifier);
        self
    }

    pub fn downfall(mut self, downfall: f32) -> Self {
        self.downfall = Some(downfall);
        self
    }

    pub fn attributes(mut self, attributes: EnvironmentAttributeMap) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn effects(mut self, effects: BiomeEffects) -> Self {
        self.effects = Some(effects);
        self
    }

    pub fn build(self) -> Biome {
        Biome {
            has_precipitation: self.has_precipitation,
            temperature: self.temperature.unwrap(),
            temperature_modifier: self.temperature_modifier,
            downfall: self.downfall.unwrap(),
            attributes: self.attributes,
            effects: self.effects.unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureModifier {
    None,
    Frozen,
}
