# Battle for Moscow — Original Rules

Source: Frank Chadwick, Game Designers' Workshop, 1986
Hosted at: grognard.com/bfm/

## Overview

**Operation Typhoon**, the German Army's final lunge to capture Moscow in 1941, was intended to break the Soviet Army and end its resistance to German conquest. If the operation succeeded, it would mean the collapse of Soviet morale (or so the Germans believed). If it failed, it would (and did) leave the exhausted Germans open to a Soviet counter-offensive that would push them forever beyond reach of Moscow.

**Battle for Moscow** is a historical wargame of the German Army's struggle to defeat the Soviet Army and capture Moscow in 1941. It is played on a map of the territory where the battle was fought, and it uses playing pieces which represent the actual military units (German corps and Soviet armies) from the battle.

## Game Components

- Map divided into hexagons
- 39 counters (German and Soviet units)
- Combat Results Table
- Terrain Effects Chart
- Turn Record Track
- One six-sided die

### Counters

Each unit has two counters: full-strength and half-strength.

**Counter information:**
- Unit type: Infantry or Panzer (only Germans have panzers)
- Combat strength: value in battle (higher = stronger)
- Movement allowance: how far the unit can move
- Unit size and identification: historical interest only

Soviet units are orange/red. German units are green.

### Map

The map is divided into hexagons (hexes) which define unit positions. The map shows:
- Forests
- Cities (including Moscow)
- Fortifications
- Rivers
- Railroads

## How to Play

There are **seven turns** in Battle for Moscow. Each turn represents one week.
**Exception:** Turns 3 and 4 each represent two weeks (mud slows the battle).

Each turn has eight phases performed in exact order:

### German Player's Turn
1. **German Replacement Phase** — Germans receive replacements
2. **German Panzer Movement Phase** — All panzers may move
3. **German Combat Phase** — All German units may attack
4. **German Movement Phase** — All German units may move (including panzers that moved in phase 2)

### Soviet Player's Turn
5. **Soviet Replacement Phase** — Soviets receive replacements
6. **Soviet Rail Movement Phase** — All Soviet units on rail lines may move along rails
7. **Soviet Combat Phase** — All Soviet units may attack
8. **Soviet Movement Phase** — All Soviet units may move (including those that moved in phase 6)

## Zone of Control

Each unit has a **zone of control (ZOC)** consisting of the six hexes surrounding it, including hexes occupied by enemy units.

**Effects:**
- **Movement:** A unit entering an enemy ZOC must immediately end its movement phase
- **Combat:** Units cannot end their retreat in an enemy ZOC (eliminated if they must)
- **Replacements:** ZOC affects how paths can be traced for replacements

## Movement

Units move during movement phases (phases 2, 4, 6, and 8).

Each unit has a **movement allowance** representing distance in hexes it can move.
**Exception:** A forest hex counts as two hexes for movement.

**Rules:**
- Move any or all qualifying units (only panzers in panzer phase; only Soviets on rail in rail phase)
- Units move one at a time, hex to hex, in any direction
- Cannot enter a hex containing an enemy unit
- Can enter hex with friendly unit, but only one unit per hex at end of phase
- Entering enemy ZOC immediately ends movement

**Rail Movement (Soviet only):**
- Units must start on a rail line
- Must move only along the rail line
- Forest counts as one hex (not two) for rail movement

## Combat

In each combat phase, units may attack adjacent enemy units.

### Declaration
First, the attacking player announces ALL battles:
- Which enemy units to attack
- Which of his units will attack them

**Rules:**
- A single unit may only attack once per phase
- A single enemy unit may only be attacked once per phase
- Once announced, attacker cannot change mind

### Resolution
Battles resolved one at a time, in any order attacker wants.

**Sequence for each battle:**
1. Total combat strengths of all attacking units
2. Divide by defender's combat strength, drop fractions, to get odds (e.g., 16 vs 4 = 4:1; 15 vs 4 = 3:1)
3. Determine if terrain reduces odds
4. Roll one die, consult CRT
5. Apply result immediately
6. If defender hex vacated, one attacker may advance into it
7. Proceed to next battle

