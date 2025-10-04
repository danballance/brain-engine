//! Brain Engine Core - Map generation library
//!
//! This library provides map generation functionality with configurable tile generators.
//! It can be used standalone or integrated with Bevy game engine.

pub mod map;
pub mod map_tile;
pub mod screen;
pub mod tile_generator;

// Re-export commonly used types for convenience
pub use map::Map;
pub use map_tile::{Direction, MapTile};
pub use screen::Screen;
pub use tile_generator::{TileGenerator, TileGeneratorDefault};
