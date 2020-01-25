use specs::prelude::*;
use specs::Component;

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(DenseVecStorage)]
#[allow(dead_code)]
pub enum Tile {
    Ground,
    Water,
    AirBlock,
    SpawnPoint(f32),
}

impl Default for Tile {
    fn default() -> Self {
        Self::Ground
    }
}