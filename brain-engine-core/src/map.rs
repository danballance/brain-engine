use crate::map_tile::{Direction, MapTile};
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

    pub fn iterate_tiles(&self) -> impl Iterator<Item = (IVec2, String)> + '_ {
        iproduct!(0..self.x, 0..self.y).map(|(x, y)| {
            let position = IVec2::new(x as i32, y as i32);
            let tile_type = self.tiles.get(&position).unwrap();
            let texture_file_name = format!("map-{}-{}.png", *tile_type as u8, *tile_type);
            (position, texture_file_name)
        })
    }

    pub fn can_move(&self, from: IVec2, to: IVec2) -> bool {
        if from == to {
            return false;
        }

        let max_x = self.x as i32;
        let max_y = self.y as i32;
        if from.x < 0
            || from.y < 0
            || from.x >= max_x
            || from.y >= max_y
            || to.x < 0
            || to.y < 0
            || to.x >= max_x
            || to.y >= max_y
        {
            return false;
        }

        let delta = to - from;

        let direction = match (delta.x, delta.y) {
            (0, 1) => Direction::North,
            (1, 0) => Direction::East,
            (0, -1) => Direction::South,
            (-1, 0) => Direction::West,
            _ => return false,
        };

        let Some(from_tile) = self.tiles.get(&from) else {
            return false;
        };
        let Some(to_tile) = self.tiles.get(&to) else {
            return false;
        };

        from_tile.directions().contains(&direction)
            && to_tile.directions().contains(&direction.opposite())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile_generator::TileGenerator;

    struct StaticGenerator;

    impl TileGenerator for StaticGenerator {
        fn tile_at(
            &self,
            _tiles: &std::collections::HashMap<IVec2, MapTile>,
            _location: IVec2,
        ) -> MapTile {
            MapTile::NESW
        }
    }

    #[test]
    fn cannot_move_out_of_bounds() {
        let map = Map::new(2, StaticGenerator);

        assert!(!map.can_move(IVec2::new(0, 0), IVec2::new(2, 0)));
    }

    #[test]
    fn cannot_move_when_not_adjacent() {
        let map = Map::new(4, StaticGenerator);

        assert!(!map.can_move(IVec2::new(0, 0), IVec2::new(0, 2)));
    }

    #[test]
    fn cannot_move_without_bidirectional_exits() {
        let mut map = Map::new(3, StaticGenerator);
        map.tiles.insert(IVec2::new(0, 0), MapTile::E);
        map.tiles.insert(IVec2::new(1, 0), MapTile::N);

        assert!(!map.can_move(IVec2::new(0, 0), IVec2::new(1, 0)));
    }

    #[test]
    fn can_move_when_exits_align() {
        let mut map = Map::new(3, StaticGenerator);
        map.tiles.insert(IVec2::new(0, 0), MapTile::E);
        map.tiles.insert(IVec2::new(1, 0), MapTile::W);

        assert!(map.can_move(IVec2::new(0, 0), IVec2::new(1, 0)));
    }

    #[test]
    fn cannot_move_to_same_tile() {
        let map = Map::new(3, StaticGenerator);

        assert!(!map.can_move(IVec2::new(1, 1), IVec2::new(1, 1)));
    }
}
