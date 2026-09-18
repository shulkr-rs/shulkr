use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    registry::Registry,
    util::Key,
};
use serde::{Serialize, de::DeserializeOwned};
use shulkr_nbt::NbtTag;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct RegistryDataPacket {
    pub registry_id: Key,
    pub entries: Vec<RegistryEntry>,
}

impl Packet for RegistryDataPacket {}
impl ServerPacket for RegistryDataPacket {}

impl Encode for RegistryDataPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_key(&this.registry_id)?;
        w.write_array(&this.entries, RegistryEntry::encode)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub entry_id: Key,
    pub data: Option<NbtTag>,
}

impl Encode for RegistryEntry {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_key(&this.entry_id)?;
        w.write_option(&this.data, |w, v| w.write_nbt_tag(v))?;
        Ok(())
    }
}

impl<T> From<&Registry<T>> for RegistryDataPacket
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn from(value: &Registry<T>) -> Self {
        let registry_id = value.key().to_key();
        RegistryDataPacket {
            registry_id,
            entries: value
                .iter()
                .map(|(key, v)| RegistryEntry {
                    entry_id: key.clone(),
                    // Most registry entries are objects (an NBT compound), but some are
                    // bare values (e.g. context float/int providers, or lists like
                    // block transformers), so the root tag type isn't fixed.
                    data: Some(v.serialize(shulkr_nbt::Serializer).unwrap()),
                })
                .collect(),
        }
    }
}
