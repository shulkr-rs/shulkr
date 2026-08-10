use crate::entity::meta::{
    MetaAccessor, MetadataHolder,
    refs::interaction::{HEIGHT, RESPONSIVE, WIDTH},
};

pub struct InteractionMeta {
    holder: MetadataHolder,
}

impl InteractionMeta {
    pub fn get_width(&self) -> f32 {
        self.holder.get(WIDTH)
    }

    pub fn set_width(&self, value: f32) {
        self.holder.set(WIDTH, value);
    }

    pub fn get_height(&self) -> f32 {
        self.holder.get(HEIGHT)
    }

    pub fn set_height(&self, value: f32) {
        self.holder.set(HEIGHT, value);
    }

    pub fn is_responsive(&self) -> bool {
        self.holder.get(RESPONSIVE)
    }

    pub fn set_responsive(&self, value: bool) {
        self.holder.set(RESPONSIVE, value);
    }
}

impl MetaAccessor for InteractionMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}
