use serde::{Deserialize, Serialize};

use crate::world::Particle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientParticle {
    pub particle: Particle,
    pub probability: f32,
}
