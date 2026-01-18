# Battle for Moscow MCP Server — Technical Specification

## 1. Overview

An MCP (Model Context Protocol) server that allows LLMs to play Battle for Moscow against a human opponent (or another LLM). Connects to the existing Battle for Moscow game engine via its REST API.

## 2. Goals

- Let any MCP-compatible LLM play as either side
- Provide clear, readable game state descriptions
- Support "what if" queries before committing to actions
- Handle the full game flow from setup to victory

## 3. Technology

| Component | Choice | Rationale |
|-----------|--------|-----------|
| MCP Server | Rust | Matches game engine, your preference |
| Protocol | MCP over stdio | Standard for Claude desktop etc. |
| Game connection | HTTP client | Calls existing REST API |
| Config | TOML or JSON | Game API endpoint, preferences |

## 4. Architecture

```
┌──────────────────┐      ┌──────────────────┐      ┌──────────────────┐
│  MCP Client      │      │  MCP Server      │      │  BfM Game        │
│  (Claude, etc)   │◄────►│  (this project)  │◄────►│  Engine API      │
│                  │ MCP  │                  │ HTTP │                  │
└──────────────────┘      └──────────────────┘      └──────────────────┘
```

The MCP server is stateless — all game state lives in the game engine. The server:
1. Receives tool calls from LLM
2. Translates to REST API calls
3. Formats responses as readable text
4. Returns to LLM

## 5. Configuration

```toml
[game]
api_url = "http://localhost:3000/api"

[player]
side = "soviet"  # or "german" or "both"

[display]
coordinate_format = "axial"  # or "original" for XXYY
verbosity = "normal"  # "brief" or "detailed"
```

## 6. MCP Tools

### 6.1 State Query Tools

#### get_situation

High-level overview of current game state.

**Parameters:** none

**Returns:** Narrative text describing:
- Current turn and phase
- Whose action it is
- Overall strategic situation
- Immediate threats/opportunities
- Victory status

**Example response:**
```
TURN 3 (Mud) — German Combat Phase

The German offensive has stalled in the autumn mud. Your panzers are
within striking distance of Moscow but all units move at half speed
and attack at half strength.

Moscow remains in Soviet hands. The Soviets have formed a defensive
line along the Oka river but the southern flank is weak.

5 German units can attack this phase. The best odds available are 3:1
against the Soviet 30th Army at Kaluga.

It is your turn to declare attacks.
```

---

#### get_units

List units for one or both sides.

**Parameters:**
- `side` (optional): "german", "soviet", or omit for both

**Returns:** Structured list with:
- Unit ID and type
- Current strength (full/half/eliminated)
- Position (hex coordinates + nearby landmark)
- Adjacent enemies
- Whether unit has acted this phase

**Example response:**
```
GERMAN UNITS:

Panzers:
- XLVII Panzer (strength 9, full) at [5,4] near Kaluga
  Adjacent to: Soviet 30th Army
  Can move: Yes (panzer phase complete, regular movement available)

- XLI Panzer (strength 12, full) at [7,3] south of Mozhaisk
  Adjacent to: Soviet 5th Army, Soviet 16th Army
  Can move: Yes

Infantry:
- VII Infantry (strength 7, full) at [6,4]
  Adjacent to: Soviet 30th Army
  Can move: Yes
[...]

ELIMINATED: XXXIV Infantry
```

---

#### get_threats

Analyse dangerous situations.

**Parameters:**
- `side` (optional): which side's perspective, defaults to current player

**Returns:** 
- Units at risk of being destroyed
- Enemy units that could advance dangerously
- Weak points in the line

**Example response:**
```
THREATS TO SOVIET POSITION:

Critical:
- 30th Army at Kaluga can be attacked at 4:1 odds
  If destroyed, German path to Moscow's southern approach opens

- Gap between 5th Army and 16th Army
  German XLVII Panzer could exploit in movement phase

Concerning:
- 43rd Army at half strength, in German ZOC
  Cannot retreat safely if attacked
```

---

### 6.2 Action Query Tools

#### get_valid_moves

List where a unit can move.

**Parameters:**
- `unit_id`: which unit

**Returns:**
- List of valid destination hexes
- Movement cost to reach each
- What stops further movement (ZOC, terrain, MP limit)
- Strategic notes (approaching Moscow, cutting rail, etc.)

**Example response:**
```
VALID MOVES FOR XLVII PANZER:

Current position: [5,4]
Movement allowance: 6 (but reduced to 1 for mud)

Can reach:
- [5,3]: Clear terrain. Would be adjacent to Soviet 5th Army.
- [4,4]: Forest. Moves away from front.
- [6,4]: Clear. Already occupied by VII Infantry — BLOCKED.
- [5,5]: Clear. Enters ZOC of Soviet 30th Army — movement would end here.

Note: Mud limits all movement to 1 hex this turn.
```

