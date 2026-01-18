use crate::config::Config;
use crate::game::client::GameClient;
use crate::game::narrator::Narrator;
use crate::text::commands::{parse_command, Command, ParseError};
use crate::text::prompts::{print_phase_prompt, HELP_TEXT};
use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use serde_json::Value;

pub struct CommandOutput {
    pub text: String,
    pub phase_changed: bool,
}

impl CommandOutput {
    pub fn text(text: String) -> Self {
        Self {
            text,
            phase_changed: false,
        }
    }

    pub fn with_phase_change(text: String) -> Self {
        Self {
            text,
            phase_changed: true,
        }
    }
}

pub async fn run_text_mode(config: &Config) -> Result<()> {
    let client = GameClient::new(config.game.api_url.clone());
    let narrator = Narrator::new(
        config.display.coordinate_format.clone(),
        config.display.verbosity.clone(),
    );

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║           BATTLE FOR MOSCOW - Text Mode                   ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    println!("Type HELP for commands.\n");

    // Show initial state
    match client.get_state().await {
        Ok(state) => {
            println!("{}", print_phase_prompt(&state, &narrator));
        }
        Err(e) => {
            eprintln!("Error connecting to game server: {}", e);
            eprintln!("\nMake sure the game server is running at: {}", config.game.api_url);
            return Ok(());
        }
    }

    let mut rl = DefaultEditor::new()?;

    loop {
        let prompt = "> ";
        match rl.readline(prompt) {
            Ok(line) => {
                if !line.trim().is_empty() {
                    let _ = rl.add_history_entry(&line);
                }

                match parse_command(&line) {
                    Ok(cmd) => match execute_command(&client, &narrator, cmd).await {
                        Ok(output) => {
                            println!("{}\n", output.text);

                            // Show new phase prompt if phase changed
                            if output.phase_changed {
                                match client.get_state().await {
                                    Ok(state) => {
                                        println!("{}", print_phase_prompt(&state, &narrator));
                                    }
                                    Err(e) => eprintln!("Error getting state: {}\n", e),
                                }
                            }
                        }
                        Err(e) => println!("Error: {}\n", e),
                    },
                    Err(ParseError::Empty) => continue,
                    Err(e) => println!("Parse error: {}\n", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

async fn execute_command(
    client: &GameClient,
    narrator: &Narrator,
    cmd: Command,
) -> Result<CommandOutput> {
    match cmd {
        Command::Help => Ok(CommandOutput::text(HELP_TEXT.to_string())),

        Command::Situation => {
            let state = client.get_state().await?;
            Ok(CommandOutput::text(narrator.narrate_situation(&state)))
        }

        Command::Units { side } => {
            let state = client.get_state().await?;
            let side_str = side.as_deref();
            Ok(CommandOutput::text(narrator.narrate_units(&state, side_str)))
        }

        Command::Threats => {
            let state = client.get_state().await?;
            // Determine current side from phase
            let phase = state["phase"].as_str().unwrap_or("");
            let side = if phase.contains("German") {
                "german"
            } else {
                "soviet"
            };
            Ok(CommandOutput::text(narrator.narrate_threats(&state, side)))
        }

        Command::Rules { topic } => Ok(CommandOutput::text(narrator.narrate_rules(&topic))),

        Command::Moves { unit_id } => {
            let state = client.get_state().await?;
            // For now, return a placeholder - actual API endpoint would be needed
            Ok(CommandOutput::text(format!(
                "Valid moves for {}: [Feature requires API endpoint]\n\
                 The game engine needs to expose a /valid_moves endpoint.",
                unit_id
            )))
        }

        Command::Attacks => {
            let state = client.get_state().await?;
            Ok(CommandOutput::text(format!(
                "Possible attacks: [Feature requires API endpoint]\n\
                 The game engine needs to expose an /available_attacks endpoint."
            )))
        }

        Command::Preview { defender, attackers } => Ok(CommandOutput::text(format!(
            "Attack preview: {} vs {}\n\
             [Feature requires API endpoint]\n\
             The game engine needs to expose a /preview_attack endpoint.",
            attackers.join(" + "),
            defender
        ))),

        Command::Replacements => {
            let state = client.get_state().await?;
            Ok(CommandOutput::text(format!(
                "Available replacements: [Feature requires API endpoint]\n\
                 The game engine needs to expose a /replacements endpoint."
            )))
        }

        Command::Move { unit_id, to } => {
            let result = client.move_unit(&unit_id, to).await?;
            if result["success"].as_bool().unwrap_or(false) {
                Ok(CommandOutput::text(format!(
                    "MOVED: {} → [{},{}]",
                    unit_id, to.0, to.1
                )))
            } else {
                let error = result["error"].as_str().unwrap_or("Unknown error");
                Ok(CommandOutput::text(format!("Move failed: {}", error)))
            }
        }

        Command::Attack { defender, attackers } => {
            // Create battle declaration
            let battle = serde_json::json!({
                "defender": defender,
                "attackers": attackers,
            });
            let result = client.declare_attacks(vec![battle]).await?;

            if result["success"].as_bool().unwrap_or(false) {
                Ok(CommandOutput::text(format!(
                    "DECLARED: {} vs {}\n\n\
                     More attacks? Or DONE ATTACKS to resolve.",
                    attackers.join(" + "),
                    defender
                )))
            } else {
                let error = result["error"].as_str().unwrap_or("Unknown error");
                Ok(CommandOutput::text(format!("Attack failed: {}", error)))
            }
        }

        Command::DoneAttacks => Ok(CommandOutput::text(
            "Attacks finalized. Use RESOLVE to resolve each battle.".to_string(),
        )),

        Command::Resolve => {
            let result = client.resolve_next_battle().await?;
            let output = narrator.narrate_combat_result(&result);

            // Check if advance is available
            if result["can_advance"].as_bool().unwrap_or(false) {
                Ok(CommandOutput::text(format!(
                    "{}\n\nADVANCE <unit> or SKIP",
                    output
                )))
            } else {
                Ok(CommandOutput::text(output))
            }
        }

        Command::Advance { unit_id } => {
            let result = client.advance_unit(&unit_id).await?;
            if result["success"].as_bool().unwrap_or(false) {
                Ok(CommandOutput::text(format!("Advanced: {}", unit_id)))
            } else {
                let error = result["error"].as_str().unwrap_or("Unknown error");
                Ok(CommandOutput::text(format!("Advance failed: {}", error)))
            }
        }

        Command::Skip => {
            let result = client.skip_advance().await?;
            if result["success"].as_bool().unwrap_or(false) {
                Ok(CommandOutput::text("Advance skipped.".to_string()))
            } else {
                let error = result["error"].as_str().unwrap_or("Unknown error");
                Ok(CommandOutput::text(format!("Skip failed: {}", error)))
            }
        }

        Command::Replace { unit_id, hex } => {
            let result = client.use_replacement(&unit_id, hex).await?;
            if result["success"].as_bool().unwrap_or(false) {
                Ok(CommandOutput::text(format!("Replacement used: {}", unit_id)))
            } else {
                let error = result["error"].as_str().unwrap_or("Unknown error");
                Ok(CommandOutput::text(format!(
                    "Replacement failed: {}",
                    error
                )))
            }
        }

        Command::End => {
            let old_state = client.get_state().await?;
            let old_phase = old_state["phase"].as_str().unwrap_or("");

            let result = client.end_phase().await?;

            if result["success"].as_bool().unwrap_or(false) {
                let new_state = client.get_state().await?;
                let new_phase = new_state["phase"].as_str().unwrap_or("");

                Ok(CommandOutput::with_phase_change(
                    "Phase complete.".to_string(),
                ))
            } else {
                let error = result["error"].as_str().unwrap_or("Unknown error");
                Ok(CommandOutput::text(format!("End phase failed: {}", error)))
            }
        }

        Command::New => Ok(CommandOutput::text(
            "New game: [Feature requires API endpoint]\n\
             The game engine needs to expose a /new_game endpoint."
                .to_string(),
        )),

        Command::Quit => {
            println!("Goodbye!");
            std::process::exit(0);
        }
    }
}
