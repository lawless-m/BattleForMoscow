use crate::config::Config;
use crate::game::{GameClient, Narrator};
use crate::mcp::tools::ToolHandler;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

pub struct McpServer {
    config: Config,
    game_client: GameClient,
    narrator: Narrator,
    tool_handler: ToolHandler,
}

impl McpServer {
    pub fn new(config: Config) -> Self {
        let game_client = GameClient::new(config.game.api_url.clone());
        let narrator = Narrator::new(
            config.display.coordinate_format.clone(),
            config.display.verbosity.clone(),
        );
        let tool_handler = ToolHandler::new();

        Self {
            config,
            game_client,
            narrator,
            tool_handler,
        }
    }

    pub async fn run(&self) -> Result<()> {
        eprintln!("Battle for Moscow MCP Server starting...");
        eprintln!("API URL: {}", self.config.game.api_url);
        eprintln!("Player side: {}", self.config.player.side);

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            eprintln!("Received: {}", line);

            match serde_json::from_str::<JsonRpcRequest>(&line) {
                Ok(req) => {
                    let response = self.handle_request(req).await;
                    let response_str = serde_json::to_string(&response)?;
                    writeln!(stdout, "{}", response_str)?;
                    stdout.flush()?;
                }
                Err(e) => {
                    eprintln!("Failed to parse request: {}", e);
                    let error_response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(json!({
                            "code": -32700,
                            "message": "Parse error"
                        })),
                    };
                    let response_str = serde_json::to_string(&error_response)?;
                    writeln!(stdout, "{}", response_str)?;
                    stdout.flush()?;
                }
            }
        }

        Ok(())
    }

    async fn handle_request(&self, req: JsonRpcRequest) -> JsonRpcResponse {
        eprintln!("Handling method: {}", req.method);

        let result = match req.method.as_str() {
            "initialize" => self.handle_initialize(req.params.as_ref()),
            "tools/list" => self.handle_list_tools(),
            "tools/call" => self.handle_call_tool(req.params.as_ref()).await,
            "resources/list" => self.handle_list_resources(),
            "prompts/list" => self.handle_list_prompts(),
            _ => Err(anyhow::anyhow!("Unknown method: {}", req.method)),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(value),
                error: None,
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(json!({
                    "code": -32603,
                    "message": format!("Internal error: {}", e)
                })),
            },
        }
    }

    fn handle_initialize(&self, _params: Option<&Value>) -> Result<Value> {
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {},
                "resources": {},
                "prompts": {}
            },
            "serverInfo": {
                "name": "battle-for-moscow-mcp",
                "version": "0.1.0"
            }
        }))
    }

    fn handle_list_tools(&self) -> Result<Value> {
        let tools = self.tool_handler.list_tools();
        Ok(json!({ "tools": tools }))
    }

    async fn handle_call_tool(&self, params: Option<&Value>) -> Result<Value> {
        let params = params.ok_or_else(|| anyhow::anyhow!("Missing params"))?;

        let tool_name = params["name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing tool name"))?;

        let empty_args = json!({});
        let arguments = params.get("arguments").unwrap_or(&empty_args);

        eprintln!("Calling tool: {} with args: {}", tool_name, arguments);

        let result = self
            .tool_handler
            .call_tool(tool_name, arguments, &self.game_client, &self.narrator)
            .await?;

        Ok(json!({
            "content": [{
                "type": "text",
                "text": result
            }]
        }))
    }

    fn handle_list_resources(&self) -> Result<Value> {
        Ok(json!({
            "resources": [
                {
                    "uri": "bfm://rules/full",
                    "name": "Complete Rules",
                    "description": "Full game rules reference"
                },
                {
                    "uri": "bfm://rules/crt",
                    "name": "Combat Results Table",
                    "description": "Combat outcomes by odds and die roll"
                }
            ]
        }))
    }

    fn handle_list_prompts(&self) -> Result<Value> {
        Ok(json!({
            "prompts": [
                {
                    "name": "play_german",
                    "description": "Play as German commander",
                    "arguments": []
                },
                {
                    "name": "play_soviet",
                    "description": "Play as Soviet commander",
                    "arguments": []
                }
            ]
        }))
    }
}
