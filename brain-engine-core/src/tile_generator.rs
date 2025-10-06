use crate::map_tile::{Direction, MapTile, Tile, TileSet};
use bevy::prelude::*;
use rand::{rng, rngs::StdRng, Rng, SeedableRng};
use std::{collections::HashMap, sync::Mutex};

enum RandomSource {
    Thread,
    Seeded(Mutex<StdRng>),
}

impl RandomSource {
    fn random_bool(&self, probability: f64) -> bool {
        match self {
            RandomSource::Thread => rng().random_bool(probability),
            RandomSource::Seeded(rng) => rng.lock().unwrap().random_bool(probability),
        }
    }
}

#[derive(Resource)]
pub struct TileGeneratorDefault {
    pub tile_exit_probability: f64,
    pub room_probability: f64,
    rng: RandomSource,
}

impl TileGeneratorDefault {
    pub fn new() -> Self {
        Self::new_with_rng(RandomSource::Thread)
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::new_with_rng(RandomSource::Seeded(Mutex::new(StdRng::seed_from_u64(
            seed,
        ))))
    }

    pub fn with_probabilities(tile_exit_probability: f64, room_probability: f64) -> Self {
        Self {
            tile_exit_probability,
            room_probability,
            rng: RandomSource::Thread,
        }
    }

    pub fn with_seed_and_probabilities(
        seed: u64,
        tile_exit_probability: f64,
        room_probability: f64,
    ) -> Self {
        Self {
            tile_exit_probability,
            room_probability,
            rng: RandomSource::Seeded(Mutex::new(StdRng::seed_from_u64(seed))),
        }
    }

    fn new_with_rng(rng: RandomSource) -> Self {
        Self {
            tile_exit_probability: 0.35,
            room_probability: 0.35,
            rng,
        }
    }

    fn random_bool(&self, probability: f64) -> bool {
        self.rng.random_bool(probability)
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
                if self.random_bool(self.tile_exit_probability) {
                    tile_exits.push(direction);
                }
            }
        }
        let map_tile = MapTile::from_directions(&tile_exits).unwrap();

        // Randomly select room or corridor based on room_probability
        let tile_set = if self.random_bool(self.room_probability) {
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
        assert_eq!(generator.tile_exit_probability, 0.35);
        assert_eq!(generator.room_probability, 0.35);
    }

    #[test]
    fn tile_generator_with_seed_is_reproducible() {
        let generator_a = TileGeneratorDefault::with_seed(42);
        let generator_b = TileGeneratorDefault::with_seed(42);
        let tiles = HashMap::new();

        let sample_locations = [IVec2::new(0, 0), IVec2::new(1, 2), IVec2::new(-3, 5)];

        for location in sample_locations {
            let tile_a = generator_a.tile_at(&tiles, location);
            let tile_b = generator_b.tile_at(&tiles, location);

            assert_eq!(tile_a.tile_set, tile_b.tile_set);
            assert_eq!(tile_a.map_tile, tile_b.map_tile);
        }
    }

    #[test]
    fn tile_generator_with_room_probability_one_only_creates_rooms() {
        let mut generator = TileGeneratorDefault::with_seed(7);
        generator.tile_exit_probability = 0.5;
        generator.room_probability = 1.0;
        let tiles = HashMap::new();

        let tile = generator.tile_at(&tiles, IVec2::new(0, 0));
        assert_eq!(tile.tile_set, TileSet::Room);
    }

    #[test]
    fn tile_generator_with_room_probability_zero_only_creates_corridors() {
        let mut generator = TileGeneratorDefault::with_seed(11);
        generator.tile_exit_probability = 0.5;
        generator.room_probability = 0.0;
        let tiles = HashMap::new();

        let tile = generator.tile_at(&tiles, IVec2::new(0, 0));
        assert_eq!(tile.tile_set, TileSet::Corridor);
    }

    #[test]
    fn tile_generator_respects_neighbor_exits() {
        let mut generator = TileGeneratorDefault::with_seed(99);
        generator.tile_exit_probability = 0.0;
        generator.room_probability = 1.0;
        let mut tiles = HashMap::new();

        // Create a tile with only an East exit at (0, 0)
        tiles.insert(IVec2::new(0, 0), Tile::new(TileSet::Room, MapTile::E));

        // Generate tile at (1, 0) - should have West exit to connect
        let tile = generator.tile_at(&tiles, IVec2::new(1, 0));
        assert!(tile.map_tile.directions().contains(&Direction::West));
    }
}
