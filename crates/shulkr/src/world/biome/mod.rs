mod effects;
mod generation_settings;

pub use effects::*;
pub use generation_settings::*;

use crate::world::attribute::EnvironmentAttributeMap;
use serde::{Deserialize, Serialize};

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
    #[serde(flatten, skip_serializing)]
    generation_settings: BiomeGenerationSettings,
}

impl Biome {
    pub fn has_precipitation(&self) -> bool {
        self.has_precipitation
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    pub fn temperature_modifier(&self) -> TemperatureModifier {
        self.temperature_modifier
            .unwrap_or(TemperatureModifier::None)
    }

    pub fn downfall(&self) -> f32 {
        self.downfall
    }

    pub fn attributes(&self) -> &EnvironmentAttributeMap {
        &self.attributes
    }

    pub fn effects(&self) -> &BiomeEffects {
        &self.effects
    }

    pub fn generation_settings(&self) -> &BiomeGenerationSettings {
        &self.generation_settings
    }
}

pub struct BiomeBuilder {
    has_precipitation: bool,
    temperature: Option<f32>,
    temperature_modifier: Option<TemperatureModifier>,
    downfall: Option<f32>,
    attributes: EnvironmentAttributeMap,
    effects: Option<BiomeEffects>,
    generation_settings: BiomeGenerationSettings,
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
            generation_settings: BiomeGenerationSettings::default(),
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

    pub fn generation_settings(mut self, generation_settings: BiomeGenerationSettings) -> Self {
        self.generation_settings = generation_settings;
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
            generation_settings: self.generation_settings,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureModifier {
    None,
    Frozen,
}
