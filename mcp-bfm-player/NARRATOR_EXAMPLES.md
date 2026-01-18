# Battle for Moscow MCP — Narrator Examples

This document provides examples of how to convert game state into readable text. Use these as templates for implementing the narrator module.

## Situation Overview

### Turn 1, German Panzer Movement Phase

```
TURN 1 — German Panzer Movement Phase

Operation Typhoon begins. Your panzer spearheads are poised to smash
through the Soviet defenses and drive on Moscow.

The Soviet line is weak — all their units are at half strength from
the summer's fighting. But they have numbers and will receive 
reinforcements every turn.

Weather: Clear (full movement and combat)
Moscow: Soviet controlled

8 panzer units ready to move. Infantry will move in the regular
movement phase after combat.

It is your turn. Move your panzers to position for attack.
```

### Turn 3, Soviet Combat Phase (Mud)

```
TURN 3 (MUD) — Soviet Combat Phase

The autumn rains have turned the roads to mire. All movement is 
limited to 1 hex. Attack strength is halved.

Your defensive line has held but the Germans are close. Their
panzers are 3 hexes from Moscow in the south.

Key positions:
- Moscow: Secure, garrisoned by 16th Army
- Kaluga: THREATENED — German XLVII Panzer adjacent
- Tula: Holding, but 43rd Army is at half strength

You have 3 units adjacent to enemies. Counterattacking in mud is
risky — your attacks are at half strength too.

3 possible attacks this phase. Best odds: 1:1 vs German XXXIV Infantry.

What attacks, if any, do you declare?
```

### Turn 7, End of Game Check

```
TURN 7 — Soviet Movement Phase (FINAL TURN)

This is the last turn. After this phase, the game ends.

Moscow: SOVIET CONTROLLED

The German offensive has failed. Despite reaching the outskirts of
the city, they could not break through your final defensive line.

Current situation:
- German units adjacent to Moscow: XLI Panzer, XLVII Panzer
- Soviet units in/adjacent to Moscow: 16th Army (in city), 5th Army, 1st Shock

The Germans cannot take Moscow this phase (no combat phase remaining).

You have won. Move your units to consolidate, then end phase to 
complete the game.
```

## Unit Lists

### German Units (Full Detail)

```
GERMAN FORCES — Turn 2

PANZER CORPS (movement 6, but 1 in mud):

  XXIV Panzer ★★★ [3,4] Vyazma area
    Strength: 12/6 (full)
    Adjacent enemies: None
    Status: Can move (not yet moved this phase)
    Note: Your strongest unit. Use for breakthroughs.

  XLI Panzer ★★★ [5,3] approaching Mozhaisk  
    Strength: 12/6 (full)
    Adjacent enemies: Soviet 5th Army
    Status: Can move, in contact
    
  XLVI Panzer ★★ [6,5] south of Mozhaisk
    Strength: 10/5 (full)
    Adjacent enemies: Soviet 32nd Army
    Status: Can move, in contact

  XLVII Panzer ★★ [4,6] Kaluga area
    Strength: 9/4 (full)
    Adjacent enemies: Soviet 43rd Army, Soviet 49th Army
    Status: Can move, in contact
    Note: Threatening two Soviet units — good position.

  [... remaining panzers ...]

INFANTRY CORPS (movement 4, but 1 in mud):

  VII Infantry [5,4]
    Strength: 7/4 (full)
    Adjacent enemies: Soviet 5th Army
    Status: Can move

  [... etc ...]

ELIMINATED: None

Total: 22 units (22 active, 0 eliminated)
```

### Soviet Units (Brief)

```
SOVIET FORCES — Turn 2

ARMIES (all movement 4):

In contact with enemy:
  5th Army (4) at [6,3] — half strength, facing XLI Panzer
  32nd Army (4) at [7,5] — half strength, facing XLVI Panzer  
  43rd Army (4) at [5,6] — half strength, facing XLVII Panzer
  49th Army (4) at [4,7] — half strength, facing XLVII Panzer

Reserve / Second line:
  16th Army (4) at [10,5] MOSCOW — half strength, garrison
  10th Army (4) at [8,4] — half strength
  [...]

Available for replacement:
  29th Army, 30th Army, 33rd Army (in pool)
  
Not yet available:
  1st Shock Army — available from Turn 4

Replacements this turn: 5 (none used yet)
```

