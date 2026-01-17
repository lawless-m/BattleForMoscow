# Battle for Moscow — Unit Roster

## Soviet Units (17 total)

All Soviet units are **infantry** with **movement allowance 4**.

| ID | Full Strength | Half Strength | Notes |
|----|---------------|---------------|-------|
| 3  | 8 | 4 | |
| 5  | 8 | 4 | |
| 10 | 8 | 4 | |
| 13 | 8 | 4 | |
| 16 | 8 | 4 | |
| 19 | 8 | 4 | |
| 20 | 8 | 4 | |
| 24 | 8 | 4 | |
| 29 | 8 | 4 | |
| 30 | 8 | 4 | |
| 32 | 8 | 4 | |
| 33 | 8 | 4 | |
| 40 | 8 | 4 | |
| 43 | 8 | 4 | |
| 49 | 8 | 4 | |
| 50 | 8 | 4 | |
| 1S | 10 | 5 | 1st Shock Army — not available until Turn 4 |

**Setup:** 13 units start on map at half strength (on hexes marked with hammer and sickle). 3 units start in replacement pool. 1st Shock Army starts unavailable.

## German Units (22 total)

### Infantry (14 units)

All German infantry have **movement allowance 4**.

| ID | Full Strength | Half Strength |
|----|---------------|---------------|
| V | 6 | 3 |
| VI | 5 | 2 |
| VII | 7 | 4 |
| VIII | 7 | 4 |
| IX | 7 | 4 |
| XII | 5 | 2 |
| XIII | 6 | 3 |
| XX | 6 | 3 |
| XXII | 8 | 4 |
| XXVII | 6 | 3 |
| XXXIV | 4 | 2 |
| XXXV | 8 | 4 |
| XLII | 4 | 2 |
| LIII | 6 | 3 |

### Panzer (8 units)

All German panzers have **movement allowance 6**.

| ID | Full Strength | Half Strength |
|----|---------------|---------------|
| XXIV | 12 | 6 |
| XL | 8 | 4 |
| XLI | 12 | 6 |
| XLVI | 10 | 5 |
| XLVII | 9 | 4 |
| XLVIII | 8 | 4 |
| LVI | 9 | 4 |
| LVII | 12 | 6 |

**Setup:** All 22 German units start on map at full strength (on hexes marked with black cross).

## JSON Format

```json
{
  "units": [
    {"id": "3", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "5", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "10", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "13", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "16", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "19", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "20", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "24", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "29", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "30", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "32", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "33", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "40", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "43", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "49", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "50", "side": "soviet", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "1S", "side": "soviet", "type": "infantry", "full_strength": 10, "half_strength": 5, "movement": 4, "available_turn": 4},

    {"id": "V", "side": "german", "type": "infantry", "full_strength": 6, "half_strength": 3, "movement": 4},
    {"id": "VI", "side": "german", "type": "infantry", "full_strength": 5, "half_strength": 2, "movement": 4},
    {"id": "VII", "side": "german", "type": "infantry", "full_strength": 7, "half_strength": 4, "movement": 4},
    {"id": "VIII", "side": "german", "type": "infantry", "full_strength": 7, "half_strength": 4, "movement": 4},
    {"id": "IX", "side": "german", "type": "infantry", "full_strength": 7, "half_strength": 4, "movement": 4},
    {"id": "XII", "side": "german", "type": "infantry", "full_strength": 5, "half_strength": 2, "movement": 4},
    {"id": "XIII", "side": "german", "type": "infantry", "full_strength": 6, "half_strength": 3, "movement": 4},
    {"id": "XX", "side": "german", "type": "infantry", "full_strength": 6, "half_strength": 3, "movement": 4},
    {"id": "XXII", "side": "german", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "XXVII", "side": "german", "type": "infantry", "full_strength": 6, "half_strength": 3, "movement": 4},
    {"id": "XXXIV", "side": "german", "type": "infantry", "full_strength": 4, "half_strength": 2, "movement": 4},
    {"id": "XXXV", "side": "german", "type": "infantry", "full_strength": 8, "half_strength": 4, "movement": 4},
    {"id": "XLII", "side": "german", "type": "infantry", "full_strength": 4, "half_strength": 2, "movement": 4},
    {"id": "LIII", "side": "german", "type": "infantry", "full_strength": 6, "half_strength": 3, "movement": 4},

    {"id": "XXIV", "side": "german", "type": "panzer", "full_strength": 12, "half_strength": 6, "movement": 6},
    {"id": "XL", "side": "german", "type": "panzer", "full_strength": 8, "half_strength": 4, "movement": 6},
    {"id": "XLI", "side": "german", "type": "panzer", "full_strength": 12, "half_strength": 6, "movement": 6},
    {"id": "XLVI", "side": "german", "type": "panzer", "full_strength": 10, "half_strength": 5, "movement": 6},
    {"id": "XLVII", "side": "german", "type": "panzer", "full_strength": 9, "half_strength": 4, "movement": 6},
    {"id": "XLVIII", "side": "german", "type": "panzer", "full_strength": 8, "half_strength": 4, "movement": 6},
    {"id": "LVI", "side": "german", "type": "panzer", "full_strength": 9, "half_strength": 4, "movement": 6},
    {"id": "LVII", "side": "german", "type": "panzer", "full_strength": 12, "half_strength": 6, "movement": 6}
  ]
}
```

## Notes

- Unit IDs use Roman numerals for German corps, Arabic numerals for Soviet armies
- The "1S" designation indicates 1st Shock Army
- Movement allowance is in hexes (forest costs 2 for non-rail movement)
- All Soviet units are identical except 1st Shock Army
- German units have varied strengths reflecting historical orders of battle
