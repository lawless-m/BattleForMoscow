# Battle for Moscow MCP Player

## Project Overview

An MCP (Model Context Protocol) server that enables LLMs to play Battle for Moscow. Lives in the same monorepo as the game engine.

## Contents

| File | Description | Start Here? |
|------|-------------|-------------|
| **CONTENTS.md** | This file — project overview | ✓ Read first |
| **SPEC.md** | Full technical specification — architecture, tools, implementation | ✓ Main reference |
| **NARRATOR_EXAMPLES.md** | Example output formats for state descriptions | Reference for narrator |

## Monorepo Structure

This crate lives alongside the existing game engine:

```
BattleForMoscow/
├── Cargo.toml              # workspace root
├── backend/                # game engine crate
│   ├── Cargo.toml
│   └── src/
├── static/                 # web UI
├── mcp-player/             # THIS CRATE
│   ├── Cargo.toml
│   └── src/
│       ├── game/           # game client and narrator
│       ├── mcp/            # MCP protocol implementation
│       └── text/           # text mode interface
├── data/
│   ├── map.json
│   └── units.json
├── bfm-project/            # game specification documents
├── mcp-player-spec/        # this specification
└── text-mode-spec/         # text mode specification
```

## Technology Stack

- **Language:** Rust
- **Protocol:** MCP over stdio
- **Game connection:** Can share types with `backend` crate, or use HTTP client

## Key Concepts

### MCP Tools

The server exposes tools that LLMs can call:

**State queries:**
- `get_situation()` — narrative overview
- `get_units()` — detailed unit list
- `get_threats()` — danger analysis

**Action queries:**
- `get_valid_moves(unit_id)` — where can this unit go?
- `preview_attack(attackers, defender)` — what would happen?
- `get_valid_attacks()` — all possible attacks

**Actions:**
- `move_unit(unit_id, to)`
- `declare_attacks(battles)`
- `resolve_next_battle()`
- `use_replacement(unit_id, hex)`
- `end_phase()`

**Reference:**
- `get_rules(topic)` — explain game rules

### Narrator

The narrator module converts JSON game state into readable text. See NARRATOR_EXAMPLES.md for the style and format to use.

Key principles:
- Lead with important information
- Include coordinates AND landmarks
- Explain modifiers, don't just list them
- Suggest next steps

## Implementation Order

1. Project setup, config loading
2. HTTP client for game API
3. Basic narrator (state → text)
4. MCP server skeleton
5. State query tools
6. Action query tools (preview_attack is key)
7. Action tools
8. Reference tools
9. Polish and error handling

## Testing

1. Unit tests for narrator output
2. Integration tests against running game server
3. Manual testing with Claude Desktop

## Usage

The mcp-player crate supports two modes:

```bash
# From repo root — start the game server (separate terminal)
cargo run -p backend

# Start in text mode (default, interactive terminal)
cargo run -p mcp-player
cargo run -p mcp-player -- --mode text

# Start in MCP mode (for AI assistants)
cargo run -p mcp-player -- --mode mcp

# Connect via Claude Desktop or other MCP client
```

For MCP mode, configure in Claude Desktop's MCP settings to point at this server with `--mode mcp`.

## Shared Types

The `mcp-player` crate can depend on `backend` to share type definitions:

```toml
# mcp-player/Cargo.toml
[dependencies]
backend = { path = "../backend" }
```

This avoids duplicating game state structs. The backend would need to expose its types publicly.
