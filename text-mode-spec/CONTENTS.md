# Text Mode Addition for mcp-player

## What This Is

A spec for adding text-based terminal gameplay to the existing `mcp-player` crate. This doesn't replace the MCP functionality — it adds a second mode.

## Files

| File | Description |
|------|-------------|
| **CONTENTS.md** | This file |
| **TEXT_MODE.md** | Full spec — commands, parsing, REPL loop, code examples |

## Quick Summary

Add `--mode text` flag to mcp-player:

```bash
cargo run -p mcp-player -- --mode text
```

This gives you a terminal interface:

```
> sit
[game situation]

> moves XLVII  
[valid moves for XLVII Panzer]

> move XLVII 4,5
MOVED: XLVII Panzer → [4,5]

> end
Phase complete.
```

## Why

- Human-playable without web UI
- Any LLM can use it (not just Claude with MCP)
- Fair interface for LLM battle ladder
- Easier to test and debug

## Implementation

Reuses existing narrator and game client. Adds:
- Command parser (~100 lines)
- REPL loop (~50 lines)
- Phase prompts (adapt from narrator)

See TEXT_MODE.md for full details and code examples.
