use crate::map_tile::{Direction, MapTile, Tile, TileSet};
use bevy::prelude::*;
use rand::{Rng, rng};
use std::collections::HashMap;

#[derive(Resource)]
pub struct TileGeneratorDefault {
    pub tile_exit_probability: f64,
    pub room_probability: f64,
}

impl TileGeneratorDefault {
    pub fn new() -> Self {
        Self {
            tile_exit_probability: 0.35,
            room_probability: 0.35,
        }
    }
}

impl TileGenerator for TileGeneratorDefault {
    fn tile_at(&self, tiles: &HashMap<IVec2, Tile>, location: IVec2) -> Tile {
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
                if tile.map_tile.directions().contains(&direction.opposite()) {
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
        let map_tile = MapTile::from_directions(&tile_exits).unwrap();

        // Randomly select room or corridor based on room_probability
        let tile_set = if rng().random_bool(self.room_probability) {
            TileSet::Room
        } else {
            TileSet::Corridor
        };

        Tile::new(tile_set, map_tile)
    }
}

pub trait TileGenerator {
    fn tile_at(&self, tiles: &HashMap<IVec2, Tile>, location: IVec2) -> Tile;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_generator_default_new_has_correct_defaults() {
        let generator = TileGeneratorDefault::new();
        assert_eq!(generator.tile_exit_probability, 0.5);
        assert_eq!(generator.room_probability, 0.5);
    }

    #[test]
    fn tile_generator_creates_tiles_with_room_or_corridor() {
        let generator = TileGeneratorDefault {
            tile_exit_probability: 0.5,
            room_probability: 0.5,
        };
        let tiles = HashMap::new();

        // Generate multiple tiles to check both room and corridor can be produced
        let mut has_room = false;
        let mut has_corridor = false;

        for x in 0..20 {
            for y in 0..20 {
                let tile = generator.tile_at(&tiles, IVec2::new(x, y));
                match tile.tile_set {
                    TileSet::Room => has_room = true,
                    TileSet::Corridor => has_corridor = true,
                }
                if has_room && has_corridor {
                    break;
                }
            }
            if has_room && has_corridor {
                break;
            }
        }

        // With probability 0.5 and 400 tiles, we should statistically get both
        assert!(has_room, "Should generate at least one room tile");
        assert!(has_corridor, "Should generate at least one corridor tile");
    }

    #[test]
    fn tile_generator_with_room_probability_one_only_creates_rooms() {
        let generator = TileGeneratorDefault {
            tile_exit_probability: 0.5,
            room_probability: 1.0,
        };
        let tiles = HashMap::new();

        for x in 0..10 {
            for y in 0..10 {
                let tile = generator.tile_at(&tiles, IVec2::new(x, y));
                assert_eq!(tile.tile_set, TileSet::Room);
            }
        }
    }

    #[test]
    fn tile_generator_with_room_probability_zero_only_creates_corridors() {
        let generator = TileGeneratorDefault {
            tile_exit_probability: 0.5,
            room_probability: 0.0,
        };
        let tiles = HashMap::new();

        for x in 0..10 {
            for y in 0..10 {
                let tile = generator.tile_at(&tiles, IVec2::new(x, y));
                assert_eq!(tile.tile_set, TileSet::Corridor);
            }
        }
    }

    #[test]
    fn tile_generator_respects_neighbor_exits() {
        let generator = TileGeneratorDefault {
            tile_exit_probability: 0.5,
            room_probability: 0.5,
        };
        let mut tiles = HashMap::new();

        // Create a tile with only an East exit at (0, 0)
        tiles.insert(IVec2::new(0, 0), Tile::new(TileSet::Room, MapTile::E));

        // Generate tile at (1, 0) - should have West exit to connect
        let tile = generator.tile_at(&tiles, IVec2::new(1, 0));
        assert!(tile.map_tile.directions().contains(&Direction::West));
    }
}
