use crate::protocol::{
    decode::{Decode, DecodeError, PacketRead},
    packet::{ClientPacket, Packet},
};

#[derive(Debug, Clone)]
pub struct ClickContainerPacket {
    pub window_id: i32,
    pub state_id: i32,
    pub slot: i16,
    pub button: i8,
    pub mode: i32,
    pub changed_slots: Vec<ChangedSlot>,
    pub carried_item: HashedSlot,
}

impl Packet for ClickContainerPacket {}
impl ClientPacket for ClickContainerPacket {}

impl Decode for ClickContainerPacket {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            window_id:     r.read_varint()?,
            state_id:      r.read_varint()?,
            slot:          r.read_i16()?,
            button:        r.read_i8()?,
            mode:          r.read_varint()?,
            changed_slots: r.read_array(ChangedSlot::decode)?,
            carried_item:  HashedSlot::decode(r)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChangedSlot {
    pub slot_number: i16,
    pub slot_data: HashedSlot,
}

#[derive(Debug, Clone)]
pub struct HashedSlot {
    pub present: bool,
    pub item_id: Option<i32>,
    pub count: i32,
    pub components: HashedComponents,
}

#[derive(Debug, Clone, Default)]
pub struct HashedComponents {
    pub added: Vec<HashedComponent>,
    pub removed: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct HashedComponent {
    pub component_id: i32,
    pub hash: i32,
}

impl Decode for ChangedSlot {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        Ok(Self {
            slot_number: r.read_i16()?,
            slot_data:   HashedSlot::decode(r)?,
        })
    }
}

impl Decode for HashedSlot {
    #[rustfmt::skip]
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let present = r.read_bool()?;
        if !present {
            return Ok(Self {
                present,
                item_id: None,
                count: 0,
                components: HashedComponents::default(),
            });
        }

        Ok(Self {
            present,
            item_id:    Some(r.read_varint()?),
            count:      r.read_varint()?,
            components: HashedComponents::decode(r)?,
        })
    }
}

impl Decode for HashedComponents {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let added_len = r.read_varint()? as usize;
        let mut added = Vec::with_capacity(added_len);
        for _ in 0..added_len {
            added.push(HashedComponent {
                component_id: r.read_varint()?,
                hash: r.read_i32()?,
            });
        }

        let removed_len = r.read_varint()? as usize;
        let mut removed = Vec::with_capacity(removed_len);
        for _ in 0..removed_len {
            removed.push(r.read_varint()?);
        }

        Ok(Self { added, removed })
    }
}
