use specs::prelude::*;
use specs::{System};

use rltk::VirtualKeyCode;

use crate::components::Player;
use crate::components::Position;
use crate::consts;

pub struct PlayerMovementSystem {

}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        Read<'a, Option<VirtualKeyCode>>
    );

    fn run(&mut self, (players, mut positions, optkey): Self::SystemData) {
        //todo!();

        if let Some(key) = *optkey {
            match key {
                VirtualKeyCode::Left => {
                    for (player, pos) in (&players, &mut positions).join() {
                        pos.x -= 1;
                        if pos.x < 0 {
                            pos.x = 0;
                        }
                        println!("Player {:?} is at {:?}", player, pos);
                    }
                },
                VirtualKeyCode::Right => {
                    for (player, pos) in (&players, &mut positions).join() {
                        pos.x += 1;
                        if pos.x > consts::WIDTH as i32 - 1 {
                            pos.x = consts::WIDTH as i32 -1;
                        }
                        println!("Player {:?} is at {:?}", player, pos);
                    }
                }
                _ => {}
            }
        }
    }
}