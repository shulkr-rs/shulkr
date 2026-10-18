use crate::{
    command::{
        arguments::Arg,
        error::{self, Error},
        string_reader::StringReader,
    },
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
            min: $value,
            max: $value,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    min: <$value>::MIN,
                    max: <$value>::MAX,
                }
            }

            pub fn at_least(min: $value) -> Self {
                Self {
                    min,
                    max: <$value>::MAX,
                }
            }

            pub fn between(min: $value, max: $value) -> Self {
                Self { min, max }
            }

            pub fn min(&self) -> $value {
                self.min
            }

            pub fn max(&self) -> $value {
                self.max
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

            fn parse(&self, reader: &mut StringReader) -> Result<$value, Error> {
                let start = reader.cursor();
                let value = reader.$read()?;
                if value < self.min {
                    reader.set_cursor(start);
                    return Err(error::$too_low
                        .create([error::arg(self.min), error::arg(value)])
                        .with_context(reader.string(), start));
                }
                if value > self.max {
                    reader.set_cursor(start);
                    return Err(error::$too_high
                        .create([error::arg(self.max), error::arg(value)])
                        .with_context(reader.string(), start));
                }
                Ok(value)
            }

            fn encode_properties<W: PacketWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
                let has_min = self.min != <$value>::MIN;
                let has_max = self.max != <$value>::MAX;

                let mut flags = 0u8;
                if has_min {
                    flags |= FLAG_HAS_MIN;
                }
                if has_max {
                    flags |= FLAG_HAS_MAX;
                }
                w.write_u8(flags)?;

                if has_min {
                    w.$write(self.min)?;
                }
                if has_max {
                    w.$write(self.max)?;
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
    too_low = FLOAT_TOO_LOW,
    too_high = FLOAT_TOO_HIGH,
    examples = ["0", "1.2", ".5", "-1", "-.5", "-1234.56"],
);

numeric_argument!(
    DoubleArg,
    "double",
    f64,
    2,
    read = read_double,
    write = write_f64,
    too_low = DOUBLE_TOO_LOW,
    too_high = DOUBLE_TOO_HIGH,
    examples = ["0", "1.2", ".5", "-1", "-.5", "-1234.56"],
);

numeric_argument!(
    IntArg,
    "integer",
    i32,
    3,
    read = read_int,
    write = write_i32,
    too_low = INTEGER_TOO_LOW,
    too_high = INTEGER_TOO_HIGH,
    examples = ["0", "123", "-123"],
);

numeric_argument!(
    LongArg,
    "long",
    i64,
    4,
    read = read_long,
    write = write_i64,
    too_low = LONG_TOO_LOW,
    too_high = LONG_TOO_HIGH,
    examples = ["0", "123", "-123"],
);