## Attack Previews

### Simple Attack

```
ATTACK PREVIEW

Target: Soviet 43rd Army at [5,6]
  Strength: 4 (half strength)
  Terrain: Clear
  Retreat path: Available (can retreat to [6,7] or [5,7])

Your attackers:
  XLVII Panzer (strength 9)
  
Calculation:
  Attack total: 9
  Defence: 4
  Raw odds: 9 ÷ 4 = 2:1
  
Terrain modifiers: None
River crossing: No

FINAL ODDS: 2:1

Outcomes:
  Roll 1-2 (33%): AL — XLVII Panzer takes step loss (9 → 4)
  Roll 3-4 (33%): NE — No effect
  Roll 5-6 (33%): DR/DRL — 43rd Army retreats, possibly takes loss

Assessment: Reasonable attack. 2 in 3 chance of no harm to you,
1 in 3 chance of pushing them back.
```

### Complex Attack with Modifiers

```
ATTACK PREVIEW

Target: Soviet 5th Army at [6,3]
  Strength: 4 (half strength)
  Terrain: Forest
  Position: Behind Oka River from south
  Retreat path: Limited — [7,3] only, or eliminated

Your attackers:
  XLI Panzer (strength 12) — attacking from [5,3], no river
  VII Infantry (strength 7) — attacking from [5,4], across river
  VIII Infantry (strength 7) — attacking from [6,4], across river

Calculation:
  Attack total: 12 + 7 + 7 = 26
  Defence: 4
  Raw odds: 26 ÷ 4 = 6:1

Terrain modifiers:
  Forest: -1 (defender in forest)
  River: NOT applied (XLI Panzer not crossing river)

FINAL ODDS: 5:1

Outcomes:
  Roll 1-2 (33%): NE/DR — No effect or defender retreats
  Roll 3-4 (33%): DR/DRL — Defender retreats, possibly loses step
  Roll 5-6 (33%): DRL/DE — Defender loses step or eliminated

Assessment: Strong attack. Very likely to push them back or destroy
them. However, committing 3 units here means fewer attacks elsewhere.

Note: If 5th Army retreats to [7,3], they would be in ZOC of your
units at [6,4] and [7,4]. Check if they have a valid retreat.
Actually — [7,3] is their only retreat hex. If it's in ZOC, they're
eliminated instead of retreating. Confirm adjacent unit positions.
```

### Mud Turn Attack

```
ATTACK PREVIEW (MUD — all attack strengths halved)

Target: Soviet 30th Army at [7,5]
  Strength: 8 (full strength)
  Terrain: Clear

Your attackers:
  XLVII Panzer (strength 9 → 4 in mud)
  XLVI Panzer (strength 10 → 5 in mud)
  VII Infantry (strength 7 → 3 in mud)

Calculation:
  Attack total: 4 + 5 + 3 = 12 (halved for mud)
  [Printed strength: 9 + 10 + 7 = 26]
  Defence: 8 (NOT halved — only attackers affected)
  Raw odds: 12 ÷ 8 = 1:1

FINAL ODDS: 1:1

Outcomes:
  Roll 1-2 (33%): AL — One of your units takes step loss
  Roll 3-4 (33%): NE — No effect  
  Roll 5-6 (33%): DR — Defender retreats

Assessment: RISKY. 1 in 3 chance of losing a step on one of your
panzers or infantry. In mud, it's often better to reposition rather
than attack at poor odds.

If you get an Exchange result (not possible at 1:1, but for reference):
your losses would be calculated on PRINTED strength, not halved.
```

## Move Descriptions

### Successful Move

