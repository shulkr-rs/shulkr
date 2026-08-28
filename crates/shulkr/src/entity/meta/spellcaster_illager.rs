use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::spellcaster_illager::SPELL};
use shulkr_macros::Enumeration;

pub struct SpellcasterIllagerMeta {
    holder: MetadataHolder,
}

impl SpellcasterIllagerMeta {
    pub fn get_spell(&self) -> SpellcasterIllagerSpell {
        SpellcasterIllagerSpell::try_from(i32::from(self.holder.get(SPELL))).unwrap_or_default()
    }

    pub fn set_spell(&self, value: SpellcasterIllagerSpell) {
        self.holder.set(SPELL, value as u8);
    }
}

impl MetaAccessor for SpellcasterIllagerMeta {
    fn new(holder: MetadataHolder) -> Self {
        Self { holder }
    }
}

#[derive(Enumeration, Default)]
pub enum SpellcasterIllagerSpell {
    #[default]
    None,
    SummonVex,
    Attack,
    Wololo,
    Dissapear,
    Blindness,
}
