use bevy::math::{Vec2, Vec4};

#[derive(Clone, Debug, Default)]
pub struct Particle {
    pub health: ParticleHealth,
    pub velocity: Vec2,
    pub color: Vec4,
    pub affected_by_gravity: bool,
    pub updated: bool
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ParticleHealth {
    pub amount: i32,
}