//! MCP server for Interactive Feedback MCP

use std::io::{self, BufRead};

use ifm_ruta_core::{
    models::AppError,
    services::{SettingsManagerImpl, ProcessManagerImpl, EventBusImpl},
    utils::init_logging,
};

mod mcp;
mod tools;

use mcp::MCPServer;
use mcp::server::MCPRequest;
use tools::InteractiveFeedbackTool;

/// Main entry point
fn main() -> Result<(), AppError> {
    // Initialize logging
    init_logging(tracing::Level::INFO)?;
    
    // Initialize core services
    let settings_manager = std::sync::Arc::new(SettingsManagerImpl::new());
    let process_manager = std::sync::Arc::new(ProcessManagerImpl::new());
    let event_bus = std::sync::Arc::new(EventBusImpl::new());
    
    // Create MCP server
    let mut server = MCPServer::new(settings_manager, process_manager, event_bus);
    
    // Register tools
    server.register_tool(Box::new(InteractiveFeedbackTool::new()));
    
    // Run the server with stdin/stdout like Go
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    while let Some(line) = lines.next() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Parse JSON request like Go
        let request: MCPRequest = serde_json::from_str(&line)?;
        
        // Handle request and send response like Go
        if let Some(response) = server.handle_request(request)? {
            println!("{}", serde_json::to_string(&response)?);
        }
        // Notifications don't get responses (per JSON-RPC 2.0 spec)
    }
    
    Ok(())
}
