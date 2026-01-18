use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum Command {
    // Information
    Help,
    Situation,
    Units { side: Option<String> },
    Threats,
    Rules { topic: String },

    // Queries
    Moves { unit_id: String },
    Attacks,
    Preview { defender: String, attackers: Vec<String> },
    Replacements,

    // Actions
    Move { unit_id: String, to: (i32, i32) },
    Attack { defender: String, attackers: Vec<String> },
    DoneAttacks,
    Resolve,
    Advance { unit_id: String },
    Skip,
    Replace { unit_id: String, hex: Option<(i32, i32)> },
    End,

    // Game
    New,
    Quit,
}

#[derive(Debug)]
pub enum ParseError {
    Empty,
    UnknownCommand(String),
    MissingUnitId,
    MissingHex,
    InvalidHex(String),
    MissingWith,
    MissingAttackers,
    MissingDefender,
    MissingTopic,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "Empty command"),
            ParseError::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
            ParseError::MissingUnitId => write!(f, "Missing unit ID"),
            ParseError::MissingHex => write!(f, "Missing hex coordinates"),
            ParseError::InvalidHex(s) => write!(f, "Invalid hex format: {}", s),
            ParseError::MissingWith => write!(f, "Missing WITH keyword in attack command"),
            ParseError::MissingAttackers => write!(f, "Missing attacker units"),
            ParseError::MissingDefender => write!(f, "Missing defender unit"),
            ParseError::MissingTopic => write!(f, "Missing topic for RULES command"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_command(input: &str) -> Result<Command, ParseError> {
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
        "THREATS" | "T" => Ok(Command::Threats),
        "RULES" | "R" => {
            let topic = parts.get(1).ok_or(ParseError::MissingTopic)?.to_string();
            Ok(Command::Rules { topic })
        }
        "MOVES" | "M" => {
            let unit_id = parts.get(1).ok_or(ParseError::MissingUnitId)?;
            Ok(Command::Moves {
                unit_id: unit_id.to_string(),
            })
        }
        "ATTACKS" | "A" => Ok(Command::Attacks),
        "PREVIEW" | "P" => parse_preview(&parts),
        "REPLACEMENTS" | "REP" => Ok(Command::Replacements),
        "MOVE" => {
            let unit_id = parts.get(1).ok_or(ParseError::MissingUnitId)?;
            let hex = parts.get(2).ok_or(ParseError::MissingHex)?;
            let (q, r) = parse_hex(hex)?;
            Ok(Command::Move {
                unit_id: unit_id.to_string(),
                to: (q, r),
            })
        }
        "ATTACK" => parse_attack(&parts),
        "DONE" => {
            // Check if it's "DONE ATTACKS"
            if parts.len() > 1 && parts[1].to_uppercase() == "ATTACKS" {
                Ok(Command::DoneAttacks)
            } else {
                Ok(Command::End)
            }
        }
        "RESOLVE" => Ok(Command::Resolve),
        "ADVANCE" => {
            let unit_id = parts.get(1).ok_or(ParseError::MissingUnitId)?;
            Ok(Command::Advance {
                unit_id: unit_id.to_string(),
            })
        }
        "SKIP" => Ok(Command::Skip),
        "REPLACE" => {
            let unit_id = parts.get(1).ok_or(ParseError::MissingUnitId)?;
            let hex = parts.get(2).map(|h| parse_hex(h)).transpose()?;
            Ok(Command::Replace {
                unit_id: unit_id.to_string(),
                hex,
            })
        }
        "END" => Ok(Command::End),
        "NEW" => Ok(Command::New),
        "QUIT" | "EXIT" | "Q" => Ok(Command::Quit),
        _ => Err(ParseError::UnknownCommand(parts[0].to_string())),
    }
}

fn parse_side(s: &str) -> Result<String, ParseError> {
    let side = s.to_lowercase();
    match side.as_str() {
        "german" | "g" => Ok("german".to_string()),
        "soviet" | "s" => Ok("soviet".to_string()),
        _ => Ok(side), // Allow any side name
    }
}

fn parse_hex(s: &str) -> Result<(i32, i32), ParseError> {
    // Accept "5,3" or "5 3" or "[5,3]"
    let clean = s.trim_matches(|c| c == '[' || c == ']');
    let parts: Vec<&str> = clean.split(',').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidHex(s.to_string()));
    }
    let q = parts[0]
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidHex(s.to_string()))?;
    let r = parts[1]
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidHex(s.to_string()))?;
    Ok((q, r))
}

fn parse_attack(parts: &[&str]) -> Result<Command, ParseError> {
    // ATTACK <defender> WITH <attacker> [attacker...]
    let with_pos = parts
        .iter()
        .position(|&p| p.eq_ignore_ascii_case("WITH"))
        .ok_or(ParseError::MissingWith)?;

    let defender = parts
        .get(1)
        .ok_or(ParseError::MissingDefender)?
        .to_string();
    let attackers: Vec<String> = parts[with_pos + 1..]
        .iter()
        .map(|s| s.to_string())
        .collect();

    if attackers.is_empty() {
        return Err(ParseError::MissingAttackers);
    }

    Ok(Command::Attack {
        defender,
        attackers,
    })
}

fn parse_preview(parts: &[&str]) -> Result<Command, ParseError> {
    // PREVIEW ATTACK <defender> WITH <attacker> [attacker...]
    // or just: PREVIEW <defender> WITH <attacker> [attacker...]
    let start_idx = if parts.len() > 1 && parts[1].eq_ignore_ascii_case("ATTACK") {
        2
    } else {
        1
    };

    let with_pos = parts
        .iter()
        .skip(start_idx)
        .position(|&p| p.eq_ignore_ascii_case("WITH"))
        .map(|pos| pos + start_idx)
        .ok_or(ParseError::MissingWith)?;

    let defender = parts
        .get(start_idx)
        .ok_or(ParseError::MissingDefender)?
        .to_string();
    let attackers: Vec<String> = parts[with_pos + 1..]
        .iter()
        .map(|s| s.to_string())
        .collect();

    if attackers.is_empty() {
        return Err(ParseError::MissingAttackers);
    }

    Ok(Command::Preview {
        defender,
        attackers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_help() {
        assert!(matches!(parse_command("help"), Ok(Command::Help)));
        assert!(matches!(parse_command("HELP"), Ok(Command::Help)));
        assert!(matches!(parse_command("h"), Ok(Command::Help)));
    }

    #[test]
    fn test_parse_move() {
        match parse_command("move XLVII 4,5") {
            Ok(Command::Move { unit_id, to }) => {
                assert_eq!(unit_id, "XLVII");
                assert_eq!(to, (4, 5));
            }
            _ => panic!("Failed to parse move command"),
        }
    }

    #[test]
    fn test_parse_attack() {
        match parse_command("attack 16A WITH XLVII XL") {
            Ok(Command::Attack {
                defender,
                attackers,
            }) => {
                assert_eq!(defender, "16A");
                assert_eq!(attackers, vec!["XLVII", "XL"]);
            }
            _ => panic!("Failed to parse attack command"),
        }
    }
}
