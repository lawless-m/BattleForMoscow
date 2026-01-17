# Battle for Moscow — Technical Specification

## 1. Overview

A web-based implementation of the 1986 board wargame "Battle for Moscow" by Frank Chadwick. Two-player hot-seat initially, with architecture supporting future AI opponent and network play.

**Source:** The game was released free by Frank Chadwick after GDW closed in 1996. Rules and assets available at grognard.com/bfm/

## 2. Technology

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Backend | Rust + Axum | Performance, type safety |
| Frontend | Vanilla HTML/CSS/JS | Simple, no build step, renders SVG hex map |
| Data format | JSON | Map and unit definitions |
| Communication | REST API | Stateless requests, easy to debug |
| State | Server-side | Rule enforcement, future AI/network support |

## 3. Hex System

### Coordinate System

Use **axial coordinates** (q, r). This simplifies distance calculations and neighbour finding compared to offset coordinates.

```
      ___     ___
     /   \___/   \
     \0,0/   \1,0/
      \-/ 0,1 \-/
       \___/
```

### Key Formulas

**Distance between two hexes:**
```
distance = max(abs(a.q - b.q), abs(a.r - b.r), abs((a.q + a.r) - (b.q + b.r)))
```

**Six neighbours of hex (q, r):**
```
(q+1, r), (q-1, r), (q, r+1), (q, r-1), (q+1, r-1), (q-1, r+1)
```

**Directions:**
- NE: (q+1, r-1)
- E:  (q+1, r)
- SE: (q, r+1)
- SW: (q-1, r+1)
- W:  (q-1, r)
- NW: (q, r-1)

### Map Bounds

The original map is approximately 22 hexes wide by 15 hexes tall. Exact dimensions must be determined from the PDF assets.

## 4. Data Definitions

### 4.1 Map Hex

```json
{
  "q": 5,
  "r": 3,
  "terrain": "clear",
  "city": {
    "name": "Moscow",
    "is_moscow": true
  },
  "fortification": false,
  "rail": true,
  "river_edges": ["NE", "E"],
  "setup": "soviet"
}
```

**Fields:**
- `q`, `r`: Axial coordinates (integers)
- `terrain`: `"clear"` | `"forest"`
- `city`: `null` or object with `name` (string) and `is_moscow` (boolean)
- `fortification`: boolean — Soviet units defend better here
- `rail`: boolean — hex contains rail line
- `river_edges`: array of directions where rivers cross hex edges
- `setup`: `null` | `"german"` | `"soviet"` — initial unit placement markers

### 4.2 Unit Definition (Static)

```json
{
  "id": "XLVII",
  "side": "german",
  "type": "panzer",
  "full_strength": 9,
  "half_strength": 4,
  "movement": 6
}
```

**Fields:**
- `id`: string — unit identifier (Roman numerals for German, Arabic for Soviet)
- `side`: `"german"` | `"soviet"`
- `type`: `"infantry"` | `"panzer"` — only Germans have panzers
- `full_strength`: integer — combat strength at full
- `half_strength`: integer — combat strength when reduced
- `movement`: integer — movement allowance (4 for infantry, 6 for panzer)

### 4.3 Unit State (Runtime)

```json
{
  "id": "XLVII",
  "position": [5, 3],
  "strength": "full"
}
```

**Fields:**
- `id`: string — references unit definition
- `position`: `[q, r]` or `null` if eliminated
- `strength`: `"full"` | `"half"` | `"eliminated"`

### 4.4 Game State

```json
{
  "turn": 1,
  "phase": "german_panzer_movement",
  "units": [...],
  "city_control": {
    "Moscow": "soviet",
    "Kalinin": "soviet"
  },
  "pending_battles": [],
  "moved_this_phase": ["XLVII", "XLI"],
  "german_replacement_used": false,
  "soviet_replacements_remaining": 5,
  "first_shock_army_available": false
}
```

### 4.5 Battle Declaration

```json
{
  "attackers": ["XLVII", "XLVI", "VII"],
  "defender": "5",
  "resolved": false
}
```

## 5. Turn Structure

Seven turns total. Each turn has eight phases in strict order:

| # | Phase | Player | Description |
|---|-------|--------|-------------|
| 1 | `german_replacement` | German | Receive 1 replacement (skip turn 1) |
| 2 | `german_panzer_movement` | German | Only panzer units may move |
| 3 | `german_combat` | German | Declare and resolve all attacks |
| 4 | `german_movement` | German | All units may move |
| 5 | `soviet_replacement` | Soviet | Receive 5 replacements |
| 6 | `soviet_rail_movement` | Soviet | Units on rail may move along rail |
| 7 | `soviet_combat` | Soviet | Declare and resolve all attacks |
| 8 | `soviet_movement` | Soviet | All units may move |

