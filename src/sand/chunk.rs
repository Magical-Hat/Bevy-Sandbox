use bevy::prelude::Entity;
use super::particle::Particle;

const MAX_TICKS_BEFORE_SLEEP: u8 = 2;

#[derive(Clone, Debug)]
pub struct Chunk {
    width: usize,
    height: usize,
    pub local_position: (usize, usize),
    particles: Vec<Option<Particle>>,
    pub colliders: Vec<Entity>,
    strong_ticks_remaining: u8,
    weak_ticks_remaining: u8
}