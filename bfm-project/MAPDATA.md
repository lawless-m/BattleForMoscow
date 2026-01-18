# Battle for Moscow — Map Data

## Overview

The map must be transcribed from the original game PDF available at:
https://grognard.com/bfm/

Download:
- map.pdf (514592 bytes) — contains map with Turn Record and Terrain Effects Chart

## Map Characteristics

The original map uses an **offset coordinate system** with numbered hexes (XXYY format).

For this implementation, convert to **axial coordinates** (q, r) for simpler geometry calculations.

### Approximate Dimensions

The map is roughly:
- 22 hexes wide (west to east)
- 15 hexes tall (north to south)

Exact dimensions must be verified from the PDF.

## Terrain Types to Capture

For each hex, record:

### Basic Terrain
- **Clear** — default, no special effects
- **Forest** — costs 2 MP to enter, reduces combat odds by 1

### Features
- **City** — named location, affects victory and replacements
  - Mark Moscow specifically (is_moscow: true)
- **Fortification** — Soviet units defend at -1 odds shift (marked with specific symbol)
- **Rail line** — allows Soviet rail movement

### Hex Edges
- **River** — if ALL attackers cross river, -1 odds shift
  - Record which edges of each hex have rivers
  - Use direction codes: NE, E, SE, SW, W, NW

### Setup Markers
- **Soviet setup** (hammer and sickle symbol) — hex where Soviet unit starts
- **German setup** (black cross symbol) — hex where German unit starts

## Cities

Cities that need to be identified (verify against map):

- **Moscow** — the victory objective, has special replacement rules
- **Kalinin** — city
- **Tula** — city  
- **Kaluga** — city
- **Volokolamsk** — city
- **Mozhaisk** — city
- **Vyazma** — city (may be off-map or German start)

Note: Some cities may be German-controlled at start if they have German setup markers.

## Data Format

```json
{
  "hexes": [
    {
      "q": 0,
      "r": 0,
      "terrain": "clear",
      "city": null,
      "fortification": false,
      "rail": false,
      "river_edges": [],
      "setup": null
    },
    {
      "q": 10,
      "r": 7,
      "terrain": "clear",
      "city": {"name": "Moscow", "is_moscow": true},
      "fortification": false,
      "rail": true,
      "river_edges": ["NE", "E"],
      "setup": "soviet"
    }
  ],
  "map_bounds": {
    "min_q": 0,
    "max_q": 21,
    "min_r": 0,
    "max_r": 14
  },
  "edges": {
    "west": "german_communication",
    "east": "soviet_communication"
  }
}
```

## Coordinate Conversion

The original map uses offset coordinates (XXYY). To convert to axial:

For **odd-q offset** (common in wargames):
```
axial_q = offset_x
axial_r = offset_y - (offset_x - (offset_x & 1)) / 2
```

For **even-q offset**:
```
axial_q = offset_x
axial_r = offset_y - (offset_x + (offset_x & 1)) / 2
```

Determine which system the original uses by examining hex numbering pattern, then apply consistent conversion.

## Transcription Process

1. Download PDF from grognard.com/bfm/map.pdf
2. Identify coordinate system used
3. List all hexes systematically (row by row or column by column)
4. For each hex record:
   - Coordinates (original and converted)
   - Terrain type
   - City name if present
   - Fortification marker if present
   - Rail line if present
   - River edges (check all 6 directions)
   - Setup marker if present
5. Verify city names against rules
6. Double-check setup hexes match unit count (13 Soviet, 22 German)

## Validation Checks

After transcription, verify:

- [ ] Total Soviet setup hexes = 13
- [ ] Total German setup hexes = 22
- [ ] Moscow is marked correctly
- [ ] All cities from rules are present
- [ ] Rail lines form connected network
- [ ] Rivers make geographical sense
- [ ] Map edges are clearly defined for communication tracing
- [ ] No orphan hexes (all hexes connected to map)

## Notes for Implementation

- The west edge of map = German communication source
- The east edge of map = Soviet communication source
- Edge hexes should be marked so communication tracing knows when it reaches the edge
- Some hexes may be partial (map edge) — decide whether to include or exclude
