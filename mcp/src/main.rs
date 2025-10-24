//! MCP server for Interactive Feedback MCP

use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use serde_json::Value;

use ifm_ruta_core::{
    traits::{Tool, ToolError},
    models::{Feedback, AppError},
    services::{SettingsManagerImpl, ProcessManagerImpl, EventBusImpl},
    utils::init_logging,
};

mod mcp;
mod tools;

use mcp::MCPServer;
use tools::InteractiveFeedbackTool;

/// Main entry point
fn main() -> Result<(), AppError> {
    // Initialize logging
    init_logging(tracing::Level::INFO)?;
    
    // Initialize core services
    let settings_manager = Arc::new(SettingsManagerImpl::new());
    let process_manager = Arc::new(ProcessManagerImpl::new());
    let event_bus = Arc::new(EventBusImpl::new());
    
    // Create MCP server
    let mut server = MCPServer::new(settings_manager, process_manager, event_bus);
    
    // Register tools
    server.register_tool(Box::new(InteractiveFeedbackTool::new()));
    
    // Run the server
    server.run()?;
    
    Ok(())
}
