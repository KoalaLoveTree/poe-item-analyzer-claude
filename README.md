# PoE Item Analyzer

A desktop application for analyzing Path of Exile timeless jewels, built with Rust and egui.

## Features

- ✅ **One-Click Data Download** - Automatically downloads and parses PoB timeless jewel data
- ✅ **Binary LUT Parser** - Parses Path of Building's zlib-compressed jewel data
- ✅ **Auto-Detection** - Finds existing data to skip re-downloads
- ✅ **Real-Time Progress** - Live progress bars and status updates
- 🚧 **Jewel Analysis** - Analyze jewels against ALL socket locations (coming soon)
- 🚧 **Value Ranking** - Rank jewels by user-defined valuable mods (coming soon)
- 🚧 **Stash Integration** - Fetch jewels from public stash tabs (coming soon)
- ✅ **Cross-Platform** - Works on Windows, Linux, and macOS

## Quick Start

```bash
# Clone the repository
git clone https://github.com/KoalaLoveTree/poe-item-analyzer-claude.git
cd poe-item-analyzer-claude

# Run the application
cargo run --release -p poe-item-analyzer-desktop
```

The app will:
1. Check for existing data in `/tmp/poe-item-analyzer-test/`
2. Auto-parse if data exists, or show download button
3. Click "🚀 Download & Parse Data" to download from PathOfBuilding
4. View parsed data (node indices, modifiers, jewel types)

## Project Structure

This is a Cargo workspace with three crates:

- **`core`** - Core business logic (no I/O)
  - Item models and traits
  - Analyzer framework
  - Scoring system
- **`api`** - API integration and I/O operations
  - PoB data downloader
  - Binary LUT parser (zlib decompression)
  - Lua file parser
  - GitHub integration
- **`desktop`** - GUI application using egui
  - Parser testing UI
  - Progress tracking
  - Results display

## Development Status

**Phase 1: Core Foundation** ✅ Complete
- [x] Cargo workspace setup
- [x] Core traits and models
- [x] TimelessJewel implementation
- [x] Basic analyzer structure
- [x] Data directory structure

**Phase 2: API Integration** ✅ Complete
- [x] GitHub API integration
- [x] Data manifest system
- [x] Update checker
- [x] Checksum validation

**Phase 3: Parser Implementation** ✅ Complete
- [x] PoB binary LUT parser with zlib decompression
- [x] Lua file parser (NodeIndexMapping, LegionPassives)
- [x] Automatic download system
- [x] Parser testing UI with progress bars
- [x] Auto-detection of existing data
- [x] Support for all jewel types (LethalPride, BrutalRestraint, ElegantHubris, MilitantFaith)

**Phase 4: Analysis Logic** 🚧 Next Up
- [ ] Timeless jewel analyzer implementation
- [ ] Scoring system for valuable mods
- [ ] Multi-socket analysis
- [ ] Ranking algorithm

**Phase 5: Main GUI** ⏳ Planned
- [ ] Mod selector UI with search
- [ ] Jewel analysis workflow
- [ ] Results display per socket
- [ ] Configuration management

## Current Capabilities

### Working Now ✅
- Download PoB timeless jewel data from GitHub
- Parse binary LUT data (zlib-compressed)
- Parse Lua metadata files
- Display parsed data:
  - ~4000+ node indices
  - ~200+ modifiers
  - 4 jewel types
  - Thousands of seeds per type
- Auto-detect and reuse existing data
- Real-time progress tracking

## Building & Development

```bash
# Check that everything compiles
cargo check

# Build in release mode
cargo build --release

# Run the desktop application
cargo run -p poe-item-analyzer-desktop

# Run with optimizations (faster)
cargo run --release -p poe-item-analyzer-desktop
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p poe-item-analyzer-api
cargo test -p poe-item-analyzer-core

# Run tests with output
cargo test -- --nocapture
```

## Data Files

The app downloads and caches data in `/tmp/poe-item-analyzer-test/` (or OS-specific temp directory):
- `NodeIndexMapping.lua` - Passive tree node mappings
- `LegionPassives.lua` - Modifier definitions
- `LethalPride.zip` - Lethal Pride seed data (zlib-compressed)
- `BrutalRestraint.zip` - Brutal Restraint seed data
- `ElegantHubris.zip` - Elegant Hubris seed data
- `MilitantFaith.zip` - Militant Faith seed data

Data is sourced from [PathOfBuilding](https://github.com/PathOfBuildingCommunity/PathOfBuilding/tree/master/src/Data/TimelessJewelData).

## Documentation

See [CLAUDE.md](./CLAUDE.md) for detailed project requirements and architecture.

## License

MIT OR Apache-2.0

## Technical Highlights

- **Async Downloads**: Multi-threaded downloads with dedicated tokio runtime
- **Zlib Decompression**: Efficient binary data parsing with flate2
- **Lua Parsing**: Parses PoB's Lua metadata files with mlua
- **Real-Time UI**: Async message passing with mpsc channels
- **Auto-Detection**: Smart caching to avoid unnecessary re-downloads
- **egui**: Immediate mode GUI for responsive interface

## Acknowledgments

- Timeless Jewel data from [PathOfBuilding Community](https://github.com/PathOfBuildingCommunity/PathOfBuilding)
- Built with [egui](https://github.com/emilk/egui)
- Inspired by [Regisle's TimelessJewelData](https://github.com/Regisle/TimelessJewelData)
