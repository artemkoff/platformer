use rltk::Rltk;

mod consts;
mod state;
mod components;
mod systems;
mod system_runner;
mod level;

rltk::embedded_resource!(TILES, "../resources/tiles.png");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    rltk::link_resource!(TILES, "resources/tiles.png");

    let mut context = Rltk::init_simple8x8(
        consts::WIDTH,
        consts::HEIGHT,
        consts::TITLE, "resources");

    let tile_font = context.register_font(rltk::font::Font::load("resources/tiles.png", (16, 16)));
    context.register_console(
        rltk::SimpleConsole::init(consts::WIDTH, consts::HEIGHT, &context.backend),
        tile_font
    );

    let font = context.register_font(rltk::font::Font::load("resources/vga8x16.png", (8, 16)));
    context.register_console(
        rltk::SparseConsole::init(consts::WIDTH, consts::HEIGHT / 2, &context.backend),
        font,
    );

    let mut state = state::State::default();
    state.load_default_level();

    rltk::main_loop(context, state);

    Ok(())
}