```
MOVED: XLI Panzer

From: [5,3]
To: [6,3]
Path: Direct (1 hex)
Movement spent: 1 of 6 (would be 1 of 1 in mud)

New situation:
  Now adjacent to: Soviet 5th Army, Soviet 32nd Army
  Threatening: Gap between Soviet 5th and 10th Armies

XLI Panzer's movement complete for this phase.
```

### Move into ZOC

```
MOVED: XLVII Panzer

From: [4,5]
To: [5,5]
Path: Direct (1 hex)
Movement spent: 1 (entered enemy ZOC — movement ends)

⚠️  Entered Zone of Control of Soviet 43rd Army at [5,6]

New situation:
  Now adjacent to: Soviet 43rd Army
  In ZOC of: Soviet 43rd Army

Movement ended due to entering enemy ZOC.
```

### Move Not Allowed

```
ERROR: Cannot move XLI Panzer to [6,4]

Reason: Hex [6,4] is occupied by VII Infantry.
        Only one unit may occupy a hex.

XLI Panzer at [5,3] can reach:
  [4,3] — Clear, away from front
  [5,2] — Clear, flanking north
  [6,3] — Clear, toward Soviet 5th Army
  [4,4] — Forest (costs 2 MP)
  [5,4] — Clear, enters ZOC of Soviet 5th Army (movement would end)
```

## Combat Resolution

### Simple Result

```
BATTLE RESOLVED

XLVII Panzer vs Soviet 43rd Army
Odds: 2:1
Die roll: 5

RESULT: DR (Defender Retreat)

Soviet 43rd Army must retreat 2 hexes from [5,6].
Valid retreat destinations: [6,7], [7,7], [7,6]

Retreated to: [7,7] (chosen by you as attacker)

You may advance XLVII Panzer into [5,6].
Type 'advance XLVII' to advance, or 'skip' to decline.
```

### Defender Eliminated on Retreat

```
BATTLE RESOLVED

XLI Panzer + VII Infantry vs Soviet 5th Army
Odds: 4:1
Die roll: 4

RESULT: DR (Defender Retreat)

Soviet 5th Army must retreat 2 hexes from [6,3].

Checking retreat paths...
  [7,3]: In ZOC of your VIII Infantry — BLOCKED
  [6,2]: In ZOC of your VI Infantry — BLOCKED
  [7,2]: Would need to pass through ZOC — BLOCKED
  [5,2]: In ZOC of your XLI Panzer — BLOCKED

NO VALID RETREAT PATH

Soviet 5th Army is ELIMINATED.

Hex [6,3] is now empty.
XLI Panzer or VII Infantry may advance.
```

### Exchange Result

```
BATTLE RESOLVED

XXIV Panzer + XLI Panzer vs Soviet 16th Army
Odds: 3:1
Die roll: 6

RESULT: EX (Exchange)

EXCHANGE RESOLUTION:

1. Defender loss:
   Soviet 16th Army (strength 8) takes step loss → strength 4
   Loss value: 4

2. Attacker must lose at least 4 strength:
   Your attacking units: XXIV (12), XLI (12)
   
   Option A: XXIV takes loss (12 → 6), loss value = 6 ✓
   Option B: XLI takes loss (12 → 6), loss value = 6 ✓
   
   Choose which unit takes the loss.

3. Defender retreats:
   Soviet 16th Army (now strength 4) must retreat 2 hexes.
   Valid retreat hexes: [11,5], [10,6]

Which unit takes the attacker loss? Type unit ID.
```

## Phase Transitions

### End of Combat Phase

```
PHASE COMPLETE: German Combat Phase

Battles resolved: 3
Results: 1× DR, 1× DRL, 1× NE
Losses: Soviet 43rd Army eliminated, Soviet 5th Army reduced

─────────────────────────────────

NOW: German Movement Phase

All German units may move (including panzers that already moved
in Panzer Movement Phase).

You pushed back the Soviet center. Consider:
- Advance toward Moscow through the gap
- Consolidate your line before Soviet counterattack
- Protect your flanks — Soviet replacements incoming

14 units have not yet moved in regular movement.
```

