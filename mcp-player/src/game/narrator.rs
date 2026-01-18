use serde_json::Value;

/// Converts game state into human-readable narrative text
pub struct Narrator {
    coordinate_format: String,
    verbosity: String,
}

impl Narrator {
    pub fn new(coordinate_format: String, verbosity: String) -> Self {
        Self {
            coordinate_format,
            verbosity,
        }
    }

    /// Generate situation overview
    pub fn narrate_situation(&self, state: &Value) -> String {
        let mut output = String::new();

        // Extract key information from state
        let turn = state["turn"].as_i64().unwrap_or(1);
        let phase = state["phase"].as_str().unwrap_or("Unknown");
        let weather = state["weather"].as_str().unwrap_or("Clear");

        output.push_str(&format!("TURN {} ({}) — {}\n\n", turn, weather, phase));

        // Add weather effects if mud
        if weather == "Mud" {
            output.push_str("The autumn rains have turned the roads to mire. All movement is limited to 1 hex. Attack strength is halved.\n\n");
        }

        // Add strategic situation
        let moscow_controller = state["moscow_controller"].as_str().unwrap_or("Soviet");
        output.push_str(&format!("Moscow: {} controlled\n\n", moscow_controller));

        // TODO: Add more detailed situation analysis
        output.push_str("Analyzing current position...\n");

        output
    }

    /// Generate unit list
    pub fn narrate_units(&self, state: &Value, side: Option<&str>) -> String {
        let mut output = String::new();

        let sides = match side {
            Some(s) => vec![s],
            None => vec!["german", "soviet"],
        };

        for side_name in sides {
            output.push_str(&format!("{} UNITS:\n\n", side_name.to_uppercase()));

            if let Some(units) = state["units"].as_array() {
                for unit in units {
                    if unit["side"].as_str().unwrap_or("") == side_name {
                        let id = unit["id"].as_str().unwrap_or("Unknown");
                        let strength = unit["strength"].as_i64().unwrap_or(0);
                        let position = &unit["position"];
                        let q = position[0].as_i64().unwrap_or(0);
                        let r = position[1].as_i64().unwrap_or(0);

                        output.push_str(&format!(
                            "- {} (strength {}) at {}\n",
                            id,
                            strength,
                            self.format_hex(q, r)
                        ));
                    }
                }
            }

            output.push('\n');
        }

        output
    }

    /// Generate threat analysis
    pub fn narrate_threats(&self, state: &Value, side: &str) -> String {
        let mut output = String::new();

        output.push_str(&format!("THREATS TO {} POSITION:\n\n", side.to_uppercase()));
        output.push_str("Analyzing tactical situation...\n");

        // TODO: Implement threat analysis logic

        output
    }

    /// Generate valid moves list
    pub fn narrate_valid_moves(&self, unit_id: &str, moves: &Value) -> String {
        let mut output = String::new();

        output.push_str(&format!("VALID MOVES FOR {}:\n\n", unit_id));

        if let Some(move_list) = moves.as_array() {
            for move_option in move_list {
                let q = move_option["q"].as_i64().unwrap_or(0);
                let r = move_option["r"].as_i64().unwrap_or(0);
                let cost = move_option["cost"].as_i64().unwrap_or(0);

                output.push_str(&format!(
                    "- {}: {} MP\n",
                    self.format_hex(q, r),
                    cost
                ));
            }
        }

        output
    }

    /// Generate attack preview
    pub fn narrate_attack_preview(&self, preview: &Value) -> String {
        let mut output = String::new();

        output.push_str("ATTACK PREVIEW:\n\n");

        let attack_strength = preview["attack_strength"].as_i64().unwrap_or(0);
        let defense_strength = preview["defense_strength"].as_i64().unwrap_or(0);
        let odds = preview["odds"].as_str().unwrap_or("1:1");

        output.push_str(&format!("Attack strength: {}\n", attack_strength));
        output.push_str(&format!("Defense strength: {}\n", defense_strength));
        output.push_str(&format!("Odds: {}\n\n", odds));

        output.push_str("POSSIBLE OUTCOMES:\n");
        // TODO: Add outcome probabilities from CRT

        output
    }

    /// Generate combat resolution narrative
    pub fn narrate_combat_result(&self, result: &Value) -> String {
        let mut output = String::new();

        output.push_str("BATTLE RESOLVED:\n\n");

        let die_roll = result["die_roll"].as_i64().unwrap_or(1);
        let outcome = result["outcome"].as_str().unwrap_or("NE");

        output.push_str(&format!("Die roll: {}\n", die_roll));
        output.push_str(&format!("Result: {}\n\n", outcome));

        // Explain the result
        match outcome {
            "NE" => output.push_str("No effect.\n"),
            "DR" => output.push_str("Defender retreats.\n"),
            "DRL" => output.push_str("Defender retreats with loss.\n"),
            "DE" => output.push_str("Defender eliminated.\n"),
            "AL" => output.push_str("Attacker loses a step.\n"),
            "EX" => output.push_str("Exchange - both sides take losses.\n"),
            _ => output.push_str("Unknown result.\n"),
        }

        output
    }

