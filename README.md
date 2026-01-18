# Battle for Moscow

A web-based implementation of Frank Chadwick's classic wargame covering Operation Typhoon (October-December 1941).

## Overview

This is a complete digital implementation of the 1986 board wargame, featuring:

- **Full game rules** - All combat, movement, and replacement mechanics
- **Web-based interface** - Play in your browser with an interactive hex map
- **Rust backend** - High-performance game engine with Axum web framework
- **RESTful API** - Complete API for all game operations
- **SVG hex map** - Beautiful, scalable hex grid rendering

## Project Structure

This is a Cargo workspace with two main crates:

```
BattleForMoscow/
├── backend/              # Game engine crate
│   ├── src/
│   │   ├── main.rs       # HTTP server entry point
│   │   ├── api.rs        # REST API endpoints
│   │   ├── hex.rs        # Hex geometry (axial coordinates)
│   │   ├── map.rs        # Map data structures
│   │   ├── unit.rs       # Unit definitions and state
│   │   ├── game_state.rs # Turn/phase management
│   │   ├── zoc.rs        # Zone of Control calculations
│   │   ├── movement.rs   # Movement validation and pathfinding
│   │   ├── combat.rs     # Combat Results Table implementation
│   │   ├── replacement.rs# Replacement logic
│   │   └── retreat.rs    # Retreat mechanics
│   └── Cargo.toml
├── mcp-player/           # MCP server for LLM gameplay
│   ├── src/
│   │   ├── main.rs       # Entry point with mode selection
│   │   ├── config.rs     # Configuration loading
│   │   ├── game/         # Game client and narrator
│   │   │   ├── client.rs # HTTP client for game API
│   │   │   └── narrator.rs # State to text conversion
│   │   ├── mcp/          # MCP protocol implementation
│   │   │   ├── server.rs # JSON-RPC server
│   │   │   └── tools.rs  # MCP tools
│   │   └── text/         # Text mode interface
│   │       ├── repl.rs   # REPL loop
│   │       ├── commands.rs # Command parsing
│   │       └── prompts.rs # Phase-specific prompts
│   └── Cargo.toml
├── data/
│   ├── units.json        # Unit roster (39 units)
│   └── map.json          # Map data (placeholder - needs real map)
├── static/
│   ├── index.html        # Main game UI
│   ├── style.css         # Styling
│   ├── hex.js            # Hex rendering utilities
│   └── game.js           # Game controller
├── bfm-project/          # Game specification documents
│   ├── SPEC.md           # Technical specification
│   ├── RULES.md          # Game rules
│   ├── UNITS.md          # Unit data
│   ├── MAPDATA.md        # Map transcription guide
│   └── CONTENTS.md       # Project overview
├── mcp-player-spec/      # MCP server specification
│   ├── SPEC.md           # MCP server specification
│   ├── CONTENTS.md       # MCP project overview
│   └── NARRATOR_EXAMPLES.md # Narrator formatting examples
├── text-mode-spec/       # Text mode specification
│   ├── TEXT_MODE.md      # Text mode interface spec
│   └── CONTENTS.md       # Text mode overview
├── config.example.toml   # MCP server configuration template
└── Cargo.toml            # Workspace manifest
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- A web browser

### Running the Game (Web UI)

1. **Build and run the backend server:**
   ```bash
   cargo run -p backend
   ```

2. **Open in browser:**
   Navigate to `http://127.0.0.1:3000`

3. **Play the game:**
   - Click "New Game" to start
   - Select units by clicking them
   - Valid moves are highlighted in green
   - Click a highlighted hex to move
   - Use "Advance Phase" to progress through turns

### Running the Player Interface (Terminal & AI)

The `mcp-player` crate provides two interfaces for playing Battle for Moscow:

**Text Mode (Default)** - Interactive terminal interface:
```bash
cargo run -p mcp-player          # Uses text mode by default
cargo run -p mcp-player -- --mode text
```

**MCP Mode** - For AI assistants like Claude:
```bash
cargo run -p mcp-player -- --mode mcp
```

#### Setting up MCP Mode:

1. **Configure the MCP server:**
   ```bash
   cp config.example.toml config.toml
   # Edit config.toml to set API URL and preferences
   ```

2. **Start the game backend:**
   ```bash
   cargo run -p backend
   ```

3. **In a separate terminal, start the MCP server:**
   ```bash
   cargo run -p mcp-player -- --mode mcp
   ```

