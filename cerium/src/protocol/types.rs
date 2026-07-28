use crate::auth::Property;
use crate::inventory::Slot;
use crate::item::{AnyDataComponent, ComponentMap, DataComponent};
use crate::protocol::encode::PacketWrite;
use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    encode::{Encode, EncodeError},
};

impl Decode for Property {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            name: r.read_string()?,
            value: r.read_string()?,
            signature: r.read_option(|r| r.read_string())?,
        })
    }
}

impl Encode for Property {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_string(&this.name)?;
        w.write_string(&this.value)?;
        w.write_option(&this.signature, |buffer, value| buffer.write_string(value))?;
        Ok(())
    }
}

impl Decode for AnyDataComponent {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(*DataComponent::from_id(r.read_varint()?).unwrap())
    }
}

impl Encode for AnyDataComponent {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.id())?;
        Ok(())
    }
}

impl Decode for Slot {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let item_count = r.read_varint()?;
        let item_id = if item_count > 0 {
            Some(r.read_varint()?)
        } else {
            None
        };

        let mut to_add: ComponentMap = ComponentMap::with_hasher(Default::default());
        let mut to_remove: Vec<i32> = vec![];
        if item_count > 0 {
            let n1 = r.read_varint()?;
            let n2 = r.read_varint()?;

            for _ in 0..n1 {
                let component = AnyDataComponent::decode(r)?;

                let id = component.id();
                let value = component.decode_value(r)?;
                to_add.insert(id, value);
            }

            for _ in 0..n2 {
                to_remove.push(r.read_varint()?);
            }
        }

        Ok(Self {
            item_count,
            item_id,
            to_add,
            to_remove,
        })
    }
}

impl Encode for Slot {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.item_count)?;
        if this.item_count > 0 {
            w.write_varint(this.item_id.unwrap())?;

            w.write_varint(this.to_add.len() as i32)?;
            w.write_varint(this.to_remove.len() as i32)?;

            for (id, data) in &this.to_add {
                let component = DataComponent::from_id(*id).unwrap();
                w.write_varint(*id)?;
                component.encode_value(w, data)?;
            }

            for id in &this.to_remove {
                w.write_varint(*id)?;
            }
        }
        Ok(())
    }
}

/// `Vec3`'s low-precision codec (26.2): a shared integer `scale` (the
/// magnitude) plus three 15-bit per-axis fractions in `[-1, 1]`, packed
/// little-end-first as `u8 + u8 + u32` (+ optional `scale >> 2` varint).
/// Used by the serverbound `Interact` packet's target position and the
/// clientbound `SpawnEntity`/`SetEntityMotion` velocity fields.
pub fn read_lp_vec3<R: PacketRead>(r: &mut R) -> Result<(f64, f64, f64), DecodeError> {
    fn unpack(value: u64) -> f64 {
        let v = std::cmp::min(value & 32767, 32766);
        v as f64 * 2.0 / 32766.0 - 1.0
    }

    let lowest = r.read_u8()?;
    if lowest == 0 {
        return Ok((0., 0., 0.));
    }

    let middle = r.read_u8()? as u64;
    let highest = r.read_u32()? as u64;
    let buffer = (highest << 16) | (middle << 8) | (lowest as u64);

    let mut scale = (lowest & 0x03) as u64;
    if (lowest & 0x04) != 0 {
        scale |= (r.read_varint()? as u32 as u64) << 2;
    }

    let x = unpack(buffer >> 3) * scale as f64;
    let y = unpack(buffer >> 18) * scale as f64;
    let z = unpack(buffer >> 33) * scale as f64;

    Ok((x, y, z))
}

