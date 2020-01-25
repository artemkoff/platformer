use specs::prelude::*;
use specs::Component;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}