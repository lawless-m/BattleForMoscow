# Battle for Moscow — Text Mode Addition

## Overview

Add a text-based command interface to the existing mcp-player crate. This provides:

- Terminal gameplay for humans
- Fair interface for LLM battles (any model)
- Easier testing/debugging than MCP

## Running

```bash
# Text mode (new)
cargo run -p mcp-player -- --mode text

# MCP mode (existing)
cargo run -p mcp-player -- --mode mcp
```

Default to text mode if no flag provided.

## Architecture

The narrator and game logic already exist for MCP. Text mode adds:

1. **Command parser** — parse text commands into actions
2. **REPL loop** — read command, execute, print result, repeat
3. **Prompt formatter** — show current phase and available actions

```
src/
├── main.rs           # add --mode flag, dispatch to text or mcp
├── text/
│   ├── mod.rs
│   ├── repl.rs       # main loop
│   ├── commands.rs   # command parsing
│   └── prompts.rs    # phase-specific prompts
├── mcp/              # existing
├── game/             # existing (narrator, client)
└── config.rs         # existing
```

## Commands

All commands case-insensitive. Minimal typing for common actions.

### Information

| Command | Short | Description |
|---------|-------|-------------|
| `HELP` | `H` | List commands |
| `SITUATION` | `SIT` | Game overview |
| `UNITS [side]` | `U` | List units |
| `THREATS` | `T` | Danger analysis |
| `RULES <topic>` | `R` | Explain a rule |

### Queries

| Command | Short | Description |
|---------|-------|-------------|
| `MOVES <unit>` | `M` | Valid moves for unit |
| `ATTACKS` | `A` | All possible attacks |
| `PREVIEW ATTACK <def> WITH <atk...>` | `P` | Calculate odds |
| `REPLACEMENTS` | `REP` | Replacement options |

### Actions

| Command | Description |
|---------|-------------|
| `MOVE <unit> <q>,<r>` | Move unit to hex |
| `ATTACK <def> WITH <atk...>` | Declare attack |
| `DONE ATTACKS` | Finish declaring, start resolution |
| `RESOLVE` | Resolve next battle |
| `ADVANCE <unit>` | Advance after combat |
| `SKIP` | Skip advance |
| `REPLACE <unit> [<q>,<r>]` | Use replacement |
| `END` or `DONE` | End current phase |

### Game

| Command | Description |
|---------|-------------|
| `NEW` | New game |
| `QUIT` | Exit |

## Command Parsing

Simple approach — split on whitespace, match first word:

```rust
fn parse_command(input: &str) -> Result<Command, ParseError> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Err(ParseError::Empty);
    }
    
    match parts[0].to_uppercase().as_str() {
        "HELP" | "H" => Ok(Command::Help),
        "SITUATION" | "SIT" => Ok(Command::Situation),
        "UNITS" | "U" => {
            let side = parts.get(1).map(|s| parse_side(s)).transpose()?;
            Ok(Command::Units { side })
        }
        "MOVES" | "M" => {
            let unit_id = parts.get(1).ok_or(ParseError::MissingUnitId)?;
            Ok(Command::Moves { unit_id: unit_id.to_string() })
        }
        "MOVE" => {
            let unit_id = parts.get(1).ok_or(ParseError::MissingUnitId)?;
            let hex = parts.get(2).ok_or(ParseError::MissingHex)?;
            let (q, r) = parse_hex(hex)?;
            Ok(Command::Move { unit_id: unit_id.to_string(), to: (q, r) })
        }
        "ATTACK" => parse_attack(&parts),
        "PREVIEW" | "P" => parse_preview(&parts),
        // ... etc
        _ => Err(ParseError::UnknownCommand(parts[0].to_string()))
    }
}

fn parse_hex(s: &str) -> Result<(i32, i32), ParseError> {
    // Accept "5,3" or "5 3" or "[5,3]"
    let clean = s.trim_matches(|c| c == '[' || c == ']');
    let parts: Vec<&str> = clean.split(',').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidHex(s.to_string()));
    }
    let q = parts[0].trim().parse().map_err(|_| ParseError::InvalidHex(s.to_string()))?;
    let r = parts[1].trim().parse().map_err(|_| ParseError::InvalidHex(s.to_string()))?;
    Ok((q, r))
}

fn parse_attack(parts: &[&str]) -> Result<Command, ParseError> {
    // ATTACK <defender> WITH <attacker> [attacker...]
    let with_pos = parts.iter().position(|&p| p.eq_ignore_ascii_case("WITH"))
        .ok_or(ParseError::MissingWith)?;
    
    let defender = parts.get(1).ok_or(ParseError::MissingUnitId)?.to_string();
    let attackers: Vec<String> = parts[with_pos + 1..].iter()
        .map(|s| s.to_string())
        .collect();
    
    if attackers.is_empty() {
        return Err(ParseError::MissingAttackers);
    }
    
    Ok(Command::Attack { defender, attackers })
}
```

## REPL Loop

```rust
fn run_text_mode(config: &Config) -> Result<()> {
    let mut game = GameClient::new(&config.api_url)?;
    let narrator = Narrator::new();
    
    println!("BATTLE FOR MOSCOW");
    println!("Type HELP for commands.\n");
    
    // Show initial state
    print_phase_prompt(&game, &narrator)?;
    
    let mut rl = DefaultEditor::new()?;  // rustyline for history/editing
    
    loop {
        let prompt = format!("> ");
        match rl.readline(&prompt) {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                
                match parse_command(&line) {
                    Ok(cmd) => {
                        match execute_command(&mut game, &narrator, cmd) {
                            Ok(output) => {
                                println!("{}\n", output);
                                
                                // Show new phase prompt if phase changed
                                if output.phase_changed {
                                    print_phase_prompt(&game, &narrator)?;
                                }
                            }
                            Err(e) => println!("Error: {}\n", e),
                        }
                    }
                    Err(ParseError::Empty) => continue,
                    Err(e) => println!("Parse error: {}\n", e),
                }
            }
            Err(ReadlineError::Interrupted) => continue,  // Ctrl-C
            Err(ReadlineError::Eof) => break,             // Ctrl-D
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}
```

