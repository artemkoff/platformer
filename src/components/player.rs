use specs::prelude::*;
use specs::Component;

use crate::consts;

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Player {
    pub health: u32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            health: consts::DEFAULT_PLAYER_HEALTH,
        }
    }
}