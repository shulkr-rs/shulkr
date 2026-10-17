use crate::{
    registry::{Id, registries},
    util::Key,
    world::block::Block,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Material(Id);

include!("../../generated/materials.rs");

pub struct MaterialData {
    pub block: Option<Block>,
}

impl MaterialData {
    pub const fn new(block: Option<Block>) -> Self {
        Self { block }
    }
}

impl Material {
    pub fn from_id(id: Id) -> Option<Material> {
        Self::try_from(id).ok()
    }

    pub fn from_key(key: Key) -> Option<Material> {
        registries::MATERIAL.by_key(&key).map(|entry| **entry)
    }

    /// Returns the vanilla default max stack size for this material.
    pub fn max_stack_size(&self) -> i32 {
        match *self {
            Material::ENDER_PEARL
            | Material::SNOWBALL
            | Material::EGG
            | Material::ENDER_EYE
            | Material::ARMOR_STAND
            | Material::HONEY_BOTTLE
            | Material::POTION => 16,
            _ => {
                let name = registries::MATERIAL
                    .key_of(Id::from(*self))
                    .map(Key::path)
                    .unwrap_or_default();
                if name.ends_with("disc_fragment")
                    || name.ends_with("sign")
                    || name.ends_with("banner")
                {
                    16
                } else if name.ends_with("bucket")
                    || name.ends_with("boat")
                    || name.ends_with("minecart")
                    || name.ends_with("stew")
                    || name.ends_with("bundle")
                    || name.ends_with("horse_armor")
                    || name.ends_with("sword")
                    || name.ends_with("pickaxe")
                    || name.ends_with("shovel")
                    || name.ends_with("hoe")
                    || name.ends_with("helmet")
                    || name.ends_with("chestplate")
                    || name.ends_with("leggings")
                    || name.ends_with("boots")
                    || name.ends_with("disc")
                    || name.ends_with("axe")
                {
                    1
                } else {
                    match *self {
                        Material::SHIELD
                        | Material::BOW
                        | Material::CROSSBOW
                        | Material::TRIDENT
                        | Material::MACE
                        | Material::ELYTRA
                        | Material::SADDLE
                        | Material::SPYGLASS
                        | Material::BRUSH
                        | Material::GOAT_HORN
                        | Material::TOTEM_OF_UNDYING
                        | Material::KNOWLEDGE_BOOK
                        | Material::ENCHANTED_BOOK
                        | Material::WRITTEN_BOOK
                        | Material::WRITABLE_BOOK
                        | Material::RECOVERY_COMPASS
                        | Material::FISHING_ROD
                        | Material::SHEARS
                        | Material::FLINT_AND_STEEL
                        | Material::DEBUG_STICK
                        | Material::CARROT_ON_A_STICK
                        | Material::WARPED_FUNGUS_ON_A_STICK
                        | Material::SPLASH_POTION
                        | Material::LINGERING_POTION
                        | Material::OMINOUS_BOTTLE
                        | Material::WOLF_ARMOR => 1,
                        _ => 64,
                    }
                }
            }
        }
    }
}

impl From<Material> for Id {
    #[inline]
    fn from(material: Material) -> Self {
        material.0
    }
}

impl TryFrom<Id> for Material {
    type Error = ();

    #[inline]
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        if (value as usize) < registries::MATERIAL.len() {
            Ok(Material(value))
        } else {
            Err(())
        }
    }
}
