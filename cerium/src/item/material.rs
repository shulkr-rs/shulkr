use crate::{
    registry::{Id, Registries},
    util::Key,
    world::block::Block,
};

include!("../registry/generated/materials.rs");

pub struct MaterialData {
    pub block: Option<Block>,
}

impl Material {
    pub fn from_id(id: Id) -> Option<Material> {
        Self::try_from(id).ok()
    }

    pub fn from_key(key: Key) -> Option<Material> {
        Registries::MATERIAL.by_key(&key).copied()
    }

    /// Returns the vanilla default max stack size for this material.
    ///
    /// Most items stack to 64; tools, armor, buckets, boats, minecarts, etc.
    /// only stack to 1, and a handful of items (signs, banners, ender pearls,
    /// …) stack to 16.
    pub fn max_stack_size(&self) -> i32 {
        match *self {
            Material::EnderPearl
            | Material::Snowball
            | Material::Egg
            | Material::EnderEye
            | Material::ArmorStand
            | Material::HoneyBottle
            | Material::Potion => 16,
            _ => {
                let name = format!("{:?}", self);
                if name.ends_with("DiscFragment")
                    || name.ends_with("Sign")
                    || name.ends_with("Banner")
                {
                    16
                } else if name.ends_with("Bucket")
                    || name.ends_with("Boat")
                    || name.ends_with("Minecart")
                    || name.ends_with("Stew")
                    || name.ends_with("Bundle")
                    || name.ends_with("HorseArmor")
                    || name.ends_with("Sword")
                    || name.ends_with("Pickaxe")
                    || name.ends_with("Shovel")
                    || name.ends_with("Hoe")
                    || name.ends_with("Helmet")
                    || name.ends_with("Chestplate")
                    || name.ends_with("Leggings")
                    || name.ends_with("Boots")
                    || name.ends_with("Disc")
                    || name.ends_with("Axe")
                {
                    1
                } else {
                    match *self {
                        Material::Shield
                        | Material::Bow
                        | Material::Crossbow
                        | Material::Trident
                        | Material::Mace
                        | Material::Elytra
                        | Material::Saddle
                        | Material::Spyglass
                        | Material::Brush
                        | Material::GoatHorn
                        | Material::TotemOfUndying
                        | Material::KnowledgeBook
                        | Material::EnchantedBook
                        | Material::WrittenBook
                        | Material::WritableBook
                        | Material::RecoveryCompass
                        | Material::FishingRod
                        | Material::Shears
                        | Material::FlintAndSteel
                        | Material::DebugStick
                        | Material::CarrotOnAStick
                        | Material::WarpedFungusOnAStick
                        | Material::SplashPotion
                        | Material::LingeringPotion
                        | Material::OminousBottle
                        | Material::WolfArmor => 1,
                        _ => 64,
                    }
                }
            }
        }
    }
}

impl TryFrom<Id> for Material {
    type Error = ();

    #[inline]
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        Self::all().get(value as usize).copied().ok_or(())
    }
}
