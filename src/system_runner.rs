use specs::{World, WorldExt, Dispatcher, DispatcherBuilder};

use crate::systems;

pub struct SystemRunner {
    dispatcher: Dispatcher<'static, 'static>,
}

impl SystemRunner {
    pub fn new() -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(systems::PlayerMovementSystem{}, "player_movement", &[])
            .build();
        
        Self { dispatcher }
    }

    pub fn run(&mut self, world: &mut World) {
        self.dispatcher.dispatch(world);
        world.maintain();
    }
}