4. **Connect Claude Desktop or another MCP client:**
   - Add MCP server configuration to your MCP client
   - Point it to the `mcp-player` binary with `--mode mcp`
   - The LLM can now play the game using natural language

**Available MCP Tools:**
- `get_situation` - Get high-level game state overview
- `get_units` - List units and their positions
- `get_threats` - Analyze tactical threats
- `get_valid_moves` - Check where a unit can move
- `preview_attack` - Calculate attack odds before committing
- `get_valid_attacks` - List all possible attacks
- `move_unit` - Move a unit to a new position
- `declare_attacks` - Declare combat attacks
- `resolve_next_battle` - Resolve pending battles
- `use_replacement` - Apply replacement points
- `end_phase` - Advance to the next phase
- `get_rules` - Explain game rules and mechanics

## Game Features

### Implemented

✅ **Core Engine:**
- Hex grid with axial coordinates
- Turn/phase management (7 turns × 8 phases)
- Unit state tracking (full/half/eliminated)

✅ **Movement:**
- Movement point system
- Terrain costs (clear = 1 MP, forest = 2 MP)
- ZOC stops movement
- German Panzer Movement phase
- Soviet Rail Movement phase
- Mud movement restrictions (turns 3-4)

✅ **Combat:**
- Complete Combat Results Table
- Odds calculation (1:1 through 6:1)
- Terrain modifiers (forest, fortifications, rivers, Moscow)
- Attack strength halving in mud
- Combat results (NE, DR, DRL, AL, DE, EX)

✅ **Replacements:**
- German: 1 replacement per turn
- Soviet: 5 replacements per turn (in Moscow only)
- Communication path tracing

✅ **Special Rules:**
- 1st Shock Army availability (turn 4)
- Moscow defense bonus
- Soviet fortifications

### To Be Completed

⏳ **Map Data:**
- Current map is a 4-hex placeholder
- Full map needs to be transcribed from PDF (see MAPDATA.md)
- Approximately 330 hexes to transcribe

⏳ **UI Enhancements:**
- Battle declaration interface
- Retreat path selection
- Replacement point application
- Victory condition display

## API Endpoints

### Game Management
- `GET /api/game` - Get current game state
- `POST /api/game/new` - Start new game
- `POST /api/game/advance-phase` - Advance to next phase

### Movement
- `POST /api/units/move` - Move a unit
- `GET /api/units/:id/valid-moves` - Get valid destinations

### Combat
- `POST /api/battle/declare` - Declare a battle
- `POST /api/battle/resolve` - Resolve pending battle

### Replacements
- `POST /api/replacement/apply` - Apply replacement
- `GET /api/replacement/valid-hexes` - Get valid placement hexes

### Retreats
- `POST /api/retreat/execute` - Execute retreat
- `GET /api/retreat/:id/valid-hexes` - Get valid retreat hexes

### Data
- `GET /api/map` - Get map data
- `GET /api/units` - Get unit definitions

## Technology Stack

- **Backend:** Rust + Axum + Tokio
- **Frontend:** Vanilla JavaScript + SVG
- **Data:** JSON
- **Architecture:** REST API

## Game Rules Summary

### Turn Structure
Each turn consists of 8 phases:
1. German Replacement
2. German Panzer Movement
3. German Combat
4. German Movement
5. Soviet Replacement
6. Soviet Rail Movement
7. Soviet Combat
8. Soviet Movement

### Victory Conditions
- Game ends after Turn 7
- German wins if they control Moscow
- Soviet wins if they retain Moscow

### Key Mechanics
- **ZOC:** Enemy-occupied hexes exert ZOC on all 6 neighbors, stopping movement
- **Combat:** Attacker totals strength, defender defends alone, odds determine CRT column
- **Terrain:** Forest and Moscow reduce attacker odds by 1 column each
- **Mud:** Turns 3-4, movement reduced to 1 MP, attacks at half strength

## Development

### Building
```bash
# Build all crates
cargo build --all

# Build specific crate
cargo build -p backend
cargo build -p mcp-player
```

### Testing
```bash
# Test all crates
cargo test --all

# Test specific crate
cargo test -p backend
cargo test -p mcp-player
```

### Running
```bash
# Run the web server
cargo run -p backend

# Run the MCP server
cargo run -p mcp-player
```

## Credits

Original game design by Frank Chadwick (1986)
Digital implementation following SPEC.md specification

## License

See LICENSE file
