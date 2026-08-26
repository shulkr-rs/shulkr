use std::borrow::Cow;

use serde::{Serialize, Serializer};

use crate::util::Key;
use crate::world::Particle;
use crate::world::attribute::{
    AmbientParticle, AmbientSounds, BackgroundMusic, BedRule, MoonPhase, TriState,
};

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Boolean(bool),
    Float(f32),
    Color(Cow<'static, str>),
    TriState(TriState),
    MoonPhase(MoonPhase),
    Activity(Key),
    BedRule(BedRule),
    Particle(Particle),
    AmbientParticles(Vec<AmbientParticle>),
    BackgroundMusic(BackgroundMusic),
    AmbientSounds(AmbientSounds),
}

impl Serialize for AttributeValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AttributeValue::Boolean(value) => value.serialize(serializer),
            AttributeValue::Float(value) => value.serialize(serializer),
            AttributeValue::Color(value) => value.serialize(serializer),
            AttributeValue::TriState(value) => value.serialize(serializer),
            AttributeValue::MoonPhase(value) => value.serialize(serializer),
            AttributeValue::Activity(value) => serializer.serialize_str(&value.to_string()),
            AttributeValue::BedRule(value) => value.serialize(serializer),
            AttributeValue::Particle(value) => value.serialize(serializer),
            AttributeValue::AmbientParticles(value) => value.serialize(serializer),
            AttributeValue::BackgroundMusic(value) => value.serialize(serializer),
            AttributeValue::AmbientSounds(value) => value.serialize(serializer),
        }
    }
}

pub trait AttributeValueType: Sized {
    fn into_value(self) -> AttributeValue;

    fn from_value(value: &AttributeValue) -> Option<&Self>;
}

macro_rules! attribute_value_type {
    ($ty:ty, $variant:ident) => {
        impl AttributeValueType for $ty {
            fn into_value(self) -> AttributeValue {
                AttributeValue::$variant(self)
            }

            fn from_value(value: &AttributeValue) -> Option<&Self> {
                match value {
                    AttributeValue::$variant(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}

attribute_value_type!(bool, Boolean);
attribute_value_type!(f32, Float);
impl AttributeValueType for Cow<'static, str> {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Color(self)
    }

    fn from_value(value: &AttributeValue) -> Option<&Self> {
        match value {
            AttributeValue::Color(value) => Some(value),
            _ => None,
        }
    }
}
attribute_value_type!(TriState, TriState);
attribute_value_type!(MoonPhase, MoonPhase);
attribute_value_type!(BedRule, BedRule);
attribute_value_type!(Particle, Particle);
attribute_value_type!(Vec<AmbientParticle>, AmbientParticles);
attribute_value_type!(BackgroundMusic, BackgroundMusic);
attribute_value_type!(AmbientSounds, AmbientSounds);

impl AttributeValueType for Key {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Activity(self)
    }

    fn from_value(value: &AttributeValue) -> Option<&Self> {
        match value {
            AttributeValue::Activity(value) => Some(value),
            _ => None,
        }
    }
}