### End of Turn

```
PHASE COMPLETE: Soviet Movement Phase

─────────────────────────────────

TURN 3 COMPLETE

Summary:
- German attacks: 4 (2 successful)
- Soviet attacks: 1 (repulsed)
- German losses: VII Infantry reduced
- Soviet losses: 43rd Army eliminated, 5th Army reduced, 30th retreated
- Territory: Germans advanced 2 hexes toward Moscow

Moscow: Still Soviet controlled
Distance to Moscow: 3 hexes (German XLVII Panzer closest)

Weather next turn: MUD (Turn 4)
Movement: 1 hex maximum
Combat: Attack strength halved

─────────────────────────────────

TURN 4 BEGINS

German Replacement Phase
You receive 1 replacement.

Eligible for restoration:
- VII Infantry at [5,4] (strength 4 → 7)

Eliminated units available:
- None

Use replacement, or end phase to skip.
```

## Rules Explanations

### Zone of Control

```
ZONE OF CONTROL (ZOC)

Every unit projects a Zone of Control into the 6 adjacent hexes:

        ┌───┐
    ┌───┤ZOC├───┐
    │ZOC├───┤ZOC│
    ├───┤ U ├───┤
    │ZOC├───┤ZOC│
    └───┤ZOC├───┘
        └───┘

EFFECTS:

1. MOVEMENT
   • Entering an enemy ZOC immediately stops your movement
   • You CAN start in enemy ZOC and move out (1 hex)
   • If that 1 hex is also enemy ZOC, you stop there

2. RETREAT  
   • Units cannot retreat into enemy ZOC
   • If all retreat hexes are in enemy ZOC → unit ELIMINATED
   • This makes surrounding enemies very powerful

3. SUPPLY/COMMUNICATION
   • Replacement paths cannot pass through enemy ZOC
   • This limits where you can place/restore units

ZOC exists even into hexes containing other units.
Terrain does not block ZOC.
Friendly units do not negate enemy ZOC.
```

### Combat Odds

```
COMBAT ODDS

1. SUM attacker strengths (halved in mud)
2. DIVIDE by defender strength
3. DROP fractions

Examples:
  15 attacking 4 = 3:1 (15÷4=3.75, drop .75)
  16 attacking 4 = 4:1 (16÷4=4)
  12 attacking 5 = 2:1 (12÷5=2.4, drop .4)

LIMITS:
  Maximum: 6:1 (anything higher counts as 6:1)
  Minimum: 1:1 (anything lower has no effect)

TERRAIN MODIFIERS (each shifts odds DOWN one level):
  • Defender in forest: 4:1 → 3:1
  • Defender in Moscow: 4:1 → 3:1  
  • Soviet defender in fortification: 4:1 → 3:1
  • ALL attackers across river: 4:1 → 3:1

Modifiers stack:
  Forest + River = two shifts: 4:1 → 2:1

If modifiers reduce odds below 1:1, attack has no effect.
```

### Mud Effects

```
MUD (Turns 3 and 4)

The autumn rains have arrived. Roads become impassable.

MOVEMENT:
  • ALL units limited to 1 hex per phase
  • Exception: Soviet rail movement unaffected

COMBAT:
  • All ATTACK strengths halved (round down)
  • Defence strengths NOT affected
  
  Example: 
    Your 12-strength panzer attacks → counts as 6
    Enemy 8-strength unit defends → still counts as 8
    Odds: 6 vs 8 = below 1:1 = NO EFFECT

EXCHANGES:
  • Your exchange losses use PRINTED strength
  • Not the halved attack strength
  
  Example:
    You lose a step on your 12-strength panzer in exchange
    Loss value = 12 - 6 = 6 (not based on halved 6 - 3)

STRATEGY:
  • Mud favours the defender
  • Consider repositioning rather than attacking
  • Wait for clear weather in Turn 5
  • Or accept poor odds if time pressure demands action
```
