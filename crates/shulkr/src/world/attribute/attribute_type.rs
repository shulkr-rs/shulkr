use crate::{
    util::{Key, deserialize_color},
    world::{
        Particle,
        attribute::{AmbientSounds, AttributeValue, BackgroundMusic, BedRule, MoonPhase, TriState},
    },
};
use serde::{Deserialize, Deserializer, de::DeserializeSeed};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeType {
    Boolean,
    TriState,
    Float,
    AngleDegrees,
    RgbColor,
    ArgbColor,
    MoonPhase,
    Activity,
    BedRule,
    Particle,
    AmbientParticles,
    BackgroundMusic,
    AmbientSounds,
}

pub(crate) struct ValueSeed(pub(crate) AttributeType);

impl<'de> DeserializeSeed<'de> for ValueSeed {
    type Value = AttributeValue;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Ok(match self.0 {
            AttributeType::Boolean => AttributeValue::Boolean(bool::deserialize(deserializer)?),
            AttributeType::Float | AttributeType::AngleDegrees => {
                AttributeValue::Float(f32::deserialize(deserializer)?)
            }
            AttributeType::RgbColor | AttributeType::ArgbColor => {
                AttributeValue::Color(deserialize_color(deserializer)?.into())
            }
            AttributeType::TriState => {
                AttributeValue::TriState(TriState::deserialize(deserializer)?)
            }
            AttributeType::MoonPhase => {
                AttributeValue::MoonPhase(MoonPhase::deserialize(deserializer)?)
            }
            AttributeType::Activity => {
                AttributeValue::Activity(Key::of(String::deserialize(deserializer)?))
            }
            AttributeType::BedRule => AttributeValue::BedRule(BedRule::deserialize(deserializer)?),
            AttributeType::Particle => {
                AttributeValue::Particle(Particle::deserialize(deserializer)?)
            }
            AttributeType::AmbientParticles => {
                AttributeValue::AmbientParticles(Vec::deserialize(deserializer)?)
            }
            AttributeType::BackgroundMusic => {
                AttributeValue::BackgroundMusic(BackgroundMusic::deserialize(deserializer)?)
            }
            AttributeType::AmbientSounds => {
                AttributeValue::AmbientSounds(AmbientSounds::deserialize(deserializer)?)
            }
        })
    }
}