**Maximum and Minimum Odds:**
- Above 6:1 reduces to 6:1
- Below 1:1 after terrain = no effect

**Terrain Effects (each reduces odds by one level):**
- Defender in forest
- Defender in Moscow
- Defender is Soviet in fortification
- All attackers across river from defender
- Multiple effects stack

## Combat Results Table

| Die | 1:1 | 2:1 | 3:1 | 4:1 | 5:1 | 6:1 |
|-----|-----|-----|-----|-----|-----|-----|
| 1   | AL  | AL  | AL  | NE  | NE  | DR  |
| 2   | AL  | AL  | NE  | NE  | DR  | DR  |
| 3   | AL  | NE  | NE  | DR  | DR  | DRL |
| 4   | NE  | NE  | DR  | DR  | DRL | DRL |
| 5   | NE  | DR  | DR  | DRL | DRL | DE  |
| 6   | DR  | DRL | DRL | DE  | DE  | DE  |

### Combat Results

**NE (No Effect):** Nothing happens.

**DR (Defender Retreat):** Defending unit moved two hexes by attacking player. Must end exactly two hexes away, may not enter enemy ZOC. If no valid path, unit eliminated. Must end in empty hex (extend retreat if necessary).

**DRL (Defender Retreat and Loss):** Defender takes loss first, then retreats if surviving. Full-strength → half-strength. Half-strength → eliminated.

**AL (Attacker Loss):** One attacking unit (attacker's choice) takes a loss (but doesn't retreat).

**DE (Defender Eliminated):** Defending unit entirely eliminated regardless of current strength.

**EX (Exchange):** Defender takes loss. Then attacker must lose at least same amount of strength from attacking units. Finally, defender retreats if surviving.

**Exchange calculation:** If full-strength unit reduced to half-strength, loss = original strength minus reduced strength. For example, panzer strength 9 reduced to 4 = loss of 5.

## Mud

Turns 3 and 4 are **mud turns**.

**Effects:**
- All movement except Soviet rail reduced to 1 hex per phase
- Rail movement unaffected
- All combat strengths halved when attacking (not defending)
- Attacker's exchange losses based on printed strength, not halved

## 1st Shock Army

This Soviet unit (10-4) may not begin on map and may not be taken as replacement until Turn 4 (Nov 1/11).

## Replacements

Both players receive replacements each turn in their replacement phases.

**Allocation:**
- Germans: 1 replacement
- Soviets: 5 replacements

**Usage:**
- Create new half-strength unit from eliminated pool
- Flip existing half-strength unit to full strength
- Cannot use two replacements at once to create full-strength unit from nothing (takes two turns)

**Soviet placement:**
- New units: east edge of map (any empty hex) OR any empty friendly-owned city in communication (max one per city)
- Existing units: must be in communication with east edge to restore
- **Moscow exception:** Can bring in or restore unit in Moscow even without communication

**German placement:**
- Same as Soviet but communication traced to west edge
- No Moscow exception

**Communication:** Path of any length without entering enemy unit or enemy ZOC to friendly map edge.

**Friendly-owned city:** Your units were last in the city. All cities Soviet-owned at start except those with German setup units.

**Unused replacements cannot be saved.**

### Game Balance Variant
- To benefit Germans: reduce Soviet replacements to 4 or 3
- To benefit Soviets: Germans receive replacement only on turns 2, 4, and 6

## Starting the Game

1. Set up one Soviet unit on each hex marked with hammer and sickle, all at half strength
2. Don't use 1st Shock Army (comes later)
3. Four Soviet units left over (including 1st Shock) — all except 1st Shock available as replacements
4. Set up one German unit on each black cross, all at full strength
5. German player begins his panzer movement phase (skips replacement phase on turn 1 since all units at full strength)

## Winning the Game

Whoever holds Moscow at the end of the game wins.

A player **holds Moscow** if one of his units was the last unit to be in the city.

The Russians hold Moscow at the start.
