use crate::{
    command::{arguments::Arg, exceptions::CommandSyntaxException, string_reader::StringReader},
    protocol::encode::{EncodeError, PacketWrite},
};

const FLAG_HAS_MIN: u8 = 0x01;
const FLAG_HAS_MAX: u8 = 0x02;

macro_rules! numeric_argument {
    (
        $name:ident, $wire:literal, $value:ty, $id:literal,
        read = $read:ident, write = $write:ident,
        too_low = $too_low:ident, too_high = $too_high:ident,
        examples = [$($example:literal),* $(,)?] $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            minimum: $value,
            maximum: $value,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    minimum: <$value>::MIN,
                    maximum: <$value>::MAX,
                }
            }

            pub fn at_least(minimum: $value) -> Self {
                Self {
                    minimum,
                    maximum: <$value>::MAX,
                }
            }

            pub fn between(minimum: $value, maximum: $value) -> Self {
                Self { minimum, maximum }
            }

            pub fn minimum(&self) -> $value {
                self.minimum
            }

            pub fn maximum(&self) -> $value {
                self.maximum
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Arg for $name {
            type Value = $value;
            const ID: i32 = $id;

            fn parse(&self, reader: &mut StringReader) -> Result<$value, CommandSyntaxException> {
                let start = reader.cursor();
                let value = reader.$read()?;
                if value < self.minimum {
                    reader.set_cursor(start);
                    return Err(CommandSyntaxException::$too_low(value, self.minimum)
                        .with_context(reader.string(), start));
                }
                if value > self.maximum {
                    reader.set_cursor(start);
                    return Err(CommandSyntaxException::$too_high(value, self.maximum)
                        .with_context(reader.string(), start));
                }
                Ok(value)
            }

            fn encode_properties<W: PacketWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
                let has_min = self.minimum != <$value>::MIN;
                let has_max = self.maximum != <$value>::MAX;

                let mut flags = 0u8;
                if has_min {
                    flags |= FLAG_HAS_MIN;
                }
                if has_max {
                    flags |= FLAG_HAS_MAX;
                }
                w.write_u8(flags)?;

                if has_min {
                    w.$write(self.minimum)?;
                }
                if has_max {
                    w.$write(self.maximum)?;
                }
                Ok(())
            }

            fn examples(&self) -> Vec<String> {
                vec![$($example.to_string()),*]
            }
        }

    };
}

numeric_argument!(
    FloatArg,
    "float",
    f32,
    1,
    read = read_float,
    write = write_f32,
    too_low = float_too_low,
    too_high = float_too_high,
    examples = ["0", "1.2", ".5", "-1", "-.5", "-1234.56"],
);

numeric_argument!(
    DoubleArg,
    "double",
    f64,
    2,
    read = read_double,
    write = write_f64,
    too_low = double_too_low,
    too_high = double_too_high,
    examples = ["0", "1.2", ".5", "-1", "-.5", "-1234.56"],
);

numeric_argument!(
    IntArg,
    "integer",
    i32,
    3,
    read = read_int,
    write = write_i32,
    too_low = int_too_low,
    too_high = int_too_high,
    examples = ["0", "123", "-123"],
);

numeric_argument!(
    LongArg,
    "long",
    i64,
    4,
    read = read_long,
    write = write_i64,
    too_low = long_too_low,
    too_high = long_too_high,
    examples = ["0", "123", "-123"],
);
