use crate::game::narrator::Narrator;
use serde_json::Value;

pub const HELP_TEXT: &str = r#"BATTLE FOR MOSCOW - COMMANDS

INFORMATION:
  HELP, H                      Show this help
  SITUATION, SIT               Game overview
  UNITS [side]                 List units (german/soviet/all)
  THREATS, T                   Danger analysis
  RULES <topic>                Explain a rule

QUERIES:
  MOVES <unit>, M <unit>       Valid moves for unit
  ATTACKS, A                   All possible attacks
  PREVIEW <def> WITH <atk...>  Calculate attack odds
  REPLACEMENTS, REP            Replacement options

ACTIONS:
  MOVE <unit> <q>,<r>          Move unit to hex
  ATTACK <def> WITH <atk...>   Declare attack
  DONE ATTACKS                 Finish declaring, start resolution
  RESOLVE                      Resolve next battle
  ADVANCE <unit>               Advance after combat
  SKIP                         Skip advance
  REPLACE <unit> [<q>,<r>]     Use replacement
  END, DONE                    End current phase

GAME:
  NEW                          New game
  QUIT, EXIT, Q                Exit

Examples:
  > sit
  > moves XLVII
  > move XLVII 4,5
  > attack 16A WITH XLVII XL
  > preview 16A WITH XLVII XL
  > end
"#;

pub fn print_phase_prompt(state: &Value, narrator: &Narrator) -> String {
    let mut output = String::new();

    output.push_str("══════════════════════════════════════════════════════════════\n\n");

    // Phase header
    let turn = state["turn"].as_i64().unwrap_or(1);
    let phase = state["phase"].as_str().unwrap_or("Unknown");
    let weather = state["weather"].as_str().unwrap_or("Clear");

    let mud = if weather == "Mud" { " (MUD)" } else { "" };
    output.push_str(&format!("TURN {}{} — {}\n\n", turn, mud, format_phase(phase)));

    // Phase-specific prompt
    output.push_str(&get_phase_description(phase, weather));

    output
}

fn format_phase(phase: &str) -> String {
    match phase {
        "GermanPanzerMovement" => "German Panzer Movement Phase",
        "GermanCombat" => "German Combat Phase",
        "GermanMovement" => "German Movement Phase",
        "GermanReplacement" => "German Replacement Phase",
        "SovietCombat" => "Soviet Combat Phase",
        "SovietMovement" => "Soviet Movement Phase",
        "SovietRailMovement" => "Soviet Rail Movement Phase",
        "SovietReplacement" => "Soviet Replacement Phase",
        _ => phase,
    }
    .to_string()
}

fn get_phase_description(phase: &str, weather: &str) -> String {
    match phase {
        "GermanPanzerMovement" => {
            let mut desc = String::from("Move your Panzer units (6 MP each");
            if weather == "Mud" {
                desc.push_str(", limited to 1 hex in mud");
            }
            desc.push_str(").\n");
            desc.push_str("Entering enemy ZOC ends movement.\n");
            desc.push_str("\nCommands: MOVES <unit>, MOVE <unit> <q>,<r>, END\n");
            desc
        }
        "GermanCombat" | "SovietCombat" => {
            let side = if phase.starts_with("German") {
                "German"
            } else {
                "Soviet"
            };
            let mut desc = format!("{} Combat Phase\n\n", side);
            desc.push_str("Declare attacks, then resolve them one by one.\n");
            if weather == "Mud" {
                desc.push_str("REMINDER: Attack strengths are halved in mud!\n");
            }
            desc.push_str("\nCommands:\n");
            desc.push_str("  ATTACKS              - List possible attacks\n");
            desc.push_str("  PREVIEW <def> WITH <atk...> - Calculate odds\n");
            desc.push_str("  ATTACK <def> WITH <atk...>  - Declare attack\n");
            desc.push_str("  DONE ATTACKS         - Finish declaring\n");
            desc.push_str("  RESOLVE              - Resolve next battle\n");
            desc.push_str("  ADVANCE <unit>       - Advance after victory\n");
            desc.push_str("  SKIP                 - Skip advance\n");
            desc
        }
        "GermanMovement" | "SovietMovement" => {
            let side = if phase.starts_with("German") {
                "German"
            } else {
                "Soviet"
            };
            let mut desc = format!("{} Movement Phase\n\n", side);
            desc.push_str("Move your infantry units (4 MP each");
            if weather == "Mud" {
                desc.push_str(", limited to 1 hex in mud");
            }
            desc.push_str(").\n");
            desc.push_str("Entering enemy ZOC ends movement.\n");
            desc.push_str("\nCommands: MOVES <unit>, MOVE <unit> <q>,<r>, END\n");
            desc
        }
        "SovietRailMovement" => {
            let mut desc = String::from("Soviet Rail Movement Phase\n\n");
            desc.push_str("Move units along rail lines. Rail movement is NOT affected by mud.\n");
            desc.push_str("\nCommands: MOVES <unit>, MOVE <unit> <q>,<r>, END\n");
            desc
        }
        "GermanReplacement" | "SovietReplacement" => {
            let side = if phase.starts_with("German") {
                "German"
            } else {
                "Soviet"
            };
            let mut desc = format!("{} Replacement Phase\n\n", side);
            desc.push_str("Use replacements to restore or bring back units.\n");
            desc.push_str("Must be placed in friendly-controlled cities with supply.\n");
            desc.push_str("\nCommands:\n");
            desc.push_str("  REPLACEMENTS         - Show available replacements\n");
            desc.push_str("  REPLACE <unit> [<q>,<r>] - Use replacement\n");
            desc.push_str("  END                  - Finish phase\n");
            desc
        }
        _ => format!("Phase: {}\n\nCommands: Type HELP for command list\n", phase),
    }
}
