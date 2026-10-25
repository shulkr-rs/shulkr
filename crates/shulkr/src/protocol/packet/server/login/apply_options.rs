use crate::protocol::{
    encode::{Encode, EncodeError, PacketWrite},
    packet::{Packet, ServerPacket},
};

/// Connection option flags shared by [`ApplyOptionsPacket`] and
/// [`LoginSuccessPacket`](super::LoginSuccessPacket).
pub struct ConnectionOptions;

impl ConnectionOptions {
    pub const ENCRYPTION: u8 = 1;
    pub const COMPRESSION: u8 = 2;
}

#[derive(Debug, Clone)]
pub struct ApplyOptionsPacket {
    pub options: u8,
}

impl Packet for ApplyOptionsPacket {}
impl ServerPacket for ApplyOptionsPacket {}

impl Encode for ApplyOptionsPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_u8(this.options)?;
        Ok(())
    }
}
