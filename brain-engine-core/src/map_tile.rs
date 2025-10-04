use std::fmt;

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
    fn test_tile_values() {
        // Test that enum variants have correct bit values
        assert_eq!(MapTile::N as u8, Direction::North as u8);
        assert_eq!(MapTile::E as u8, Direction::East as u8);
        assert_eq!(MapTile::S as u8, Direction::South as u8);
        assert_eq!(MapTile::W as u8, Direction::West as u8);

        // Test double exits
        assert_eq!(
            MapTile::NE as u8,
            Direction::North as u8 | Direction::East as u8
        );
        assert_eq!(
            MapTile::NS as u8,
            Direction::North as u8 | Direction::South as u8
        );
        assert_eq!(
            MapTile::NW as u8,
            Direction::North as u8 | Direction::West as u8
        );
        assert_eq!(
            MapTile::ES as u8,
            Direction::East as u8 | Direction::South as u8
        );
        assert_eq!(
            MapTile::EW as u8,
            Direction::East as u8 | Direction::West as u8
        );
        assert_eq!(
            MapTile::SW as u8,
            Direction::South as u8 | Direction::West as u8
        );

        // Test triple exits
        assert_eq!(
            MapTile::NES as u8,
            Direction::North as u8 | Direction::East as u8 | Direction::South as u8
        );
        assert_eq!(
            MapTile::NEW as u8,
            Direction::North as u8 | Direction::East as u8 | Direction::West as u8
        );
        assert_eq!(
            MapTile::NWS as u8,
            Direction::North as u8 | Direction::West as u8 | Direction::South as u8
        );
        assert_eq!(
            MapTile::ESW as u8,
            Direction::East as u8 | Direction::South as u8 | Direction::West as u8
        );

        // Test all exits
        assert_eq!(
            MapTile::NESW as u8,
            Direction::North as u8
                | Direction::East as u8
                | Direction::South as u8
                | Direction::West as u8
        );
    }

    #[test]
    fn test_direction_constants() {
        // Test directional constants
        assert_eq!(Direction::North as u8, 1);
        assert_eq!(Direction::East as u8, 2);
        assert_eq!(Direction::South as u8, 4);
        assert_eq!(Direction::West as u8, 8);
    }

    #[test]
    fn test_double_exit_order_independence() {
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::North]),
            MapTile::from_directions(&[Direction::North, Direction::East])
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South, Direction::North]),
            MapTile::from_directions(&[Direction::North, Direction::South])
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::North]),
            MapTile::from_directions(&[Direction::North, Direction::West])
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South, Direction::East]),
            MapTile::from_directions(&[Direction::East, Direction::South])
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::East]),
            MapTile::from_directions(&[Direction::East, Direction::West])
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::South]),
            MapTile::from_directions(&[Direction::South, Direction::West])
        );
    }

    #[test]
    fn test_triple_exit_order_independence() {
        // NES group - all permutations should be equal
        let nes_perms = [
            MapTile::from_directions(&[Direction::North, Direction::East, Direction::South]),
            MapTile::from_directions(&[Direction::North, Direction::South, Direction::East]),
            MapTile::from_directions(&[Direction::East, Direction::North, Direction::South]),
            MapTile::from_directions(&[Direction::East, Direction::South, Direction::North]),
            MapTile::from_directions(&[Direction::South, Direction::North, Direction::East]),
            MapTile::from_directions(&[Direction::South, Direction::East, Direction::North]),
        ];
        for perm in &nes_perms {
            assert_eq!(*perm, Some(MapTile::NES));
        }

        // NEW group - all permutations should be equal
        let new_perms = [
            MapTile::from_directions(&[Direction::North, Direction::East, Direction::West]),
            MapTile::from_directions(&[Direction::North, Direction::West, Direction::East]),
            MapTile::from_directions(&[Direction::East, Direction::North, Direction::West]),
            MapTile::from_directions(&[Direction::East, Direction::West, Direction::North]),
            MapTile::from_directions(&[Direction::West, Direction::North, Direction::East]),
            MapTile::from_directions(&[Direction::West, Direction::East, Direction::North]),
        ];
        for perm in &new_perms {
            assert_eq!(*perm, Some(MapTile::NEW));
        }

        // NWS group - all permutations should be equal
        let nws_perms = [
            MapTile::from_directions(&[Direction::North, Direction::West, Direction::South]),
            MapTile::from_directions(&[Direction::North, Direction::South, Direction::West]),
            MapTile::from_directions(&[Direction::West, Direction::North, Direction::South]),
            MapTile::from_directions(&[Direction::West, Direction::South, Direction::North]),
            MapTile::from_directions(&[Direction::South, Direction::North, Direction::West]),
            MapTile::from_directions(&[Direction::South, Direction::West, Direction::North]),
        ];
        for perm in &nws_perms {
            assert_eq!(*perm, Some(MapTile::NWS));
        }

        // ESW group - all permutations should be equal
        let esw_perms = [
            MapTile::from_directions(&[Direction::East, Direction::South, Direction::West]),
            MapTile::from_directions(&[Direction::East, Direction::West, Direction::South]),
            MapTile::from_directions(&[Direction::South, Direction::East, Direction::West]),
            MapTile::from_directions(&[Direction::South, Direction::West, Direction::East]),
            MapTile::from_directions(&[Direction::West, Direction::East, Direction::South]),
            MapTile::from_directions(&[Direction::West, Direction::South, Direction::East]),
        ];
        for perm in &esw_perms {
            assert_eq!(*perm, Some(MapTile::ESW));
        }
    }

    #[test]
    fn test_four_exit_order_independence() {
        // Test various permutations of all four exits
        let four_exit_perms = [
            MapTile::from_directions(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]),
            MapTile::from_directions(&[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ]),
            MapTile::from_directions(&[
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ]),
            MapTile::from_directions(&[
                Direction::North,
                Direction::West,
                Direction::East,
                Direction::South,
            ]),
            MapTile::from_directions(&[
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ]),
            MapTile::from_directions(&[
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
            ]),
            MapTile::from_directions(&[
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::North,
            ]),
        ];

        for perm in &four_exit_perms {
            assert_eq!(*perm, Some(MapTile::NESW));
        }
    }

    #[test]
    fn test_from_dirs_single_exits() {
        assert_eq!(
            MapTile::from_directions(&[Direction::North]),
            Some(MapTile::N)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East]),
            Some(MapTile::E)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South]),
            Some(MapTile::S)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West]),
            Some(MapTile::W)
        );
    }

    #[test]
    fn test_from_dirs_double_exits() {
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::East]),
            Some(MapTile::NE)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::North]),
            Some(MapTile::NE)
        ); // Different order
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::South]),
            Some(MapTile::NS)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South, Direction::North]),
            Some(MapTile::NS)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::West]),
            Some(MapTile::NW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::North]),
            Some(MapTile::NW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::South]),
            Some(MapTile::ES)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South, Direction::East]),
            Some(MapTile::ES)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::West]),
            Some(MapTile::EW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::East]),
            Some(MapTile::EW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South, Direction::West]),
            Some(MapTile::SW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::South]),
            Some(MapTile::SW)
        );
    }

    #[test]
    fn test_from_dirs_triple_exits() {
        // Test all permutations of triple exits
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::East, Direction::South]),
            Some(MapTile::NES)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::South, Direction::East]),
            Some(MapTile::NES)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::North, Direction::South]),
            Some(MapTile::NES)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::South, Direction::North]),
            Some(MapTile::NES)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South, Direction::North, Direction::East]),
            Some(MapTile::NES)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South, Direction::East, Direction::North]),
            Some(MapTile::NES)
        );

        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::East, Direction::West]),
            Some(MapTile::NEW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::West, Direction::East]),
            Some(MapTile::NEW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::North, Direction::West]),
            Some(MapTile::NEW)
        );

        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::West, Direction::South]),
            Some(MapTile::NWS)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::South, Direction::West]),
            Some(MapTile::NWS)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::North, Direction::South]),
            Some(MapTile::NWS)
        );

        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::South, Direction::West]),
            Some(MapTile::ESW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::West, Direction::South]),
            Some(MapTile::ESW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::South, Direction::East, Direction::West]),
            Some(MapTile::ESW)
        );
    }

    #[test]
    fn test_from_dirs_all_exits() {
        // Test various permutations of all four exits
        assert_eq!(
            MapTile::from_directions(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West
            ]),
            Some(MapTile::NESW)
        );
        assert_eq!(
            MapTile::from_directions(&[
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West
            ]),
            Some(MapTile::NESW)
        );
        assert_eq!(
            MapTile::from_directions(&[
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::North
            ]),
            Some(MapTile::NESW)
        );
        assert_eq!(
            MapTile::from_directions(&[
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::East
            ]),
            Some(MapTile::NESW)
        );
    }

    #[test]
    fn test_from_dirs_duplicates() {
        // Test that duplicate directions are rejected
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::North]),
            None
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::East, Direction::North]),
            None
        );
        assert_eq!(
            MapTile::from_directions(&[
                Direction::North,
                Direction::North,
                Direction::East,
                Direction::South
            ]),
            None
        );
        assert_eq!(
            MapTile::from_directions(&[
                Direction::East,
                Direction::East,
                Direction::East,
                Direction::East
            ]),
            None
        );
    }

    #[test]
    fn test_from_dirs_empty_and_invalid() {
        assert_eq!(MapTile::from_directions(&[]), Some(MapTile::ZERO));

        // Test slices longer than 4 directions are rejected
        assert_eq!(
            MapTile::from_directions(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::North
            ]),
            None
        );
        assert_eq!(
            MapTile::from_directions(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::South
            ]),
            None
        );
    }

    #[test]
    fn test_direction_length_constraint() {
        // Test that we only accept 0-4 direction constants

        // Valid 1-direction slices
        assert_eq!(
            MapTile::from_directions(&[Direction::North]),
            Some(MapTile::N)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East]),
            Some(MapTile::E)
        );

        // Valid 2-direction slices
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::East]),
            Some(MapTile::NE)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::South]),
            Some(MapTile::SW)
        );

        // Valid 3-direction slices
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::East, Direction::West]),
            Some(MapTile::NEW)
        );
        assert_eq!(
            MapTile::from_directions(&[Direction::East, Direction::South, Direction::West]),
            Some(MapTile::ESW)
        );

        // Valid 4-direction slices
        assert_eq!(
            MapTile::from_directions(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West
            ]),
            Some(MapTile::NESW)
        );
        assert_eq!(
            MapTile::from_directions(&[
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::North
            ]),
            Some(MapTile::NESW)
        );

        // Invalid: 5+ direction slices (should return None)
        assert_eq!(
            MapTile::from_directions(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
                Direction::North
            ]),
            None
        );

        // No exits
        assert_eq!(MapTile::from_directions(&[]), Some(MapTile::ZERO));
    }

    #[test]
    fn test_dirs_method() {
        // Test single exits
        assert_eq!(MapTile::N.directions(), vec![Direction::North]);
        assert_eq!(MapTile::E.directions(), vec![Direction::East]);
        assert_eq!(MapTile::S.directions(), vec![Direction::South]);
        assert_eq!(MapTile::W.directions(), vec![Direction::West]);

        // Test double exits
        assert_eq!(
            MapTile::NE.directions(),
            vec![Direction::North, Direction::East]
        );
        assert_eq!(
            MapTile::NS.directions(),
            vec![Direction::North, Direction::South]
        );
        assert_eq!(
            MapTile::NW.directions(),
            vec![Direction::North, Direction::West]
        );
        assert_eq!(
            MapTile::ES.directions(),
            vec![Direction::East, Direction::South]
        );
        assert_eq!(
            MapTile::EW.directions(),
            vec![Direction::East, Direction::West]
        );
        assert_eq!(
            MapTile::SW.directions(),
            vec![Direction::South, Direction::West]
        );

        // Test triple exits
        assert_eq!(
            MapTile::NES.directions(),
            vec![Direction::North, Direction::East, Direction::South]
        );
        assert_eq!(
            MapTile::NEW.directions(),
            vec![Direction::North, Direction::East, Direction::West]
        );
        assert_eq!(
            MapTile::NWS.directions(),
            vec![Direction::North, Direction::South, Direction::West]
        );
        assert_eq!(
            MapTile::ESW.directions(),
            vec![Direction::East, Direction::South, Direction::West]
        );

        // Test all exits
        assert_eq!(
            MapTile::NESW.directions(),
            vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West
            ]
        );
    }

    #[test]
    fn test_tile_copy_clone() {
        let tile = MapTile::NE;
        let copied = tile;
        let cloned = tile.clone();

        assert_eq!(tile, copied);
        assert_eq!(tile, cloned);
        assert_eq!(copied, cloned);
    }

    #[test]
    fn test_tile_debug() {
        // Test that Debug is implemented
        let tile = MapTile::NE;
        let debug_str = format!("{:?}", tile);
        assert_eq!(debug_str, "NE");
    }

    #[test]
    fn test_tile_equality() {
        // Test PartialEq implementation
        assert_eq!(MapTile::N, MapTile::N);
        assert_ne!(MapTile::N, MapTile::E);

        // Test with different order constructions
        assert_eq!(
            MapTile::NE,
            MapTile::from_directions(&[Direction::East, Direction::North]).unwrap()
        );
        assert_eq!(
            MapTile::NESW,
            MapTile::from_directions(&[
                Direction::West,
                Direction::South,
                Direction::East,
                Direction::North
            ])
            .unwrap()
        );
    }

    #[test]
    fn test_tile_hash() {
        use std::collections::HashMap;

        // Test that Hash is implemented by using tiles as map keys
        let mut map = HashMap::new();
        map.insert(MapTile::N, "north");
        map.insert(MapTile::NE, "northeast");

        assert_eq!(map.get(&MapTile::N), Some(&"north"));
        assert_eq!(map.get(&MapTile::NE), Some(&"northeast"));
        assert_eq!(
            map.get(&MapTile::from_directions(&[Direction::East, Direction::North]).unwrap()),
            Some(&"northeast")
        ); // Order independence should work
    }

    #[test]
    fn test_roundtrip_from_dirs_and_dirs() {
        // Test that from_dirs and dirs are inverse operations
        let tiles = [
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

        for tile in tiles.iter() {
            let dirs_vec = tile.directions();
            let parsed_tile = MapTile::from_directions(&dirs_vec);
            assert_eq!(
                parsed_tile,
                Some(*tile),
                "Failed roundtrip for tile {:?}",
                tile
            );
        }
    }

    #[test]
    fn test_direction_methods() {
        // Test opposite directions
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::West.opposite(), Direction::East);

        // Test clockwise rotation
        assert_eq!(Direction::North.rotate_clockwise(), Direction::East);
        assert_eq!(Direction::East.rotate_clockwise(), Direction::South);
        assert_eq!(Direction::South.rotate_clockwise(), Direction::West);
        assert_eq!(Direction::West.rotate_clockwise(), Direction::North);

        // Test counter-clockwise rotation
        assert_eq!(Direction::North.rotate_counter_clockwise(), Direction::West);
        assert_eq!(Direction::East.rotate_counter_clockwise(), Direction::North);
        assert_eq!(Direction::South.rotate_counter_clockwise(), Direction::East);
        assert_eq!(Direction::West.rotate_counter_clockwise(), Direction::South);

        // Test double rotation equals opposite
        for direction in Direction::all() {
            assert_eq!(
                direction.rotate_clockwise().rotate_clockwise(),
                direction.opposite()
            );
            assert_eq!(
                direction
                    .rotate_counter_clockwise()
                    .rotate_counter_clockwise(),
                direction.opposite()
            );
        }

        // Test that clockwise and counter-clockwise are opposites
        for direction in Direction::all() {
            assert_eq!(
                direction.rotate_clockwise(),
                direction
                    .rotate_counter_clockwise()
                    .opposite()
                    .opposite()
                    .opposite()
            );
        }
    }

    #[test]
    fn test_user_example_order_independence() {
        // Test the specific example provided by the user
        assert_eq!(
            MapTile::from_directions(&[Direction::West, Direction::East]),
            MapTile::from_directions(&[Direction::East, Direction::West])
        );
    }

    #[test]
    fn test_addition_approach_correctness() {
        // Test that our addition approach produces correct results
        // North (1) + East (2) = 3 = NE
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::East]),
            Some(MapTile::NE)
        );

        // North (1) + South (4) + East (2) = 7 = NES
        assert_eq!(
            MapTile::from_directions(&[Direction::North, Direction::South, Direction::East]),
            Some(MapTile::NES)
        );

        // All directions: 1 + 2 + 4 + 8 = 15 = NESW
        assert_eq!(
            MapTile::from_directions(&[
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West
            ]),
            Some(MapTile::NESW)
        );

        // Verify the math manually
        let north_east_sum = Direction::North as u8 + Direction::East as u8;
        assert_eq!(north_east_sum, 3);
        assert_eq!(MapTile::NE as u8, 3);
    }

    #[test]
    fn test_direction_all() {
        let all_dirs = Direction::all();
        assert_eq!(all_dirs.len(), 4);
        assert_eq!(all_dirs[0], Direction::North);
        assert_eq!(all_dirs[1], Direction::East);
        assert_eq!(all_dirs[2], Direction::South);
        assert_eq!(all_dirs[3], Direction::West);
    }

    #[test]
    fn test_direction_display() {
        assert_eq!(format!("{}", Direction::North), "North");
        assert_eq!(format!("{}", Direction::East), "East");
        assert_eq!(format!("{}", Direction::South), "South");
        assert_eq!(format!("{}", Direction::West), "West");
    }

    #[test]
    fn test_bit_pattern_completeness() {
        // Test that all possible non-zero 4-bit combinations map to tiles
        for bits in 1..16u8 {
            let tile_opt = match bits {
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
            };

            if let Some(expected_tile) = tile_opt {
                assert_eq!(
                    expected_tile as u8, bits,
                    "Tile bit value mismatch for bits {}",
                    bits
                );
            }
        }
    }
}