Phase advances when:
- Player explicitly ends it, OR
- No valid actions remain

## 6. Rules Implementation

### 6.1 Zone of Control (ZOC)

Every unit exerts ZOC into the six adjacent hexes.

**Effects:**
- **Movement:** Entering enemy ZOC immediately ends movement for that phase
- **Retreat:** Units cannot retreat into enemy ZOC; eliminated if no valid path
- **Replacements:** Communication path cannot pass through enemy units or ZOC

**Notes:**
- ZOC exists regardless of terrain
- ZOC extends into hexes containing enemy units
- Friendly units do not negate enemy ZOC

### 6.2 Movement

**General rules:**
- Units move one hex at a time in any direction
- Cannot enter hex containing enemy unit
- Cannot end movement stacked (max one unit per hex)
- May pass through hex with friendly unit

**Movement costs:**
| Terrain | Cost |
|---------|------|
| Clear | 1 MP |
| Forest | 2 MP |
| Forest (rail movement) | 1 MP |

**Phase-specific rules:**

| Phase | Who may move | Special rules |
|-------|--------------|---------------|
| German Panzer Movement | Panzer units only | — |
| German Movement | All German units | Panzers may move again |
| Soviet Rail Movement | Units starting on rail | Must follow rail line only |
| Soviet Movement | All Soviet units | Units may move again |

**ZOC and movement:**
- Entering enemy ZOC immediately ends movement
- Unit may start in enemy ZOC and move out (first hex only)

### 6.3 Mud (Turns 3 and 4)

- All movement reduced to **1 hex maximum** per phase
- Soviet rail movement **unaffected**
- All **attack** strengths halved (round down)
- Defence strengths unchanged
- Exchange losses based on printed strength, not halved

### 6.4 Combat

#### Declaration Phase

Before any battles are resolved:
1. Attacker declares ALL battles for the phase
2. Each attacking unit may participate in only ONE battle
3. Each defending unit may be attacked only ONCE
4. Adjacent units not assigned to attack are not required to attack

#### Resolution Sequence (per battle)

1. **Sum attacker strengths**
   - In mud turns: halve each attacker's strength (round down)

2. **Calculate odds**
   - Divide total attack by defender strength
   - Drop fractions (15 vs 4 = 3:1, not 4:1)
   - Cap at 6:1 maximum
   - Floor at 1:1 minimum (below 1:1 = no effect)

3. **Apply terrain modifiers** (each shifts odds down one level)
   - Defender in forest: -1
   - Defender in Moscow: -1
   - Defender is Soviet in fortification: -1
   - ALL attackers across river from defender: -1
   - Modifiers stack (max -4 theoretically)
   - Below 1:1 after modifiers = no effect

4. **Roll d6 and consult CRT**

5. **Apply result immediately**

6. **Advance after combat**
   - If defender hex vacated, ONE attacking unit may advance into it
   - This is optional
   - Does not cost movement

#### Combat Results Table

|      | 1:1 | 2:1 | 3:1 | 4:1 | 5:1 | 6:1 |
|------|-----|-----|-----|-----|-----|-----|
| **1** | AL  | AL  | AL  | NE  | NE  | DR  |
| **2** | AL  | AL  | NE  | NE  | DR  | DR  |
| **3** | AL  | NE  | NE  | DR  | DR  | DRL |
| **4** | NE  | NE  | DR  | DR  | DRL | DRL |
| **5** | NE  | DR  | DR  | DRL | DRL | DE  |
| **6** | DR  | DRL | DRL | DE  | DE  | DE  |

#### Combat Results Explained

