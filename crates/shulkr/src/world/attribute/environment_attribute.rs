use std::borrow::Cow;

use crate::registry::{Id, Registries};
use crate::text::TextComponent;
use crate::util::Key;
use crate::world::Particle;
use crate::world::attribute::{
    AmbientSounds, AttributeType, AttributeValue, BackgroundMusic, BedRule, BedRuleKind, MoonPhase,
    TriState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnvironmentAttribute(Id);

include!("../../../generated/environment_attributes.rs");

pub struct EnvironmentAttributeData {
    pub key: Key,
    pub r#type: AttributeType,
    pub default_value: AttributeValue,
}

impl EnvironmentAttributeData {
    pub const fn new(
        path: &'static str,
        r#type: AttributeType,
        default_value: AttributeValue,
    ) -> Self {
        Self {
            key: Key::const_vanilla(path),
            r#type,
            default_value,
        }
    }
}

const fn color(value: &'static str) -> AttributeValue {
    AttributeValue::Color(Cow::Borrowed(value))
}

impl EnvironmentAttribute {
    pub fn key(&self) -> &'static Key {
        &self.data().key
    }

    pub fn r#type(&self) -> AttributeType {
        self.data().r#type
    }

    pub fn default_value(&self) -> &'static AttributeValue {
        &self.data().default_value
    }

    pub fn from_id(id: Id) -> Option<EnvironmentAttribute> {
        Self::try_from(id).ok()
    }

    pub fn from_key(key: &Key) -> Option<EnvironmentAttribute> {
        Registries::ENVIRONMENT_ATTRIBUTE.by_key(key).copied()
    }
}

impl From<EnvironmentAttribute> for Id {
    #[inline]
    fn from(attribute: EnvironmentAttribute) -> Self {
        attribute.0
    }
}

impl TryFrom<Id> for EnvironmentAttribute {
    type Error = ();

    #[inline]
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        if (value as usize) < Registries::ENVIRONMENT_ATTRIBUTE.len() {
            Ok(EnvironmentAttribute(value))
        } else {
            Err(())
        }
    }
}
