use crate::map_tile::{Direction, MapTile};
use bevy::prelude::*;
use rand::{Rng, rng};
use std::collections::HashMap;

#[derive(Resource)]
pub struct TileGeneratorDefault {
    pub tile_exit_probability: f64,
}

impl TileGeneratorDefault {
    pub fn new() -> Self {
        Self {
            tile_exit_probability: 0.5,
        }
    }
}

impl TileGenerator for TileGeneratorDefault {
    fn tile_at(&self, tiles: &HashMap<IVec2, MapTile>, location: IVec2) -> MapTile {
        let mut tile_exits: Vec<Direction> = Vec::new();
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let direction_vector = match direction {
                Direction::North => IVec2::new(0, 1),
                Direction::East => IVec2::new(1, 0),
                Direction::South => IVec2::new(0, -1),
                Direction::West => IVec2::new(-1, 0),
            };
            let neighbor = location + direction_vector;
            if let Some(tile) = tiles.get(&neighbor) {
                if tile.directions().contains(&direction.opposite()) {
                    tile_exits.push(direction);
                } else {
                    // no exit on neighbouring tile - so don't open an exit into a wall !
                }
            } else {
                // random chance we push direction to tile_exits based on configured probability
                if rng().random_bool(self.tile_exit_probability) {
                    tile_exits.push(direction);
                }
            }
        }
        MapTile::from_directions(&tile_exits).unwrap()
    }
}

pub trait TileGenerator {
    fn tile_at(&self, tiles: &HashMap<IVec2, MapTile>, location: IVec2) -> MapTile;
}
