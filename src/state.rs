use rltk::{Rltk, GameState, Console, RGB};
use specs::prelude::*;
use specs::{World, WorldExt};

use crate::components;
use crate::system_runner::SystemRunner;
use crate::level;

pub struct State {
    world: World,
    systems: SystemRunner,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {

        self.world.insert(ctx.key);
        self.world.insert(ctx.frame_time_ms);

        self.systems.run(&mut self.world);

        self.draw_world(ctx);
        self.draw_stat(ctx);
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            world: Self::create_world(),
            systems: SystemRunner::new()
        }
    }
}

impl State {
    fn draw_stat(&self, ctx: &mut Rltk) {
        ctx.set_active_console(2);
        ctx.cls();
        ctx.draw_box_double(
            59,
            0,
            20,
            3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );
        ctx.print_color(
            60,
            1,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &format!("FPS: {}", ctx.fps),
        );
        ctx.print_color(
            60,
            2,
            RGB::named(rltk::CYAN),
            RGB::named(rltk::BLACK),
            &format!("Frame: {} ms", ctx.frame_time_ms),
        );
    }

    fn draw_world(&self, ctx: &mut Rltk) {
        ctx.set_active_console(1);
        ctx.cls();

        let renderables = self.world.write_storage::<components::Renderable>();
        let positions = self.world.write_storage::<components::Position>();

        for (ren, pos) in (&renderables, &positions).join() {
            ctx.set(pos.x, pos.y, ren.fg, ren.bg, ren.glyph);
        }
    }

    fn create_world() -> World {
        let mut world = World::new();

        world.register::<components::Tile>();
        world.register::<components::Player>();
        world.register::<components::Position>();
        world.register::<components::Renderable>();

        world
    }

    pub fn load_default_level(&mut self) {
        level::Level::default().bootstrap_world(&mut self.world);
    }
}