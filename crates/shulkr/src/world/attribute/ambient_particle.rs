use crate::world::Particle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientParticle {
    pub particle: Particle,
    pub probability: f32,
}
