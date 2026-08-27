use std::collections::HashSet;

use serde::Deserialize;

use crate::{
    network::client::Connection,
    protocol::packet::{Tag, TagRegistry},
    registry::{Registries, Registry, RegistryKey},
    util::{HashMap, Key},
};

#[derive(Debug, Deserialize)]
pub struct TagSection {
    pub values: Vec<String>,
}

fn resolve_tag(
    tag_name: &str,
    all_tags: &HashMap<String, TagSection>,
    output: &mut HashSet<String>,
) {
    let section = all_tags
        .get(tag_name)
        .unwrap_or_else(|| panic!("missing tag: {}", tag_name));

    for value in &section.values {
        if let Some(stripped) = value.strip_prefix('#') {
            resolve_tag(stripped, all_tags, output);
        } else {
            output.insert(value.clone());
        }
    }
}

fn load_tags(registry: &str) -> HashMap<String, Vec<String>> {
    let sections: HashMap<String, TagSection> = shulkr_data::load_json(&format!("tags/{registry}"))
        .into_iter()
        .map(|(name, section)| (format!("minecraft:{name}"), section))
        .collect();

    let mut result = HashMap::default();

    for tag_name in sections.keys() {
        let mut resolved = HashSet::new();
        resolve_tag(tag_name, &sections, &mut resolved);

        result.insert(tag_name.clone(), resolved.into_iter().collect());
    }

    result
}

pub fn tags<T>(registry: &'static str, reg: &Registry<T>) -> TagRegistry {
    let resolved_tags = load_tags(registry);

    let packet_tags: Vec<Tag> = resolved_tags
        .into_iter()
        .map(|(name, values)| Tag {
            tag_name: Key::of(name),
            entries: values
                .into_iter()
                .filter_map(|v| reg.get_id(RegistryKey::of(v)).map(|id| id as i32))
                .collect(),
        })
        .collect();

    TagRegistry {
        registry: Key::vanilla(registry),
        tags: packet_tags,
    }
}

#[rustfmt::skip]
pub(super) fn send_registries(c: &Connection, registries: &Registries) {
    use crate::protocol::packet::RegistryDataPacket;

    c.send_packet(&RegistryDataPacket::from(&registries.cat_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.cat_sound_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.chicken_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.chicken_sound_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.cow_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.cow_sound_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.frog_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.painting_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.pig_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.pig_sound_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.wolf_sound_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.wolf_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.zombie_nautilus_variant));
    c.send_packet(&RegistryDataPacket::from(&registries.damage_type));
    c.send_packet(&RegistryDataPacket::from(&*Registries::biomes()));
    c.send_packet(&RegistryDataPacket::from(&registries.world_clock));
    c.send_packet(&RegistryDataPacket::from(&registries.timeline));
    c.send_packet(&RegistryDataPacket::from(&registries.dimension_type));
    c.send_packet(&RegistryDataPacket::from(&registries.trim_material));
    c.send_packet(&RegistryDataPacket::from(&registries.jukebox_song));
    c.send_packet(&RegistryDataPacket::from(&registries.banner_pattern));
    c.send_packet(&RegistryDataPacket::from(&registries.instrument));
}

pub(super) fn send_registry_tags(c: &Connection, registries: &Registries) {
    use crate::protocol::packet::UpdateTagsPacket;

    c.send_packet(&UpdateTagsPacket {
        registries: vec![
            tags("timeline", &registries.timeline),
            tags("damage_type", &registries.damage_type),
            tags("banner_pattern", &registries.banner_pattern),
            tags("instrument", &registries.instrument),
            tags("block", Registries::BLOCK),
        ],
    });
}
