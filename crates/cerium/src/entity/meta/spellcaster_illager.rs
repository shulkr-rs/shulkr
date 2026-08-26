use crate::entity::meta::{MetaAccessor, MetadataHolder, refs::spellcaster_illager::SPELL};

pub struct SpellcasterIllagerMeta {
    holder: MetadataHolder,
}

impl SpellcasterIllagerMeta {
    pub fn get_spell(&self) -> SpellcasterIllagerSpell {
        SpellcasterIllagerSpell::try_from(self.holder.get(SPELL))
            .unwrap_or(SpellcasterIllagerSpell::None)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellcasterIllagerSpell {
    None,
    SummonVex,
    Attack,
    Wololo,
    Dissapear,
    Blindness,
}

impl TryFrom<u8> for SpellcasterIllagerSpell {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::None,
            1 => Self::SummonVex,
            2 => Self::Attack,
            3 => Self::Wololo,
            4 => Self::Dissapear,
            5 => Self::Blindness,
            _ => return Err(()),
        })
    }
}
