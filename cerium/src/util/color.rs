use std::borrow::Cow;

use serde::Deserializer;
use serde::de::{Error, Unexpected, Visitor};

pub fn rgb_to_hex(value: u32) -> String {
    format!("#{:06x}", value & 0x00ff_ffff)
}

pub fn argb_to_hex(value: u32) -> String {
    format!("#{:08x}", value)
}

struct ColorVisitor {
    with_alpha: bool,
}

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a hex color string or a packed integer color")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(value.to_owned())
    }

    fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(value)
    }

    fn visit_i64<E: Error>(self, value: i64) -> Result<Self::Value, E> {
        self.visit_u64(value as u32 as u64)
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
        let value = u32::try_from(value)
            .map_err(|_| E::invalid_value(Unexpected::Unsigned(value), &self))?;
        Ok(if self.with_alpha {
            argb_to_hex(value)
        } else {
            rgb_to_hex(value)
        })
    }

    fn visit_f64<E: Error>(self, value: f64) -> Result<Self::Value, E> {
        self.visit_i64(value as i64)
    }
}

pub fn to_hex_color<E: Error>(value: i64, with_alpha: bool) -> Result<Cow<'static, str>, E> {
    ColorVisitor { with_alpha }.visit_i64(value).map(Cow::Owned)
}

pub fn deserialize_color<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    deserializer.deserialize_any(ColorVisitor { with_alpha: false })
}

pub fn deserialize_optional_color<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    struct OptionalColor;

    impl<'de> Visitor<'de> for OptionalColor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an optional color")
        }

        fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            deserialize_color(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(OptionalColor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Deserialize)]
    struct Colors {
        #[serde(deserialize_with = "deserialize_color")]
        water_color: String,
        #[serde(default, deserialize_with = "deserialize_optional_color")]
        grass_color: Option<String>,
    }

    #[test]
    fn accepts_integers_and_strings() {
        let colors: Colors =
            serde_json::from_str(r##"{"water_color": 4159136, "grass_color": "#7ba4ff"}"##)
                .unwrap();
        assert_eq!(colors.water_color, "#3f76a0");
        assert_eq!(colors.grass_color.as_deref(), Some("#7ba4ff"));

        let colors: Colors = serde_json::from_str(r##"{"water_color": "#3f76e4"}"##).unwrap();
        assert_eq!(colors.water_color, "#3f76e4");
        assert_eq!(colors.grass_color, None);
    }

    #[test]
    fn formats_argb() {
        assert_eq!(argb_to_hex(0), "#00000000");
        assert_eq!(rgb_to_hex(0x0078_a7ff), "#78a7ff");
    }
}
