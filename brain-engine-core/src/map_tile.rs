use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TileSet {
    Room,
    Corridor,
}

impl fmt::Display for TileSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileSet::Room => write!(f, "room"),
            TileSet::Corridor => write!(f, "corridor"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tile {
    pub tile_set: TileSet,
    pub map_tile: MapTile,
}

impl Tile {
    pub fn new(tile_set: TileSet, map_tile: MapTile) -> Self {
        Self { tile_set, map_tile }
    }

    pub fn directions(&self) -> Vec<Direction> {
        self.map_tile.directions()
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North = 1,
    East = 2,
    South = 4,
    West = 8,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}

impl Direction {
    pub const fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub const fn rotate_clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub const fn rotate_counter_clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub const fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MapTile {
    ZERO = 0 as u8, // 0
    // single exit (4)
    N = Direction::North as u8, // 1
    E = Direction::East as u8,  // 2
    S = Direction::South as u8, // 4
    W = Direction::West as u8,  // 8

    // double exit (6 unique)
    NE = Direction::North as u8 | Direction::East as u8, // 3
    NS = Direction::North as u8 | Direction::South as u8, // 5
    NW = Direction::North as u8 | Direction::West as u8, // 9
    ES = Direction::East as u8 | Direction::South as u8, // 6
    EW = Direction::East as u8 | Direction::West as u8,  // 10
    SW = Direction::South as u8 | Direction::West as u8, // 12

    // triple exit (4 unique)
    NES = Direction::North as u8 | Direction::East as u8 | Direction::South as u8, // 7
    NEW = Direction::North as u8 | Direction::East as u8 | Direction::West as u8,  // 11
    NWS = Direction::North as u8 | Direction::West as u8 | Direction::South as u8, // 13
    ESW = Direction::East as u8 | Direction::South as u8 | Direction::West as u8,  // 14

    // all exits (1 unique)
    NESW = Direction::North as u8
        | Direction::East as u8
        | Direction::South as u8
        | Direction::West as u8, // 15
}

impl MapTile {
    /// Build a Tile from 0-4 Direction enum values.
    /// Validates that only a maximum of one of each direction is allowed.
    /// Returns None if the slice is empty, longer than 4 elements, or contains duplicate directions.
    pub fn from_directions(directions: &[Direction]) -> Option<MapTile> {
        // Only allow 0-4 directions
        if directions.len() > 4 {
            return None;
        }

        // Check for duplicates using HashSet
        use std::collections::HashSet;
        let unique_dirs: HashSet<_> = directions.iter().collect();
        if unique_dirs.len() != directions.len() {
            return None;
        }

        // Sum the direction values (works because they're powers of 2)
        let sum: u8 = directions.iter().map(|&dir| dir as u8).sum();

        match sum {
            0 => Some(MapTile::ZERO),
            1 => Some(MapTile::N),
            2 => Some(MapTile::E),
            3 => Some(MapTile::NE),
            4 => Some(MapTile::S),
            5 => Some(MapTile::NS),
            6 => Some(MapTile::ES),
            7 => Some(MapTile::NES),
            8 => Some(MapTile::W),
            9 => Some(MapTile::NW),
            10 => Some(MapTile::EW),
            11 => Some(MapTile::NEW),
            12 => Some(MapTile::SW),
            13 => Some(MapTile::NWS),
            14 => Some(MapTile::ESW),
            15 => Some(MapTile::NESW),
            _ => None,
        }
    }

    /// Return a Vec of Direction enum values representing this tile's exits in canonical NESW order.
    pub fn directions(self) -> Vec<Direction> {
        let mut dirs = Vec::new();
        let bits = self as u8;

        if bits & Direction::North as u8 != 0 {
            dirs.push(Direction::North);
        }
        if bits & Direction::East as u8 != 0 {
            dirs.push(Direction::East);
        }
        if bits & Direction::South as u8 != 0 {
            dirs.push(Direction::South);
        }
        if bits & Direction::West as u8 != 0 {
            dirs.push(Direction::West);
        }

        dirs
    }
}

impl fmt::Display for MapTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encoding = self
            .directions()
            .iter()
            .map(|dir| match dir {
                Direction::North => "N",
                Direction::East => "E",
                Direction::South => "S",
                Direction::West => "W",
            })
            .collect::<String>();
        let encoding = if encoding.is_empty() {
            "ZERO"
        } else {
            &encoding
        };
        write!(f, "{}", encoding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_directions_maps_known_tiles() {
        use Direction::*;

        let cases: &[(&[Direction], MapTile)] = &[
            (&[], MapTile::ZERO),
            (&[North], MapTile::N),
            (&[East], MapTile::E),
            (&[South, North], MapTile::NS),
            (&[East, North], MapTile::NE),
            (&[South, West], MapTile::SW),
            (&[North, East, South], MapTile::NES),
            (&[North, West, South], MapTile::NWS),
            (&[North, East, South, West], MapTile::NESW),
        ];

        for &(dirs, expected) in cases {
            assert_eq!(MapTile::from_directions(dirs), Some(expected));

            if dirs.len() > 1 {
                let mut reversed = dirs.to_vec();
                reversed.reverse();
                assert_eq!(MapTile::from_directions(&reversed), Some(expected));
            }
        }
    }

    #[test]
    fn from_directions_rejects_invalid_inputs() {
        use Direction::*;

        assert_eq!(MapTile::from_directions(&[North, North]), None);
        assert_eq!(MapTile::from_directions(&[North, East, North]), None);
        assert_eq!(
            MapTile::from_directions(&[North, East, South, West, North]),
            None
        );
    }

    #[test]
    fn directions_return_canonical_order() {
        use Direction::*;

        assert_eq!(MapTile::NE.directions(), vec![North, East]);
        assert_eq!(MapTile::ESW.directions(), vec![East, South, West]);
        assert_eq!(MapTile::NESW.directions(), vec![North, East, South, West]);
    }

    #[test]
    fn directions_and_from_directions_roundtrip() {
        let tiles = [
            MapTile::ZERO,
            MapTile::N,
            MapTile::E,
            MapTile::S,
            MapTile::W,
            MapTile::NE,
            MapTile::NS,
            MapTile::NW,
            MapTile::ES,
            MapTile::EW,
            MapTile::SW,
            MapTile::NES,
            MapTile::NEW,
            MapTile::NWS,
            MapTile::ESW,
            MapTile::NESW,
        ];

        for tile in tiles {
            assert_eq!(MapTile::from_directions(&tile.directions()), Some(tile));
        }
    }

    #[test]
    fn direction_rotation_relations_hold() {
        for direction in Direction::all() {
            assert_eq!(direction.opposite().opposite(), direction);
            assert_eq!(
                direction.rotate_clockwise().rotate_counter_clockwise(),
                direction
            );

            let mut rotated = direction;
            for _ in 0..4 {
                rotated = rotated.rotate_clockwise();
            }
            assert_eq!(rotated, direction);

            assert_eq!(
                direction.rotate_clockwise(),
                direction.rotate_counter_clockwise().opposite()
            );
        }
    }

    #[test]
    fn displays_are_readable() {
        assert_eq!(MapTile::ZERO.to_string(), "ZERO");
        assert_eq!(MapTile::NE.to_string(), "NE");
        assert_eq!(Direction::North.to_string(), "North");
    }

    #[test]
    fn tile_set_displays_correctly() {
        assert_eq!(TileSet::Room.to_string(), "room");
        assert_eq!(TileSet::Corridor.to_string(), "corridor");
    }

    #[test]
    fn tile_new_creates_correct_tile() {
        let tile = Tile::new(TileSet::Room, MapTile::NESW);
        assert_eq!(tile.tile_set, TileSet::Room);
        assert_eq!(tile.map_tile, MapTile::NESW);
    }

    #[test]
    fn tile_directions_returns_maptile_directions() {
        let room_tile = Tile::new(TileSet::Room, MapTile::NE);
        let corridor_tile = Tile::new(TileSet::Corridor, MapTile::ESW);

        assert_eq!(room_tile.directions(), vec![Direction::North, Direction::East]);
        assert_eq!(
            corridor_tile.directions(),
            vec![Direction::East, Direction::South, Direction::West]
        );
    }

    #[test]
    fn tile_can_be_cloned_and_copied() {
        let tile1 = Tile::new(TileSet::Corridor, MapTile::NS);
        let tile2 = tile1;
        let tile3 = tile1.clone();

        assert_eq!(tile1, tile2);
        assert_eq!(tile1, tile3);
        assert_eq!(tile1.tile_set, TileSet::Corridor);
        assert_eq!(tile1.map_tile, MapTile::NS);
    }
}