## Phase Prompts

Show context at the start of each phase:

```rust
fn print_phase_prompt(game: &GameClient, narrator: &Narrator) -> Result<()> {
    let state = game.get_state()?;
    
    println!("══════════════════════════════════════════════════════════════\n");
    
    // Phase header
    let mud = if state.turn == 3 || state.turn == 4 { " (MUD)" } else { "" };
    println!("TURN {} {} — {}\n", state.turn, mud, format_phase(&state.phase));
    
    // Phase-specific prompt
    match state.phase {
        Phase::GermanPanzerMovement => {
            println!("{}", narrator.panzer_movement_prompt(&state)?);
        }
        Phase::GermanCombat | Phase::SovietCombat => {
            println!("{}", narrator.combat_prompt(&state)?);
        }
        Phase::GermanMovement | Phase::SovietMovement => {
            println!("{}", narrator.movement_prompt(&state)?);
        }
        Phase::GermanReplacement | Phase::SovietReplacement => {
            println!("{}", narrator.replacement_prompt(&state)?);
        }
        Phase::SovietRailMovement => {
            println!("{}", narrator.rail_movement_prompt(&state)?);
        }
    }
    
    Ok(())
}
```

## Execute Commands

Map commands to existing game client / narrator methods:

```rust
fn execute_command(
    game: &mut GameClient, 
    narrator: &Narrator, 
    cmd: Command
) -> Result<CommandOutput> {
    match cmd {
        Command::Help => Ok(CommandOutput::text(HELP_TEXT)),
        
        Command::Situation => {
            let state = game.get_state()?;
            Ok(CommandOutput::text(narrator.situation(&state)?))
        }
        
        Command::Units { side } => {
            let state = game.get_state()?;
            Ok(CommandOutput::text(narrator.units(&state, side)?))
        }
        
        Command::Moves { unit_id } => {
            let moves = game.get_valid_moves(&unit_id)?;
            let state = game.get_state()?;
            Ok(CommandOutput::text(narrator.valid_moves(&state, &unit_id, &moves)?))
        }
        
        Command::Move { unit_id, to } => {
            let result = game.move_unit(&unit_id, to)?;
            Ok(CommandOutput::text(narrator.move_result(&result)?))
        }
        
        Command::Attack { defender, attackers } => {
            game.declare_attack(&attackers, &defender)?;
            let state = game.get_state()?;
            let preview = game.preview_attack(&attackers, &defender)?;
            Ok(CommandOutput::text(format!(
                "DECLARED: {} vs {} ({})\n\nMore attacks? Or DONE ATTACKS to resolve.",
                attackers.join(" + "), defender, preview.odds
            )))
        }
        
        Command::Preview { defender, attackers } => {
            let preview = game.preview_attack(&attackers, &defender)?;
            Ok(CommandOutput::text(narrator.attack_preview(&preview)?))
        }
        
        Command::DoneAttacks => {
            game.finalize_attacks()?;
            let state = game.get_state()?;
            let count = state.pending_battles.len();
            Ok(CommandOutput::text(format!(
                "{} battle(s) declared. Use RESOLVE to resolve each.",
                count
            )))
        }
        
        Command::Resolve => {
            let result = game.resolve_next_battle()?;
            let output = narrator.battle_result(&result)?;
            
            // Check if advance is available
            if result.can_advance {
                Ok(CommandOutput::text(format!(
                    "{}\n\nADVANCE <unit> or SKIP",
                    output
                )))
            } else {
                Ok(CommandOutput::text(output))
            }
        }
        
        Command::Advance { unit_id } => {
            let result = game.advance_unit(&unit_id)?;
            Ok(CommandOutput::text(narrator.advance_result(&result)?))
        }
        
        Command::Skip => {
            game.skip_advance()?;
            Ok(CommandOutput::text("Advance skipped."))
        }
        
        Command::End => {
            let old_phase = game.get_state()?.phase;
            game.end_phase()?;
            let new_state = game.get_state()?;
            
            Ok(CommandOutput {
                text: format!("Phase complete."),
                phase_changed: new_state.phase != old_phase,
            })
        }
        
        Command::Quit => {
            std::process::exit(0);
        }
        
        // ... etc
    }
}
```

## Dependencies

Add to `mcp-player/Cargo.toml`:

```toml
[dependencies]
rustyline = "14"  # line editing, history
```

## Testing

```bash
# Run text mode
cargo run -p mcp-player -- --mode text

# Should see:
# BATTLE FOR MOSCOW
# Type HELP for commands.
# 
# ══════════════════════════════════════════════════════════════
# 
# TURN 1 — German Panzer Movement Phase
# 
# [situation description]
# 
# > _
```

Test each command:
- `help` — shows command list
- `sit` — shows situation
- `u german` — lists German units
- `moves XLVII` — shows valid moves
- `move XLVII 4,5` — moves unit
- `end` — advances phase

## Future: LLM Orchestrator

Once text mode works, an orchestrator for LLM battles would:

1. Start a game
2. Format state as prompt for current player's LLM
3. Send prompt to LLM API (Qwen, Haiku, etc.)
4. Parse response as command
5. Execute command via text mode interface (or directly via game client)
6. Repeat until game over
7. Record result

This is a separate concern — text mode just needs to work reliably first.
