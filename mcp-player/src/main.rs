mod config;
mod game;
mod mcp;

use anyhow::Result;
use mcp::server::McpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = config::Config::load()?;

    // Create and run MCP server
    let server = McpServer::new(config);
    server.run().await?;

    Ok(())
}
