use crate::components::{ Position, Tile, Renderable, Player };
use specs::{World, WorldExt, world::Builder};

use crate::consts;

#[derive(Debug, Clone)]
pub struct TileMapEntry(Tile, Position);

#[derive(Debug, Clone)]
pub struct Level {
    pub map: Vec<TileMapEntry>
}

impl Level {
    pub fn bootstrap_world(&self, world: &mut World) {
        world.delete_all();

        for entry in self.map.iter() {
            match entry.0 {
                Tile::Ground => {
                    world.create_entity()
                        .with(Tile::Ground)
                        .with(entry.1.clone())
                        .with(Renderable {
                            glyph: 1,
                            fg: rltk::RGB::from_f32(1.0, 1.0, 1.0),
                            bg: rltk::RGB::from_f32(0.0, 0.0, 0.0)
                        })
                        .build();
                },
                _ => {}
            }
        }

        let spawn_points = self.map
            .iter()
            .filter(|entry| {
                match entry.0 {
                    Tile::SpawnPoint(_) => true,
                    _ => false
                }
            })
            .map(|entry| (entry.1, match entry.0 {
                Tile::SpawnPoint(prob) => prob,
                _ => 0.0f32
            }))
            .collect::<Vec<_>>();
        
        if spawn_points.len() > 0 {
            // TODO: select randomly, according to spawn point weight

            let sp = spawn_points[0];

            world.create_entity()
                .with(Player::default())
                .with(sp.0.clone())
                .with(Renderable {
                    glyph: 2,
                    fg: rltk::RGB::from_f32(1.0, 1.0, 1.0),
                    bg: rltk::RGB::from_f32(0.0, 0.0, 0.0)
                })
                .build();
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        let mut lvl = Self {
            map: Default::default()
        };

        for i in 0..consts::WIDTH {
            lvl.map.push(TileMapEntry(Tile::Ground, Position { x: i as i32, y: (consts::HEIGHT - 1) as i32 }));
            lvl.map.push(TileMapEntry(Tile::Ground, Position { x: i as i32, y: (consts::HEIGHT - 2) as i32}));
        }

        lvl.map.push(TileMapEntry(Tile::SpawnPoint(1.0), Position { x: 2i32, y: (consts::HEIGHT - 3) as i32 }));

        lvl
    }
}