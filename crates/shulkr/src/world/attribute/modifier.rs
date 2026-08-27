use serde::de::{DeserializeSeed, MapAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};

use crate::util::to_hex_color;
use crate::world::attribute::attribute_type::ValueSeed;
use crate::world::attribute::{AttributeType, AttributeValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeModifier {
    Override,
    AlphaBlend,
    Add,
    Subtract,
    Multiply,
    BlendToGray,
    Minimum,
    Maximum,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AttributeArgument {
    Value(AttributeValue),
    Float(f32),
    FloatWithAlpha { value: f32, alpha: f32 },
    Boolean(bool),
    Color(String),
    BlendToGray { brightness: f32, factor: f32 },
}

#[derive(Debug, Clone)]
pub struct AttributeEntry {
    modifier: AttributeModifier,
    argument: AttributeArgument,
}

impl AttributeEntry {
    pub fn new(modifier: AttributeModifier, argument: AttributeArgument) -> Self {
        Self { modifier, argument }
    }

    pub fn of(value: AttributeValue) -> Self {
        Self {
            modifier: AttributeModifier::Override,
            argument: AttributeArgument::Value(value),
        }
    }

    pub fn modifier(&self) -> AttributeModifier {
        self.modifier
    }

    pub fn argument(&self) -> &AttributeArgument {
        &self.argument
    }

    pub fn value(&self) -> Option<&AttributeValue> {
        match (&self.modifier, &self.argument) {
            (AttributeModifier::Override, AttributeArgument::Value(value)) => Some(value),
            _ => None,
        }
    }
}

impl Serialize for AttributeEntry {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;

        match self.value() {
            Some(value) => value.serialize(serializer),
            None => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("modifier", &self.modifier)?;
                map.serialize_entry("argument", &self.argument)?;
                map.end()
            }
        }
    }
}

pub(crate) struct EntrySeed(pub(crate) AttributeType);

impl<'de> DeserializeSeed<'de> for EntrySeed {
    type Value = AttributeEntry;

    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        match self.0 {
            AttributeType::Boolean
            | AttributeType::Float
            | AttributeType::AngleDegrees
            | AttributeType::RgbColor
            | AttributeType::ArgbColor => deserializer.deserialize_any(EntryVisitor(self.0)),
            _ => Ok(AttributeEntry::of(
                ValueSeed(self.0).deserialize(deserializer)?,
            )),
        }
    }
}

struct EntryVisitor(AttributeType);

impl<'de> Visitor<'de> for EntryVisitor {
    type Value = AttributeEntry;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an environment attribute value or modifier")
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(AttributeEntry::of(AttributeValue::Boolean(value)))
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        match self.0 {
            AttributeType::RgbColor | AttributeType::ArgbColor => Ok(AttributeEntry::of(
                AttributeValue::Color(to_hex_color(value, self.0 == AttributeType::ArgbColor)?),
            )),
            _ => self.visit_f64(value as f64),
        }
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
        self.visit_i64(value as i64)
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
        Ok(AttributeEntry::of(AttributeValue::Float(value as f32)))
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(AttributeEntry::of(AttributeValue::Color(
            value.to_owned().into(),
        )))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut modifier = None;
        let mut argument = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "modifier" => modifier = Some(map.next_value::<AttributeModifier>()?),
                "argument" => argument = Some(map.next_value_seed(ArgumentSeed(self.0))?),
                _ => {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        let modifier = modifier.ok_or_else(|| serde::de::Error::missing_field("modifier"))?;
        let argument = argument.ok_or_else(|| serde::de::Error::missing_field("argument"))?;

        Ok(AttributeEntry::new(modifier, argument))
    }
}

struct ArgumentSeed(AttributeType);

impl<'de> DeserializeSeed<'de> for ArgumentSeed {
    type Value = AttributeArgument;

    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(ArgumentVisitor(self.0))
    }
}

struct ArgumentVisitor(AttributeType);

impl<'de> Visitor<'de> for ArgumentVisitor {
    type Value = AttributeArgument;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an environment attribute modifier argument")
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(AttributeArgument::Boolean(value))
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        match self.0 {
            AttributeType::RgbColor | AttributeType::ArgbColor => Ok(AttributeArgument::Color(
                to_hex_color::<E>(value, self.0 == AttributeType::ArgbColor)?.into_owned(),
            )),
            _ => self.visit_f64(value as f64),
        }
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
        self.visit_i64(value as i64)
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
        Ok(AttributeArgument::Float(value as f32))
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(AttributeArgument::Color(value.to_owned()))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut value = None;
        let mut alpha = None;
        let mut brightness = None;
        let mut factor = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "value" => value = Some(map.next_value::<f32>()?),
                "alpha" => alpha = Some(map.next_value::<f32>()?),
                "brightness" => brightness = Some(map.next_value::<f32>()?),
                "factor" => factor = Some(map.next_value::<f32>()?),
                _ => {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        if let (Some(brightness), Some(factor)) = (brightness, factor) {
            return Ok(AttributeArgument::BlendToGray { brightness, factor });
        }

        match value {
            Some(value) => Ok(AttributeArgument::FloatWithAlpha {
                value,
                alpha: alpha.unwrap_or(1.0),
            }),
            None => Err(serde::de::Error::missing_field("value")),
        }
    }
}
