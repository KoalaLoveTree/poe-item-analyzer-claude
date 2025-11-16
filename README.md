# PoE Item Analyzer

A desktop application for analyzing Path of Exile timeless jewels, built with Rust and egui.

## Features (v0.1 - In Development)

- Analyze timeless jewels against ALL possible jewel socket locations
- Rank jewels by value based on user-defined valuable mods
- Fetch jewels from public stash tabs or import from files
- Cross-platform support (Windows, Linux, macOS)
- In-app data management and updates

## Project Structure

This is a Cargo workspace with three crates:

- **`core`** - Core business logic (no I/O)
- **`api`** - API integration and I/O operations
- **`desktop`** - GUI application using egui

## Development Status

**Phase 1: Core Foundation** ✅ Complete
- [x] Cargo workspace setup
- [x] Core traits and models
- [x] TimelessJewel implementation
- [x] Basic analyzer structure
- [x] Data directory structure

**Phase 2: API Integration** 🚧 In Progress
- [ ] PoE API client
- [ ] Data downloader
- [ ] LUT parser

**Phase 3: Analysis Logic** ⏳ Planned
- [ ] Timeless jewel analyzer
- [ ] Scoring system
- [ ] Multi-socket analysis

**Phase 4: GUI** ⏳ Planned
- [ ] Basic UI
- [ ] Mod selector
- [ ] Results display

## Building

```bash
# Check that everything compiles
cargo check

# Build in release mode
cargo build --release

# Run the desktop application
cargo run --bin poe-item-analyzer
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p poe-item-analyzer-core
```

## Documentation

See [CLAUDE.md](./CLAUDE.md) for detailed project requirements and architecture.

## License

MIT OR Apache-2.0

## Acknowledgments

- Timeless Jewel data from [Regisle/TimelessJewelData](https://github.com/Regisle/TimelessJewelData)
- Built with [egui](https://github.com/emilk/egui)