pub fn write_lp_vec3<W: PacketWrite>(w: &mut W, x: f64, y: f64, z: f64) -> Result<(), EncodeError> {
    let max = x.abs().max(y.abs()).max(z.abs());

    // Zero vector: a single `0` byte, matching the decoder's early-out.
    if max <= 0.0 {
        return w.write_u8(0);
    }

    // Shared magnitude: smallest integer that keeps every axis fraction in
    // `[-1, 1]`. At least 1 so a sub-block velocity stays representable.
    let scale = (max.ceil() as u64).max(1);

    let quant = |v: f64| -> u64 {
        let n = (v / scale as f64).clamp(-1.0, 1.0);
        let q = (((n + 1.0) / 2.0) * 32766.0).round() as i64;
        q.clamp(0, 32766) as u64
    };

    let continuation = scale > 0x03;
    let mut buffer: u64 = (quant(x) << 3) | (quant(y) << 18) | (quant(z) << 33) | (scale & 0x03);
    if continuation {
        buffer |= 0x04;
    }

    w.write_u8((buffer & 0xFF) as u8)?;
    w.write_u8(((buffer >> 8) & 0xFF) as u8)?;
    w.write_u32(((buffer >> 16) & 0xFFFF_FFFF) as u32)?;
    if continuation {
        w.write_varint((scale >> 2) as i32)?;
    }
    Ok(())
}

/// The `Slot` wire format used specifically by
/// [`crate::protocol::packet::client::play::set_creative_mode_slot::SetCreativeModeSlotPacket`]
/// (serverbound, i.e. client → server only).
///
/// Vanilla 26.2 `ServerboundSetCreativeModeSlotPacket.STREAM_CODEC` uses
/// `ItemStack.OPTIONAL_UNTRUSTED_STREAM_CODEC`, which — unlike every other
/// slot-carrying packet (`ItemStack.OPTIONAL_STREAM_CODEC`, backed by
/// `DataComponentPatch.STREAM_CODEC`) — wraps *each individual component's
/// value* in a `VarInt` byte-length prefix. This lets a malformed
/// client-sent creative-slot item be partially skipped instead of
/// desyncing the whole packet — but decoding it with the plain [`Slot`]
/// codec silently misaligns every byte after the first component. Only this
/// one packet needs the delimited form; every other use of [`Slot`]
/// (container clicks, window contents, …) is correctly served by the plain
/// impl above.
pub mod slot_delimited {
    use super::*;

    pub fn decode<R: PacketRead>(r: &mut R) -> Result<Slot, DecodeError> {
        let item_count = r.read_varint()?;
        let item_id = if item_count > 0 {
            Some(r.read_varint()?)
        } else {
            None
        };

        let mut to_add: ComponentMap = ComponentMap::with_hasher(Default::default());
        let mut to_remove: Vec<i32> = vec![];
        if item_count > 0 {
            let n1 = r.read_varint()?;
            let n2 = r.read_varint()?;

            for _ in 0..n1 {
                let component = AnyDataComponent::decode(r)?;
                let id = component.id();

                let len = r.read_varint()?;
                let bytes = r.read_bytes(len)?;
                let mut slice: &[u8] = &bytes;
                let value = component.decode_value(&mut slice)?;
                to_add.insert(id, value);
            }

            for _ in 0..n2 {
                to_remove.push(r.read_varint()?);
            }
        }

        Ok(Slot {
            item_count,
            item_id,
            to_add,
            to_remove,
        })
    }

    #[allow(dead_code)]
    pub fn encode<W: PacketWrite>(w: &mut W, this: &Slot) -> Result<(), EncodeError> {
        w.write_varint(this.item_count)?;
        if this.item_count > 0 {
            w.write_varint(this.item_id.unwrap())?;

            w.write_varint(this.to_add.len() as i32)?;
            w.write_varint(this.to_remove.len() as i32)?;

            for (id, data) in &this.to_add {
                let component = DataComponent::from_id(*id).unwrap();
                w.write_varint(*id)?;

                let mut buf: Vec<u8> = Vec::new();
                component.encode_value(&mut buf, data)?;
                w.write_varint(buf.len() as i32)?;
                for byte in &buf {
                    w.write_u8(*byte)?;
                }
            }

            for id in &this.to_remove {
                w.write_varint(*id)?;
            }
        }
        Ok(())
    }
}