| Result | Meaning |
|--------|---------|
| **NE** | No Effect — nothing happens |
| **DR** | Defender Retreat — defender moves 2 hexes (attacker chooses valid path) |
| **DRL** | Defender Retreat with Loss — defender loses one step, then retreats |
| **AL** | Attacker Loss — one attacking unit (attacker's choice) loses one step |
| **DE** | Defender Eliminated — defender removed regardless of current strength |
| **EX** | Exchange — defender loses one step, attacker must lose at least equal strength, then defender retreats if surviving |

#### Retreat Rules

- Defender must end exactly 2 hexes from starting position
- Cannot enter hex with enemy unit
- Cannot enter enemy ZOC
- Cannot end stacked with friendly unit (extend retreat if needed)
- If no valid retreat path exists: unit eliminated
- Attacker chooses retreat path (within rules)

#### Loss Application

- Full strength → Half strength (flip counter)
- Half strength → Eliminated (remove from map)

#### Exchange Calculation

1. Defender takes one step loss
2. Calculate defender's loss value:
   - Full → Half: loss = full_strength - half_strength
   - Half → Eliminated: loss = half_strength
3. Attacker must lose **at least** that much strength from attacking units
4. Attacker chooses which units take losses
5. Defender retreats if surviving

### 6.5 Replacements

**Allocation per turn:**
- Germans: 1 replacement (none on turn 1)
- Soviets: 5 replacements

**Usage options:**
- Restore half-strength unit to full strength
- Create new half-strength unit from eliminated pool

**Cannot:**
- Create full-strength unit directly (requires 2 turns)
- Save unused replacements for later turns

**Placement rules — Soviet:**
- New units: east map edge (any empty hex) OR any friendly city in communication (max one per city)
- Restored units: must be in communication with east edge
- Moscow exception: can always place/restore in Moscow even without communication

**Placement rules — German:**
- New units: west map edge (any empty hex) OR any friendly city in communication
- Restored units: must be in communication with west edge
- No Moscow exception

**Communication:**
- Path of any length from unit/hex to friendly map edge
- Cannot pass through enemy units
- Cannot pass through enemy ZOC
- Can pass through friendly units

**1st Shock Army (Soviet 10-4 unit):**
- Cannot be placed until turn 4 or later
- Otherwise follows normal Soviet replacement rules

### 6.6 City Control

- Cities start Soviet-controlled (except those with German setup markers)
- Control changes when a unit enters the city
- Control = last side to have a unit in the city hex
- Control persists even if city is empty

### 6.7 Victory Conditions

- Game ends after Soviet Movement phase of Turn 7
- **German victory:** Control Moscow
- **Soviet victory:** Control Moscow
- If same side that started with Moscow still controls it, that side wins
- No draws in standard rules

## 7. API Specification

### 7.1 Endpoints

#### GET /api/game

Returns complete current game state.

**Response:** GameState object

---

#### POST /api/game/new

Start a new game with initial setup.

**Response:** GameState object

---

#### POST /api/game/move

Move a unit to a new position.

**Request:**
```json
{
  "unit_id": "XLVII",
  "to": [5, 3]
}
```

**Response:** Updated GameState or error

**Validation:**
- Unit exists and is not eliminated
- Unit belongs to current player
- Current phase allows this unit to move
- Unit has not already moved this phase (unless panzer in movement phase after panzer phase)
- Destination is valid (path exists within movement allowance, respecting ZOC)

---

#### POST /api/game/declare_battles

Declare all battles for the combat phase. Must declare all at once before any resolution.

**Request:**
```json
{
  "battles": [
    {
      "attackers": ["XLVII", "VII"],
      "defender": "5"
    },
    {
      "attackers": ["VIII"],
      "defender": "10"
    }
  ]
}
```

**Response:** Updated GameState with pending_battles populated

**Validation:**
- Current phase is a combat phase
- All attackers belong to current player
- All defenders belong to enemy
- Each attacker is adjacent to its assigned defender
- No unit appears as attacker in multiple battles
- No unit appears as defender in multiple battles

---

#### POST /api/game/resolve_battle

Resolve the next pending battle. Server rolls die.

**Request:**
```json
{
  "battle_index": 0
}
```

**Response:**
```json
{
  "die_roll": 4,
  "odds": "3:1",
  "modified_odds": "2:1",
  "terrain_modifiers": ["forest", "river"],
  "result": "DR",
  "result_description": "Defender retreats",
  "state": { ... }
}
```

---

#### POST /api/game/retreat

Specify retreat path for a defender (when multiple valid paths exist).

**Request:**
```json
{
  "unit_id": "5",
  "path": [[4, 3], [3, 3]]
}
```

**Response:** Updated GameState

---

#### POST /api/game/advance

Advance an attacking unit into vacated defender hex after combat.

**Request:**
```json
{
  "unit_id": "XLVII"
}
```

**Response:** Updated GameState

---

#### POST /api/game/replacement

Use a replacement.

**Request (restore existing unit):**
```json
{
  "unit_id": "5",
  "action": "restore"
}
```

**Request (place new unit):**
```json
{
  "unit_id": "29",
  "action": "place",
  "hex": [10, 2]
}
```

**Response:** Updated GameState

**Validation:**
- Current phase is a replacement phase
- Player has replacements remaining
- For restore: unit is at half strength and in communication
- For place: unit is eliminated, hex is valid placement location

---

#### POST /api/game/end_phase

Advance to next phase.

**Response:** Updated GameState

**Validation:**
- All pending battles resolved (if combat phase)

---

#### GET /api/game/valid_moves/{unit_id}

Get all valid destination hexes for a unit.

**Response:**
```json
{
  "unit_id": "XLVII",
  "valid_hexes": [[5, 3], [5, 4], [6, 3]]
}
```

---

#### GET /api/game/valid_battles

Get all possible battle configurations.

**Response:**
```json
{
  "possible_attacks": [
    {
      "defender": "5",
      "defender_position": [7, 4],
      "possible_attackers": ["XLVII", "VII", "VIII"]
    }
  ]
}
```

---

#### GET /api/game/valid_replacements

Get valid replacement actions.

**Response:**
```json
{
  "remaining": 5,
  "restorable_units": ["5", "10"],
  "placeable_units": ["29", "30"],
  "valid_placement_hexes": [[10, 2], [10, 3], [9, 5]]
}
```

### 7.2 Error Responses

```json
{
  "error": "invalid_move",
  "message": "Unit cannot reach destination",
  "details": {
    "unit_id": "XLVII",
    "requested": [5, 3],
    "reason": "blocked by ZOC at [5, 4]"
  }
}
```

**Error codes:**
- `invalid_move` — movement rule violation
- `invalid_battle` — combat declaration violation
- `invalid_replacement` — replacement rule violation
- `wrong_phase` — action not allowed in current phase
- `wrong_player` — not your turn
- `not_found` — unit or hex doesn't exist
- `game_over` — game has ended

## 8. Frontend Requirements

### 8.1 Display Elements

**Map:**
- SVG hex grid
- Terrain rendering (clear, forest distinguished visually)
- City markers with names
- Fortification markers
- River edges (on hex borders)
- Rail lines

**Units:**
- Counter-style display
- Show unit ID
- Show current strength (attack value)
- Show movement allowance
- Distinguish full/half strength visually
- German = grey/field grey
- Soviet = brown/red

**Game info:**
- Current turn (1-7)
- Current phase
- Active player indicator
- Mud indicator (turns 3-4)
- Replacement counters remaining

**City control:**
- Visual indicator of which side controls each city

### 8.2 Interaction

**Unit selection:**
- Click unit to select
- Show valid moves highlighted
- Click valid hex to move
- Click elsewhere to deselect

**Combat phase:**
- Select attacking units (multi-select)
- Select defender
- Show odds preview
- Confirm battle declaration
- "Declare all battles" button to lock in and start resolution
- Show die roll animation
- Show result
- Handle retreat path selection if needed
- Handle advance decision

**Replacement phase:**
- Show eligible units
- Show valid placement hexes
- Click to restore or place

**Phase control:**
- "End Phase" button always visible
- Confirmation if actions remain

### 8.3 Information Displays

**Before combat resolution:**
- Attacking units and total strength
- Defender and strength
- Raw odds
- Terrain modifiers listed
- Final modified odds

**After combat resolution:**
- Die roll
- Result code and explanation
- Losses taken

**Game end:**
- Victor announcement
- Final city control status

## 9. Data Files

### 9.1 map.json

Must contain all hexes on the map with complete terrain data. See MAPDATA.md for transcription from original game.

### 9.2 units.json

All 39 units. See UNITS.md for complete unit roster.

## 10. Implementation Order

Suggested build sequence:

1. **Hex geometry module** — distance, neighbours, direction calculations
2. **Map data structure** — load and query hex data
3. **Unit data structure** — load unit definitions
4. **Game state** — initialise, serialise, phase tracking
5. **ZOC calculation** — identify enemy ZOC hexes
6. **Movement validation** — path finding with ZOC and terrain costs
7. **Movement execution** — update state, track moved units
8. **Combat declaration** — validate and store pending battles
9. **Combat resolution** — odds, modifiers, CRT lookup, result application
10. **Retreat handling** — path validation, elimination if impossible
11. **Replacement logic** — communication tracing, placement validation
12. **Turn/phase advancement** — state machine
13. **Victory detection** — check Moscow control at game end
14. **API layer** — HTTP handlers wrapping game logic
15. **Frontend: hex rendering** — SVG map display
16. **Frontend: unit rendering** — counters on map
17. **Frontend: interaction** — selection, movement, combat UI
18. **Frontend: game info** — turn track, phase display
19. **Polish** — error messages, edge cases, visual feedback

## 11. Testing Priorities

**Critical path tests:**
- Hex geometry correctness
- ZOC calculation accuracy
- Movement stops on entering ZOC
- Combat odds calculation with terrain
- CRT lookup returns correct results
- Retreat path must be exactly 2 hexes and avoid ZOC
- Retreat into ZOC = elimination
- Communication tracing for replacements
- Mud halves attack strength
- Rail movement stays on rails
- 1st Shock Army not available until turn 4
- Victory correctly detected

**Edge cases:**
- Unit surrounded by ZOC (movement = 0 unless starting in ZOC)
- Multiple terrain modifiers stacking
- Exchange losses calculation
- Panzer moving in both panzer phase and regular phase
- Replacement in Moscow without communication