---

#### preview_attack

Calculate what would happen for a potential attack.

**Parameters:**
- `attackers`: array of unit IDs
- `defender`: unit ID

**Returns:**
- Attacker total strength (and mud adjustment if applicable)
- Defender strength
- Raw odds
- Terrain modifiers with explanations
- Final odds
- Possible outcomes with probabilities
- Risk assessment

**Example response:**
```
ATTACK PREVIEW:

Attackers: XLVII Panzer (9), VII Infantry (7)
Total strength: 16, halved for mud = 8

Defender: Soviet 30th Army (8)
Terrain: Clear (no modifier)
River: No attackers crossing river

Raw odds: 8 vs 8 = 1:1
Final odds: 1:1

POSSIBLE OUTCOMES:
- Roll 1-2 (33%): AL — one of your units takes a step loss
- Roll 3-4 (33%): NE — no effect
- Roll 5-6 (33%): DR — defender retreats 2 hexes

RISK ASSESSMENT:
Moderate risk. 1 in 3 chance of losing a step, 1 in 3 chance of
pushing them back. Given mud penalties, this is a reasonable attack
if you need to create space.

If you add XXIV Panzer (strength 12, halved to 6):
New total: 14 vs 8 = 1:1 (still, just barely)
Consider whether the marginal improvement is worth committing another unit.
```

---

#### get_valid_attacks

List all possible attacks this phase.

**Parameters:** none

**Returns:**
- All enemy units that can be attacked
- Which friendly units can participate in each
- Best achievable odds for each target

**Example response:**
```
POSSIBLE ATTACKS THIS PHASE:

1. Target: Soviet 30th Army at [6,5]
   Your adjacent units: XLVII Panzer, VII Infantry, V Infantry
   Best odds achievable: 2:1 (using all three)
   
2. Target: Soviet 5th Army at [7,4]
   Your adjacent units: XLI Panzer, VIII Infantry
   Best odds achievable: 2:1

3. Target: Soviet 16th Army at [8,4]
   Your adjacent units: XLI Panzer
   Best odds achievable: 1:1 (single unit attack)

Note: Each unit can only attack once. XLI Panzer is adjacent to both
5th and 16th Army — must choose one or the other.
```

---

#### get_valid_replacements

List replacement options.

**Parameters:** none

**Returns:**
- Replacements remaining
- Units that can be restored (half → full)
- Eliminated units that can be placed
- Valid placement hexes

**Example response:**
```
SOVIET REPLACEMENTS: 5 remaining this turn

Can restore to full strength:
- 5th Army at [7,4] (currently strength 4, would become 8)
- 43rd Army at [9,6] (currently strength 4, would become 8)

Can bring back from eliminated:
- 29th Army (would place at half strength = 4)
- 32nd Army (would place at half strength = 4)

Valid placement hexes for new units:
- East edge: [12,2], [12,3], [12,4], [12,5], [12,6]
- Moscow: [10,5] — always valid for Soviets
- Tula: [9,8] — friendly city in communication

1st Shock Army: Available (turn 4+). Strength 10/5.
```

---

### 6.3 Action Tools

#### move_unit

Move a unit to a new position.

**Parameters:**
- `unit_id`: which unit
- `to`: destination hex [q, r]

