mod category;
pub(crate) mod sound_event;

pub use category::*;
pub use sound_event::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sound {
    event: SoundEvent,
    category: SoundCategory,
    volume: f32,
    pitch: f32,
    seed: Option<i64>,
}

impl Sound {
    pub const fn new(event: SoundEvent, category: SoundCategory) -> Self {
        Self {
            event,
            category,
            volume: 1.0,
            pitch: 1.0,
            seed: None,
        }
    }

    pub const fn event(&self) -> SoundEvent {
        self.event
    }

    pub const fn category(&self) -> SoundCategory {
        self.category
    }

    pub const fn volume(&self) -> f32 {
        self.volume
    }

    pub const fn pitch(&self) -> f32 {
        self.pitch
    }

    pub const fn seed(&self) -> Option<i64> {
        self.seed
    }

    pub const fn set_event(&mut self, event: SoundEvent) {
        self.event = event;
    }

    pub const fn set_category(&mut self, category: SoundCategory) {
        self.category = category;
    }

    pub const fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub const fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    pub const fn set_seed(&mut self, seed: i64) {
        self.seed = Some(seed);
    }

    pub(crate) fn seed_or_random(&self) -> i64 {
        self.seed.unwrap_or_else(rand::random)
    }
}

impl From<SoundEvent> for Sound {
    fn from(event: SoundEvent) -> Self {
        Self::new(event, SoundCategory::Master)
    }
}