    /// Format hex coordinates
    fn format_hex(&self, q: i64, r: i64) -> String {
        match self.coordinate_format.as_str() {
            "axial" => format!("[{},{}]", q, r),
            "original" => {
                // Convert to XXYY format if needed
                format!("[{},{}]", q, r)
            }
            _ => format!("[{},{}]", q, r),
        }
    }

    /// Generate rules explanation
    pub fn narrate_rules(&self, topic: &str) -> String {
        match topic.to_lowercase().as_str() {
            "zoc" | "zone of control" => self.explain_zoc(),
            "combat" | "attacking" => self.explain_combat(),
            "odds" | "combat odds" => self.explain_odds(),
            "retreat" => self.explain_retreat(),
            "movement" => self.explain_movement(),
            "mud" => self.explain_mud(),
            "replacements" => self.explain_replacements(),
            "victory" => self.explain_victory(),
            _ => format!("No rules explanation available for topic: {}", topic),
        }
    }

    fn explain_zoc(&self) -> String {
        r#"ZONE OF CONTROL (ZOC)

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

ZOC is NOT blocked by terrain or friendly units."#
            .to_string()
    }

    fn explain_combat(&self) -> String {
        r#"COMBAT

During a combat phase, you declare all attacks simultaneously, then resolve them one at a time.

DECLARING ATTACKS:
- Multiple units can attack a single defender
- Each attacker must be adjacent to the defender
- Each unit can only attack once per phase

RESOLVING ATTACKS:
1. Calculate total attack strength
2. Apply modifiers (mud, terrain)
3. Determine odds ratio
4. Roll one die
5. Consult Combat Results Table
6. Apply results (losses, retreats)

COMBAT RESULTS:
- NE: No effect
- DR: Defender retreats 2 hexes
- DRL: Defender retreats with step loss
- DE: Defender eliminated
- AL: Attacker loses a step
- EX: Exchange - both sides lose steps"#
            .to_string()
    }

    fn explain_odds(&self) -> String {
        r#"COMBAT ODDS

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
  Forest + River = two shifts: 4:1 → 2:1"#
            .to_string()
    }

    fn explain_retreat(&self) -> String {
        r#"RETREAT

When a combat result requires retreat, the unit must move 2 hexes away from the attacker.

VALID RETREAT:
- Must move directly away from attacker
- Cannot enter enemy ZOC
- Cannot enter occupied hexes
- Cannot cross impassable terrain

FAILED RETREAT:
If no valid retreat path exists, the unit is ELIMINATED instead.

RETREAT WITH LOSS (DRL):
Unit takes a step loss (full → half, half → eliminated) before retreating."#
            .to_string()
    }

    fn explain_movement(&self) -> String {
        r#"MOVEMENT

Each unit has a movement allowance based on type:
- Panzers: 6 MP
- Infantry: 4 MP
- Soviet Armies: 4 MP

TERRAIN COSTS:
- Clear: 1 MP
- Forest: 2 MP
- River: +1 MP to cross

ZONE OF CONTROL:
Entering enemy ZOC immediately ends movement.

MUD:
All movement limited to 1 hex maximum."#
            .to_string()
    }

    fn explain_mud(&self) -> String {
        r#"MUD (Turns 3 and 4)

The autumn rains have arrived. Roads become impassable.

MOVEMENT:
  • ALL units limited to 1 hex per phase
  • Exception: Soviet rail movement unaffected

COMBAT:
  • All ATTACK strengths halved (round down)
  • Defence strengths NOT affected

STRATEGY:
  • Mud favours the defender
  • Consider repositioning rather than attacking
  • Wait for clear weather in Turn 5"#
            .to_string()
    }

    fn explain_replacements(&self) -> String {
        r#"REPLACEMENTS

Each side receives replacements each turn:
- German: 1 replacement per turn
- Soviet: 5 replacements per turn

USES:
1. Restore half-strength unit to full strength
2. Bring eliminated unit back at half strength

PLACEMENT:
- Must be in friendly-controlled city
- Must have line of communication (no enemy ZOC)
- Soviets can always place in Moscow"#
            .to_string()
    }

    fn explain_victory(&self) -> String {
        r#"VICTORY CONDITIONS

The game lasts 7 turns (October-December 1941).

GERMAN VICTORY:
Capture Moscow by the end of Turn 7.

SOVIET VICTORY:
Hold Moscow through Turn 7.

Moscow is worth the entire war - there is no partial victory."#
            .to_string()
    }
}
