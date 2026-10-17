use crate::{
    network::client::Connection,
    protocol::packet::{Tag, TagRegistry},
    registry::{Registry, registries},
};

pub fn tags<T: Send + Sync>(reg: &Registry<T>) -> TagRegistry {
    let packet_tags: Vec<Tag> = reg
        .tags()
        .iter()
        .map(|(name, tag)| Tag {
            tag_name: name.clone(),
            entries: tag
                .keys()
                .iter()
                .filter_map(|key| reg.get_id(key).map(i32::from))
                .collect(),
        })
        .collect();

    TagRegistry {
        registry: reg.key().to_key(),
        tags: packet_tags,
    }
}

#[rustfmt::skip]
pub(super) fn send_registries(c: &Connection) {

        use crate::protocol::packet::RegistryDataPacket;


    c.send_packet(&RegistryDataPacket::from(&registries::CAT_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::CAT_SOUND_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::CHICKEN_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::CHICKEN_SOUND_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::COW_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::COW_SOUND_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::FROG_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::PAINTING_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::PIG_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::PIG_SOUND_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::WOLF_SOUND_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::WOLF_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::ZOMBIE_NAUTILUS_VARIANT));
    c.send_packet(&RegistryDataPacket::from(&registries::DAMAGE_TYPE));
    c.send_packet(&RegistryDataPacket::from(&registries::BIOME));
    c.send_packet(&RegistryDataPacket::from(&registries::WORLD_CLOCK));
    c.send_packet(&RegistryDataPacket::from(&registries::TIMELINE));
    c.send_packet(&RegistryDataPacket::from(&registries::DIMENSION_TYPE));
    c.send_packet(&RegistryDataPacket::from(&registries::TRIM_MATERIAL));
    c.send_packet(&RegistryDataPacket::from(&registries::JUKEBOX_SONG));
    c.send_packet(&RegistryDataPacket::from(&registries::BANNER_PATTERN));
    c.send_packet(&RegistryDataPacket::from(&registries::INSTRUMENT));
    c.send_packet(&RegistryDataPacket::from(&registries::BLOCK_TRANSFORMER));
    c.send_packet(&RegistryDataPacket::from(&registries::CONTEXT_FLOAT_PROVIDER));
    c.send_packet(&RegistryDataPacket::from(&registries::CONTEXT_INT_PROVIDER));
    c.send_packet(&RegistryDataPacket::from(&registries::DECORATED_POT_PATTERN));
    c.send_packet(&RegistryDataPacket::from(&registries::PREDICATE));
}

pub(super) fn send_registry_tags(c: &Connection) {
    use crate::protocol::packet::UpdateTagsPacket;

    c.send_packet(&UpdateTagsPacket {
        registries: vec![
            tags(&registries::TIMELINE),
            tags(&registries::DAMAGE_TYPE),
            tags(&registries::BANNER_PATTERN),
            tags(&registries::INSTRUMENT),
            tags(&registries::PAINTING_VARIANT),
            tags(&registries::BLOCK),
        ],
    });
}
