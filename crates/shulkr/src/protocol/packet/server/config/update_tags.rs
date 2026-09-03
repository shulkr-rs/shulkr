use crate::{
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    util::Key,
};

#[derive(Debug, Clone)]
pub struct UpdateTagsPacket {
    pub registries: Vec<TagRegistry>,
}

impl Packet for UpdateTagsPacket {}
impl ServerPacket for UpdateTagsPacket {}

impl Encode for UpdateTagsPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_array(&this.registries, TagRegistry::encode)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TagRegistry {
    pub registry: Key,
    pub tags: Vec<Tag>,
}

impl Encode for TagRegistry {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_key(&this.registry)?;
        w.write_array(&this.tags, |w, tag| {
            w.write_key(&tag.tag_name)?;
            w.write_array(&tag.entries, |w, v| w.write_varint(*v))?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub tag_name: Key,
    pub entries: Vec<i32>,
}
