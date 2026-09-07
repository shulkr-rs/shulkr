use crate::{
    registry::{Id, Registries},
    util::Key,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundEvent(Id);

include!("../../generated/sound_events.rs");

pub struct SoundEventData;

impl Default for SoundEventData {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundEventData {
    pub const fn new() -> Self {
        Self
    }
}

impl SoundEvent {
    pub fn from_id(id: Id) -> Option<SoundEvent> {
        Self::try_from(id).ok()
    }

    pub fn from_key(key: Key) -> Option<SoundEvent> {
        Registries::SOUND_EVENT.by_key(&key).copied()
    }

    pub fn key(&self) -> Option<&'static Key> {
        Registries::SOUND_EVENT.key_of(self.0)
    }
}

impl From<SoundEvent> for Key {
    #[inline]
    fn from(sound_event: SoundEvent) -> Self {
        sound_event
            .key()
            .expect("a sound event is always registered")
            .clone()
    }
}

impl From<SoundEvent> for Id {
    #[inline]
    fn from(sound_event: SoundEvent) -> Self {
        sound_event.0
    }
}

impl TryFrom<Id> for SoundEvent {
    type Error = ();

    #[inline]
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        if (value as usize) < Registries::SOUND_EVENT.len() {
            Ok(SoundEvent(value))
        } else {
            Err(())
        }
    }
}
