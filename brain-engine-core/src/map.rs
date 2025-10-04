use crate::map_tile::MapTile;
use crate::tile_generator::TileGenerator;

use bevy::prelude::*;
use itertools::iproduct;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Map<G: TileGenerator> {
    pub size: usize,
    pub x: usize,
    pub y: usize,
    pub tiles: HashMap<IVec2, MapTile>,
    pub generator: G,
}

impl<G: TileGenerator> Map<G> {
    pub fn new(size: usize, generator: G) -> Self {
        let mut map = Self {
            size,
            x: size,
            y: size,
            tiles: HashMap::new(),
            generator,
        };
        for (x, y) in iproduct!(0..map.x, 0..map.y) {
            let position = IVec2::new(x as i32, y as i32);
            let tile = map.generator.tile_at(&map.tiles, position);
            map.tiles.insert(position, tile);
        }
        map
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = (IVec2, String)> + '_ {
        iproduct!(0..self.x, 0..self.y).map(|(x, y)| {
            let position = IVec2::new(x as i32, y as i32);
            let tile_type = self.tiles.get(&position).unwrap();
            let texture_file_name = format!("map-{}-{}.png", *tile_type as u8, *tile_type);
            (position, texture_file_name)
        })
    }
}
