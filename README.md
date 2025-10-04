# Brain Engine

## Getting Started

### Prerequisites

```shell
direnv allow
```

### Build Everything

```shell
cargo build --all
```

### Run the Game

```shell
cargo run -p brain-engine-bin
```

### Run Tests

```shell
# Test the library
cargo test -p brain-engine-core

# Test everything
cargo test --all
```

### Use the Library in Other Projects

Add to your `Cargo.toml`:

```toml
[dependencies]
brain-engine-core = { path = "../bevy-starter/brain-engine-core" }
```

Then in your code:

```rust
use brain_engine_core::{Map, TileGeneratorDefault};

let generator = TileGeneratorDefault::new();
let map = Map::new(10, generator);
```

## Development

### Working on the Library

The library is in `brain-engine-core/`. Changes here are immediately available to `brain-engine-bin` and any other local projects that depend on it.

### Working on the Game

The game is in `brain-engine-bin/`. It imports the library with:

```rust
use brain_engine_core::{Map, TileGeneratorDefault};
```

### Adding New Tile Generators

Implement the `TileGenerator` trait in `brain-engine-core`:

```rust
use brain_engine_core::TileGenerator;

pub struct MyGenerator;

impl TileGenerator for MyGenerator {
    fn tile_at(&self, tiles: &HashMap<IVec2, MapTile>, location: IVec2) -> MapTile {
        // Your generation logic
    }
}
```

## Assets

Game assets are located in `brain-engine-bin/assets/` directory at the workspace root.
