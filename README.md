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

```
BattleForMoscow/
├── src/
│   ├── main.rs           # Server entry point
│   ├── api.rs            # REST API endpoints
│   ├── hex.rs            # Hex geometry (axial coordinates)
│   ├── map.rs            # Map data structures
│   ├── unit.rs           # Unit definitions and state
│   ├── game_state.rs     # Turn/phase management
│   ├── zoc.rs            # Zone of Control calculations
│   ├── movement.rs       # Movement validation and pathfinding
│   ├── combat.rs         # Combat Results Table implementation
│   ├── replacement.rs    # Replacement logic
│   └── retreat.rs        # Retreat mechanics
├── data/
│   ├── units.json        # Unit roster (39 units)
│   └── map.json          # Map data (placeholder - needs real map)
├── static/
│   ├── index.html        # Main game UI
│   ├── style.css         # Styling
│   ├── hex.js            # Hex rendering utilities
│   └── game.js           # Game controller
└── bfm-project/          # Specification documents
    ├── SPEC.md           # Technical specification
    ├── RULES.md          # Game rules
    ├── UNITS.md          # Unit data
    ├── MAPDATA.md        # Map transcription guide
    └── CONTENTS.md       # Project overview
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- A web browser

### Running the Game

1. **Build and run the server:**
   ```bash
   cargo run
   ```

2. **Open in browser:**
   Navigate to `http://127.0.0.1:3000`

3. **Play the game:**
   - Click "New Game" to start
   - Select units by clicking them
   - Valid moves are highlighted in green
   - Click a highlighted hex to move
   - Use "Advance Phase" to progress through turns

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
cargo build
```

### Testing
```bash
cargo test
```

### Running
```bash
cargo run
```

## Credits

Original game design by Frank Chadwick (1986)
Digital implementation following SPEC.md specification

## License

See LICENSE file
