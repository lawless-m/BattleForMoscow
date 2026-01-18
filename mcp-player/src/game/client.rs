use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

/// HTTP client for communicating with the Battle for Moscow game engine
pub struct GameClient {
    client: Client,
    base_url: String,
}

impl GameClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Get the current game state
    pub async fn get_state(&self) -> Result<Value> {
        let url = format!("{}/state", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.json().await?)
    }

    /// Move a unit
    pub async fn move_unit(&self, unit_id: &str, to: (i32, i32)) -> Result<Value> {
        let url = format!("{}/move", self.base_url);
        let body = serde_json::json!({
            "unit_id": unit_id,
            "to": to,
        });
        let response = self.client.post(&url).json(&body).send().await?;
        Ok(response.json().await?)
    }

    /// Declare attacks
    pub async fn declare_attacks(&self, battles: Vec<Value>) -> Result<Value> {
        let url = format!("{}/declare_attacks", self.base_url);
        let body = serde_json::json!({ "battles": battles });
        let response = self.client.post(&url).json(&body).send().await?;
        Ok(response.json().await?)
    }

    /// Resolve next battle
    pub async fn resolve_next_battle(&self) -> Result<Value> {
        let url = format!("{}/resolve_battle", self.base_url);
        let response = self.client.post(&url).send().await?;
        Ok(response.json().await?)
    }

    /// End current phase
    pub async fn end_phase(&self) -> Result<Value> {
        let url = format!("{}/end_phase", self.base_url);
        let response = self.client.post(&url).send().await?;
        Ok(response.json().await?)
    }

    /// Use a replacement
    pub async fn use_replacement(&self, unit_id: &str, hex: Option<(i32, i32)>) -> Result<Value> {
        let url = format!("{}/replacement", self.base_url);
        let mut body = serde_json::json!({ "unit_id": unit_id });
        if let Some(hex) = hex {
            body["hex"] = serde_json::json!(hex);
        }
        let response = self.client.post(&url).json(&body).send().await?;
        Ok(response.json().await?)
    }

    /// Advance unit after combat
    pub async fn advance_unit(&self, unit_id: &str) -> Result<Value> {
        let url = format!("{}/advance", self.base_url);
        let body = serde_json::json!({ "unit_id": unit_id });
        let response = self.client.post(&url).json(&body).send().await?;
        Ok(response.json().await?)
    }

    /// Skip advance
    pub async fn skip_advance(&self) -> Result<Value> {
        let url = format!("{}/skip_advance", self.base_url);
        let response = self.client.post(&url).send().await?;
        Ok(response.json().await?)
    }
}
