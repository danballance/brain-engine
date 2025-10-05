# LLM Guidlines

## Project Commands
Never try to build the entire project, it's just too slow. I can run that check when you are done. Instead use the commands below:

```shell
# testing
cargo test --all-features
cargo test --all-features --package <package-name>  # Test specific package
cargo test --all-features <test-name>               # Run specific test
# linting and formatting
cargo clippy --all-targets --all-features
cargo fmt
# Fast compilation check
cargo check --all-features
```


## Coding Style
- Never use abbreviations for names, eg always `TilePosition`, not `TilePos`, `tile_position` not `tile_pos`
