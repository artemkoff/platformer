use specs::prelude::*;
use specs::Component;
use rltk::RGB;

#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}