use crate::{
    auth::ResolvableProfile,
    entity::meta::{
        MetaAccessor, MetadataHolder,
        refs::mannequin::{IMMOVABLE, PROFILE, TEXT_BELOW},
    },
    text::TextComponent,
};

pub struct MannequinMeta {
    holder: MetadataHolder,
}

impl MannequinMeta {
    pub fn get_profile(&self) -> ResolvableProfile {
        self.holder.get(PROFILE)
    }

    pub fn set_profile(&self, value: ResolvableProfile) {
        self.holder.set(PROFILE, value);
    }

    pub fn is_immovable(&self) -> bool {
        self.holder.get(IMMOVABLE)
    }

    pub fn set_immovable(&self, value: bool) {
        self.holder.set(IMMOVABLE, value);
    }

    pub fn get_text_below(&self) -> Option<TextComponent> {
        self.holder.get(TEXT_BELOW)
    }

    pub fn set_text_below(&self, value: Option<TextComponent>) {
        self.holder.set(TEXT_BELOW, value);
    }
}

impl MetaAccessor for MannequinMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
