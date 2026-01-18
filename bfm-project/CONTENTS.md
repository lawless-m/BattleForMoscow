# Battle for Moscow — Digital Implementation

## Project Overview

A web-based implementation of Frank Chadwick's 1986 introductory wargame "Battle for Moscow", covering Operation Typhoon — the German attempt to capture Moscow in late 1941.

## Contents

| File | Description | Start Here? |
|------|-------------|-------------|
| **CONTENTS.md** | This file — project overview | ✓ Read first |
| **SPEC.md** | Technical specification — architecture, data structures, API, implementation order | ✓ Main reference |
| **RULES.md** | Original game rules transcribed for reference | Reference |
| **UNITS.md** | Complete unit roster with stats, includes JSON format | Data source |
| **MAPDATA.md** | Guidance for transcribing map from PDF | Task guidance |

## Where to Start

1. **Read SPEC.md** — this is the main technical specification
2. **Reference RULES.md** — when you need to verify rule interpretations
3. **Use UNITS.md** — copy the JSON unit data directly
4. **Follow MAPDATA.md** — to create the map.json data file

## Technology Stack

- **Backend:** Rust + Axum
- **Frontend:** Vanilla HTML/CSS/JS with SVG rendering
- **Data:** JSON files for map and unit definitions
- **Communication:** REST API

## External Resources

The original game assets (map, counters, rules) are freely available at:
- https://grognard.com/bfm/

Download the PDF versions:
- map.pdf — game map with charts
- counters.pdf — unit counters

## Implementation Order

See SPEC.md Section 10 for detailed build sequence. Summary:

1. Hex geometry
2. Map/unit data structures
3. Game state management
4. Movement rules (with ZOC)
5. Combat system
6. Replacements
7. Turn/phase management
8. Victory detection
9. API layer
10. Frontend

## Key Rules to Get Right

- **ZOC stops movement** — entering enemy ZOC ends movement immediately
- **Retreat exactly 2 hexes** — not 1, not 3, exactly 2 (unless eliminated)
- **Cannot retreat into ZOC** — unit eliminated if no valid path
- **Mud halves attack only** — defence unchanged, and exchange losses use printed values
- **1st Shock Army** — not available until turn 4
- **Moscow exception** — Soviets can place/restore there without communication

## Licence

The Battle for Moscow game is copyright Frank Chadwick. It was released free for recreational use after GDW closed in 1996. Not for commercial use.
