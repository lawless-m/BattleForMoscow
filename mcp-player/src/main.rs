mod config;
mod game;
mod mcp;
mod text;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use mcp::server::McpServer;
use text::run_text_mode;

#[derive(Parser)]
#[command(name = "mcp-player")]
#[command(about = "Battle for Moscow - Player Interface", long_about = None)]
struct Cli {
    /// Mode to run in
    #[arg(long, default_value = "text")]
    mode: Mode,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    /// Text-based terminal interface
    Text,
    /// MCP (Model Context Protocol) server
    Mcp,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = config::Config::load()?;

    match cli.mode {
        Mode::Text => {
            // Run text mode
            run_text_mode(&config).await?;
        }
        Mode::Mcp => {
            // Run MCP server
            let server = McpServer::new(config);
            server.run().await?;
        }
    }

    Ok(())
}