**Returns:**
- Success/failure
- New position
- What the unit is now adjacent to
- Remaining moves this phase (if any units haven't moved)

**Example response:**
```
MOVED: XLVII Panzer [5,4] → [5,3]

Now adjacent to: Soviet 5th Army
Movement for this unit complete (mud: 1 hex maximum).

12 units still to move this phase.
```

**Errors:**
- "Invalid move: destination not reachable"
- "Invalid move: hex occupied"
- "Invalid move: unit cannot move this phase"
- "Invalid move: unit already moved"

---

#### declare_attacks

Declare all attacks for the combat phase.

**Parameters:**
- `battles`: array of { attackers: [unit_ids], defender: unit_id }

**Returns:**
- Confirmation of declared battles
- Odds for each
- Ready to resolve

**Example request:**
```json
{
  "battles": [
    {"attackers": ["XLVII", "VII"], "defender": "30"},
    {"attackers": ["XLI"], "defender": "5"}
  ]
}
```

**Example response:**
```
ATTACKS DECLARED:

Battle 1: XLVII Panzer + VII Infantry vs Soviet 30th Army
  Odds: 2:1

Battle 2: XLI Panzer vs Soviet 5th Army
  Odds: 1:1

2 battles to resolve. Use resolve_next_battle to proceed.
```

**Errors:**
- "Invalid attack: [unit] not adjacent to defender"
- "Invalid attack: [unit] already assigned to another battle"
- "Invalid attack: [defender] already being attacked"

---

#### resolve_next_battle

Resolve the next pending battle (server rolls die).

**Parameters:** none (resolves in declaration order)

**Returns:**
- Which battle
- Die roll
- Result
- Consequences (losses, retreats)
- Remaining battles

**Example response:**
```
BATTLE RESOLVED:

XLVII Panzer + VII Infantry vs Soviet 30th Army
Odds: 2:1
Die roll: 5

RESULT: DR (Defender Retreat)

Soviet 30th Army retreats from [6,5].
Valid retreat hexes: [7,6], [8,5]
Retreated to: [7,6]

XLVII Panzer may advance into [6,5]. Use advance_unit to advance,
or proceed to next battle.

1 battle remaining.
```

---

#### advance_unit

Advance a unit into a vacated hex after combat.

**Parameters:**
- `unit_id`: which attacking unit (must have participated in last battle)

**Returns:**
- Confirmation
- New position

**Example response:**
```
ADVANCED: XLVII Panzer [5,4] → [6,5]

Now adjacent to: Soviet 43rd Army
```

---

#### skip_advance

Decline to advance after combat.

**Parameters:** none

**Returns:**
- Confirmation
- Next battle or phase status

---

#### use_replacement

Use a replacement to restore or place a unit.

**Parameters:**
- `unit_id`: which unit to restore or place
- `hex` (optional): where to place if bringing back eliminated unit

**Returns:**
- Confirmation
- New unit state
- Replacements remaining

**Example response:**
```
REPLACEMENT USED:

5th Army restored to full strength (4 → 8)
Position: [7,4]

Soviet replacements remaining: 4
```

---

#### end_phase

End the current phase and advance to next.

**Parameters:** none

**Returns:**
- New phase
- Whose turn
- Brief situation update

**Example response:**
```
PHASE COMPLETE: German Combat Phase ended.

Now: German Movement Phase
All German units may now move (panzers may move again).

The attack on 30th Army opened a gap in the Soviet line.
Consider pushing XLVII Panzer toward Moscow.
```

---

### 6.4 Reference Tools

#### get_rules

Explain a game rule or concept.

**Parameters:**
- `topic`: what to explain

**Topics:**
- "zoc" / "zone of control"
- "combat" / "attacking"
- "odds" / "combat odds"
- "crt" / "combat results"
- "retreat"
- "movement"
- "mud"
- "replacements"
- "rail" / "rail movement"
- "victory"
- "terrain"
- "stacking"

**Example response (topic: "zoc"):**
```
ZONE OF CONTROL (ZOC)

Every unit exerts a Zone of Control into the six adjacent hexes.

Effects on MOVEMENT:
- Entering an enemy ZOC immediately ends your movement
- You CAN leave an enemy ZOC (move one hex) but if you enter
  another enemy ZOC, you must stop

Effects on RETREAT:
- Units cannot retreat into enemy ZOC
- If no valid retreat path exists, the unit is eliminated

Effects on SUPPLY:
- Communication paths cannot pass through enemy ZOC
- This affects where replacements can be placed/restored

ZOC is NOT blocked by terrain or friendly units.
```

---

## 7. MCP Resources

Expose reference documents as MCP resources:

```
resources:
  - uri: bfm://rules/full
    name: "Complete Rules"
    description: "Full game rules reference"
    
  - uri: bfm://rules/crt
    name: "Combat Results Table"
    description: "Combat outcomes by odds and die roll"
    
  - uri: bfm://units/german
    name: "German Order of Battle"
    description: "All German units with stats"
    
  - uri: bfm://units/soviet
    name: "Soviet Order of Battle"
    description: "All Soviet units with stats"
```

## 8. MCP Prompts

Pre-built prompts for common scenarios:

```
prompts:
  - name: play_german
    description: "Play as German commander"
    arguments: []
    
  - name: play_soviet
    description: "Play as Soviet commander"
    arguments: []
    
  - name: analyse_position
    description: "Analyse current position without taking action"
    arguments: []
```

### play_german prompt content:

```
You are playing Battle for Moscow as the German commander.

Your objective: Capture Moscow by the end of Turn 7.

You command Army Group Center's forces including powerful panzer corps.
The offensive began well but autumn mud is slowing your advance.

Use the available tools to:
1. Check the situation with get_situation
2. Review your units with get_units
3. Preview attacks with preview_attack before committing
4. Make your moves and attacks

Think strategically:
- Panzers are your striking power — use them to exploit breakthroughs
- Infantry holds ground — don't let your line get too thin
- Moscow is heavily defended — you may need to approach from multiple directions
- Time is against you — be aggressive but not reckless

It is currently your turn. What would you like to do?
```

## 9. Response Formatting

### Principles

1. **Lead with the important information** — don't bury the key point
2. **Use clear structure** — headers, lists, but not excessive
3. **Include coordinates AND landmarks** — "[5,4] near Kaluga"
4. **Explain modifiers** — don't just say "-1", say "forest: -1"
5. **Offer next steps** — what can the player do now?

### Coordinate Display

Always show both axial coordinates and a reference:
- "[5,4] near Kaluga"
- "[10,5] in Moscow"
- "[7,3] south of Mozhaisk"

### Numbers

- Strength: "strength 9" or "9-6" for full/half
- Odds: "3:1" always in this format
- Probabilities: "33%" or "1 in 3" not "0.33"

### Tone

Informative, slightly military flavour, not robotic:
- "The Soviet line is crumbling" not "Soviet units are at low strength"
- "Your panzers can exploit the gap" not "Panzer units have valid moves"

But don't overdo it — clarity first.

## 10. Error Handling

### Invalid Actions

Return clear error with:
- What was attempted
- Why it failed
- What would be valid instead

```
ERROR: Cannot move XLVII Panzer to [8,5]

Reason: Hex occupied by VII Infantry.
        Only one unit may occupy a hex.

XLVII can move to: [5,3], [4,4], [5,5]
```

### Connection Errors

If game API is unreachable:

```
ERROR: Cannot connect to game server at http://localhost:3000

Is the Battle for Moscow server running?
Start it with: cd battle-for-moscow && cargo run
```

### Game State Errors

If game is in unexpected state:

```
ERROR: Cannot declare attacks — not in a combat phase.

Current phase: German Movement Phase
You can: move units, end phase

Attacks are declared in the Combat Phase.
```

## 11. Project Structure

Lives in the existing `battle-for-moscow` monorepo:

```
battle-for-moscow/
├── Cargo.toml              # workspace — add mcp-player to members
├── backend/                # existing game engine
├── frontend/               # existing web UI
├── mcp-player/             # THIS CRATE
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # MCP server entry point
│       ├── mcp/
│       │   ├── mod.rs
│       │   ├── server.rs   # MCP protocol handling
│       │   ├── tools.rs    # Tool definitions
│       │   ├── resources.rs# Resource definitions
│       │   └── prompts.rs  # Prompt definitions
│       ├── game/
│       │   ├── mod.rs
│       │   ├── client.rs   # HTTP client OR use backend types directly
│       │   └── narrator.rs # State → readable text
│       └── config.rs       # Configuration loading
├── data/
│   ├── map.json
│   └── units.json
└── config.example.toml     # MCP player config
```

### Workspace Setup

Update root `Cargo.toml`:

```toml
[workspace]
members = ["backend", "mcp-player"]
```

### Shared Types Option

The `mcp-player` can depend on `backend` to reuse type definitions:

```toml
# mcp-player/Cargo.toml
[dependencies]
backend = { path = "../backend" }
```

This avoids duplicating `GameState`, `Unit`, `Hex` etc. The backend crate would need to make these types public.

## 12. Implementation Order

1. **Project setup** — Cargo, MCP dependencies
2. **Config loading** — API URL, player side
3. **Game API client** — HTTP calls to existing endpoints
4. **Type definitions** — match game engine types
5. **Basic narrator** — state to text conversion
6. **MCP server skeleton** — protocol handling
7. **State tools** — get_situation, get_units
8. **Query tools** — get_valid_moves, preview_attack
9. **Action tools** — move, attack, replace, end_phase
10. **Reference tools** — get_rules
11. **Resources** — rules documents
12. **Prompts** — play_german, play_soviet
13. **Polish** — error messages, edge cases

## 13. Testing

### Unit Tests

- Narrator produces readable output for various game states
- Query tools calculate odds correctly
- Action tools generate correct API calls

### Integration Tests

- Full game flow: movement → combat → resolution → phase change
- Error cases: invalid moves, wrong phase, etc.

### Manual Testing

- Connect Claude Desktop to MCP server
- Play a full game as each side
- Verify LLM can understand state and make sensible moves

## 14. Future Considerations

Not in scope now, but keep in mind:

- **Other games** — narrator and tools could be pluggable
- **Multiple simultaneous games** — game ID in config/calls
- **LLM vs LLM** — two MCP clients playing each other
- **Analysis mode** — evaluate positions without playing

## 15. Dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.11", features = ["json"] }
toml = "0.8"
# MCP SDK - use appropriate Rust MCP library
```

Note: Check for current Rust MCP server libraries. If none suitable, implement protocol directly over stdio (it's JSON-RPC based).
