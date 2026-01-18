use crate::game::{GameClient, Narrator};
use anyhow::Result;
use serde_json::{json, Value};

pub struct ToolHandler;

impl ToolHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn list_tools(&self) -> Vec<Value> {
        vec![
            // State Query Tools
            json!({
                "name": "get_situation",
                "description": "Get high-level overview of current game state",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            json!({
                "name": "get_units",
                "description": "List units for one or both sides",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "side": {
                            "type": "string",
                            "enum": ["german", "soviet"],
                            "description": "Which side's units to list (omit for both)"
                        }
                    },
                    "required": []
                }
            }),
            json!({
                "name": "get_threats",
                "description": "Analyze dangerous situations and vulnerabilities",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "side": {
                            "type": "string",
                            "enum": ["german", "soviet"],
                            "description": "Which side's perspective (defaults to current player)"
                        }
                    },
                    "required": []
                }
            }),
            // Action Query Tools
            json!({
                "name": "get_valid_moves",
                "description": "List where a unit can move",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "unit_id": {
                            "type": "string",
                            "description": "ID of the unit to check"
                        }
                    },
                    "required": ["unit_id"]
                }
            }),
            json!({
                "name": "preview_attack",
                "description": "Calculate what would happen for a potential attack",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "attackers": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Array of attacking unit IDs"
                        },
                        "defender": {
                            "type": "string",
                            "description": "Defending unit ID"
                        }
                    },
                    "required": ["attackers", "defender"]
                }
            }),
            json!({
                "name": "get_valid_attacks",
                "description": "List all possible attacks this phase",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            // Action Tools
            json!({
                "name": "move_unit",
                "description": "Move a unit to a new position",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "unit_id": {
                            "type": "string",
                            "description": "ID of the unit to move"
                        },
                        "to": {
                            "type": "array",
                            "items": {"type": "integer"},
                            "minItems": 2,
                            "maxItems": 2,
                            "description": "Destination hex coordinates [q, r]"
                        }
                    },
                    "required": ["unit_id", "to"]
                }
            }),
            json!({
                "name": "declare_attacks",
                "description": "Declare all attacks for the combat phase",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "battles": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "attackers": {
                                        "type": "array",
                                        "items": {"type": "string"}
                                    },
                                    "defender": {"type": "string"}
                                },
                                "required": ["attackers", "defender"]
                            },
                            "description": "Array of battles to declare"
                        }
                    },
                    "required": ["battles"]
                }
            }),
            json!({
                "name": "resolve_next_battle",
                "description": "Resolve the next pending battle (server rolls die)",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            json!({
                "name": "advance_unit",
                "description": "Advance a unit into a vacated hex after combat",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "unit_id": {
                            "type": "string",
                            "description": "ID of the unit to advance"
                        }
                    },
                    "required": ["unit_id"]
                }
            }),
            json!({
                "name": "skip_advance",
                "description": "Decline to advance after combat",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            json!({
                "name": "use_replacement",
                "description": "Use a replacement to restore or place a unit",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "unit_id": {
                            "type": "string",
                            "description": "ID of the unit to restore or place"
                        },
                        "hex": {
                            "type": "array",
                            "items": {"type": "integer"},
                            "minItems": 2,
                            "maxItems": 2,
                            "description": "Where to place if bringing back eliminated unit [q, r]"
                        }
                    },
                    "required": ["unit_id"]
                }
            }),
            json!({
                "name": "end_phase",
                "description": "End the current phase and advance to next",
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }),
            // Reference Tools
            json!({
                "name": "get_rules",
                "description": "Explain a game rule or concept",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "topic": {
                            "type": "string",
                            "enum": ["zoc", "combat", "odds", "retreat", "movement", "mud", "replacements", "victory"],
                            "description": "Which rule topic to explain"
                        }
                    },
                    "required": ["topic"]
                }
            }),
        ]
    }

    pub async fn call_tool(
        &self,
        name: &str,
        arguments: &Value,
        client: &GameClient,
        narrator: &Narrator,
    ) -> Result<String> {
        match name {
            // State Query Tools
            "get_situation" => self.get_situation(client, narrator).await,
            "get_units" => {
                let side = arguments["side"].as_str();
                self.get_units(client, narrator, side).await
            }
            "get_threats" => {
                let side = arguments["side"].as_str().unwrap_or("soviet");
                self.get_threats(client, narrator, side).await
            }

            // Action Query Tools
            "get_valid_moves" => {
                let unit_id = arguments["unit_id"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing unit_id"))?;
                self.get_valid_moves(client, narrator, unit_id).await
            }
            "preview_attack" => {
                let attackers = arguments["attackers"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Missing attackers"))?;
                let defender = arguments["defender"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing defender"))?;
                self.preview_attack(client, narrator, attackers, defender)
                    .await
            }
            "get_valid_attacks" => self.get_valid_attacks(client, narrator).await,

            // Action Tools
            "move_unit" => {
                let unit_id = arguments["unit_id"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing unit_id"))?;
                let to = arguments["to"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Missing to coordinates"))?;
                let q = to[0].as_i64().ok_or_else(|| anyhow::anyhow!("Invalid q"))?;
                let r = to[1].as_i64().ok_or_else(|| anyhow::anyhow!("Invalid r"))?;
                self.move_unit(client, narrator, unit_id, (q as i32, r as i32))
                    .await
            }
            "declare_attacks" => {
                let battles = arguments["battles"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Missing battles"))?;
                self.declare_attacks(client, narrator, battles).await
            }
            "resolve_next_battle" => self.resolve_next_battle(client, narrator).await,
            "advance_unit" => {
                let unit_id = arguments["unit_id"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing unit_id"))?;
                self.advance_unit(client, narrator, unit_id).await
            }
            "skip_advance" => self.skip_advance(client, narrator).await,
            "use_replacement" => {
                let unit_id = arguments["unit_id"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing unit_id"))?;
                let hex = if let Some(hex_array) = arguments["hex"].as_array() {
                    let q = hex_array[0]
                        .as_i64()
                        .ok_or_else(|| anyhow::anyhow!("Invalid q"))?;
                    let r = hex_array[1]
                        .as_i64()
                        .ok_or_else(|| anyhow::anyhow!("Invalid r"))?;
                    Some((q as i32, r as i32))
                } else {
                    None
                };
                self.use_replacement(client, narrator, unit_id, hex).await
            }
            "end_phase" => self.end_phase(client, narrator).await,

            // Reference Tools
            "get_rules" => {
                let topic = arguments["topic"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing topic"))?;
                Ok(narrator.narrate_rules(topic))
            }

            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }

    // State Query Tool Implementations
    async fn get_situation(&self, client: &GameClient, narrator: &Narrator) -> Result<String> {
        let state = client.get_state().await?;
        Ok(narrator.narrate_situation(&state))
    }

    async fn get_units(
        &self,
        client: &GameClient,
        narrator: &Narrator,
        side: Option<&str>,
    ) -> Result<String> {
        let state = client.get_state().await?;
        Ok(narrator.narrate_units(&state, side))
    }

    async fn get_threats(
        &self,
        client: &GameClient,
        narrator: &Narrator,
        side: &str,
    ) -> Result<String> {
        let state = client.get_state().await?;
        Ok(narrator.narrate_threats(&state, side))
    }

    // Action Query Tool Implementations
    async fn get_valid_moves(
        &self,
        client: &GameClient,
        narrator: &Narrator,
        unit_id: &str,
    ) -> Result<String> {
        // TODO: Call backend API to get valid moves
        let moves = json!([]);
        Ok(narrator.narrate_valid_moves(unit_id, &moves))
    }

    async fn preview_attack(
        &self,
        client: &GameClient,
        narrator: &Narrator,
        attackers: &[Value],
        defender: &str,
    ) -> Result<String> {
        // TODO: Call backend API to preview attack
        let preview = json!({
            "attack_strength": 0,
            "defense_strength": 0,
            "odds": "1:1"
        });
        Ok(narrator.narrate_attack_preview(&preview))
    }

    async fn get_valid_attacks(&self, client: &GameClient, narrator: &Narrator) -> Result<String> {
        // TODO: Call backend API to get valid attacks
        Ok("POSSIBLE ATTACKS THIS PHASE:\n\nAnalyzing...".to_string())
    }

    // Action Tool Implementations
    async fn move_unit(
        &self,
        client: &GameClient,
        narrator: &Narrator,
        unit_id: &str,
        to: (i32, i32),
    ) -> Result<String> {
        let result = client.move_unit(unit_id, to).await?;
        Ok(format!("MOVED: {} to [{},{}]\n", unit_id, to.0, to.1))
    }

    async fn declare_attacks(
        &self,
        client: &GameClient,
        narrator: &Narrator,
        battles: &[Value],
    ) -> Result<String> {
        let result = client.declare_attacks(battles.to_vec()).await?;
        Ok(format!("ATTACKS DECLARED:\n\n{} battles ready to resolve.\n", battles.len()))
    }

    async fn resolve_next_battle(
        &self,
        client: &GameClient,
        narrator: &Narrator,
    ) -> Result<String> {
        let result = client.resolve_next_battle().await?;
        Ok(narrator.narrate_combat_result(&result))
    }

    async fn advance_unit(
        &self,
        client: &GameClient,
        narrator: &Narrator,
        unit_id: &str,
    ) -> Result<String> {
        let result = client.advance_unit(unit_id).await?;
        Ok(format!("ADVANCED: {}\n", unit_id))
    }

    async fn skip_advance(&self, client: &GameClient, narrator: &Narrator) -> Result<String> {
        let result = client.skip_advance().await?;
        Ok("Advance declined.\n".to_string())
    }

    async fn use_replacement(
        &self,
        client: &GameClient,
        narrator: &Narrator,
        unit_id: &str,
        hex: Option<(i32, i32)>,
    ) -> Result<String> {
        let result = client.use_replacement(unit_id, hex).await?;
        Ok(format!("REPLACEMENT USED: {}\n", unit_id))
    }

    async fn end_phase(&self, client: &GameClient, narrator: &Narrator) -> Result<String> {
        let result = client.end_phase().await?;
        Ok("PHASE COMPLETE\n\nAdvanced to next phase.\n".to_string())
    }
}
