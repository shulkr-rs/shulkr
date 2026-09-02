use crate::{
    util::Key,
    world::attribute::{
        AttributeArgument, AttributeEntry, AttributeModifier, AttributeValueType,
        EnvironmentAttribute, modifier::EntrySeed,
    },
};
use indexmap::IndexMap;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

#[derive(Debug, Clone, Default)]
pub struct EnvironmentAttributeMap {
    entries: IndexMap<EnvironmentAttribute, AttributeEntry>,
}

impl EnvironmentAttributeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T>(&mut self, attribute: EnvironmentAttribute, value: T)
    where
        T: AttributeValueType,
    {
        self.entries
            .insert(attribute, AttributeEntry::of(value.into_value()));
    }

    pub fn with<T>(mut self, attribute: EnvironmentAttribute, value: T) -> Self
    where
        T: AttributeValueType,
    {
        self.set(attribute, value);
        self
    }

    pub fn modify(
        &mut self,
        attribute: EnvironmentAttribute,
        modifier: AttributeModifier,
        argument: AttributeArgument,
    ) {
        self.entries
            .insert(attribute, AttributeEntry::new(modifier, argument));
    }

    pub fn entry(&self, attribute: EnvironmentAttribute) -> Option<&AttributeEntry> {
        self.entries.get(&attribute)
    }

    pub fn get<T>(&self, attribute: EnvironmentAttribute) -> Option<&T>
    where
        T: AttributeValueType,
    {
        T::from_value(self.entry(attribute)?.value()?)
    }

    pub fn get_or_default<T>(&self, attribute: EnvironmentAttribute) -> Option<&T>
    where
        T: AttributeValueType,
    {
        self.get(attribute)
            .or_else(|| T::from_value(attribute.default_value()))
    }

    pub fn contains(&self, attribute: EnvironmentAttribute) -> bool {
        self.entries.contains_key(&attribute)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Serialize for EnvironmentAttributeMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.entries.len()))?;
        for (attribute, entry) in &self.entries {
            map.serialize_entry(&attribute.key().to_string(), entry)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for EnvironmentAttributeMap {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MapVisitor;

        impl<'de> Visitor<'de> for MapVisitor {
            type Value = EnvironmentAttributeMap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of environment attributes")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut entries = IndexMap::new();
                while let Some(key) = map.next_key::<String>()? {
                    let key = Key::of(key);
                    let attribute = EnvironmentAttribute::from_key(&key).ok_or_else(|| {
                        serde::de::Error::custom(format!("no such environment attribute: {key}"))
                    })?;

                    let entry = map.next_value_seed(EntrySeed(attribute.r#type()))?;
                    entries.insert(attribute, entry);
                }
                Ok(EnvironmentAttributeMap { entries })
            }
        }

        deserializer.deserialize_map(MapVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::attribute::MoonPhase;
    use std::borrow::Cow;

    #[test]
    fn round_trip() {
        let map = EnvironmentAttributeMap::new()
            .with(EnvironmentAttribute::SKY_COLOR, Cow::Borrowed("#7BA4FF"))
            .with(EnvironmentAttribute::FAST_LAVA, true)
            .with(EnvironmentAttribute::MOON_PHASE, MoonPhase::NewMoon)
            .with(EnvironmentAttribute::CLOUD_HEIGHT, 192.5);

        let json = serde_json::to_string(&map).unwrap();
        assert_eq!(
            json,
            r##"{"minecraft:visual/sky_color":"#7BA4FF","minecraft:gameplay/fast_lava":true,"minecraft:visual/moon_phase":"new_moon","minecraft:visual/cloud_height":192.5}"##
        );

        let map: EnvironmentAttributeMap = serde_json::from_str(&json).unwrap();
        assert_eq!(map.get(EnvironmentAttribute::FAST_LAVA), Some(&true));
        assert_eq!(map.get(EnvironmentAttribute::CLOUD_HEIGHT), Some(&192.5));
        assert_eq!(
            map.get_or_default(EnvironmentAttribute::MUSIC_VOLUME),
            Some(&1.0)
        );
    }
}
