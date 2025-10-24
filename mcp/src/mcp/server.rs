//! MCP server implementation

use std::collections::HashMap;
use std::sync::Arc;
use std::io::{self, BufRead};
use serde_json::{Value, json};

use ifm_ruta_core::{
    traits::{Tool, ToolError, SettingsManager, ProcessManager, EventBus},
    models::AppError,
};

use crate::mcp::protocol::MCPRequest;

/// MCP server
pub struct MCPServer {
    tools: HashMap<String, Box<dyn Tool>>,
    settings_manager: Arc<dyn SettingsManager>,
    process_manager: Arc<dyn ProcessManager>,
    event_bus: Arc<dyn EventBus>,
}

impl MCPServer {
    /// Create a new MCP server
    pub fn new(
        settings_manager: Arc<dyn SettingsManager>,
        process_manager: Arc<dyn ProcessManager>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            tools: HashMap::new(),
            settings_manager,
            process_manager,
            event_bus,
        }
    }
    
    /// Register a tool
    pub fn register_tool(&mut self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
    }
    
    /// Run the MCP server
    pub fn run(&mut self) -> Result<(), AppError> {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin);
        let mut line = String::new();
        
        while reader.read_line(&mut line)? > 0 {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            
            // Parse the request
            let request: MCPRequest = serde_json::from_str(trimmed)?;
            
            // Handle the request
            let response = self.handle_request(request)?;
            
            // Send the response
            println!("{}", serde_json::to_string(&response)?);
            
            line.clear();
        }
        
        Ok(())
    }
    
    /// Handle a request
    fn handle_request(&self, request: MCPRequest) -> Result<Value, AppError> {
        match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params),
            "tools/list" => self.handle_tools_list(),
            "tools/call" => self.handle_tool_call(request.params),
            _ => Err(AppError::InternalError(anyhow::anyhow!("Unknown method: {}", request.method))),
        }
    }
    
    /// Handle initialize request
    fn handle_initialize(&self, _params: Value) -> Result<Value, AppError> {
        Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "ifm-ruta-mcp",
                "version": "0.1.0"
            }
        }))
    }
    
    /// Handle tools/list request
    fn handle_tools_list(&self) -> Result<Value, AppError> {
        let tools: Vec<Value> = self.tools
            .values()
            .map(|tool| {
                json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "inputSchema": tool.input_schema()
                })
            })
            .collect();
        
        Ok(json!({
            "tools": tools
        }))
    }
    
    /// Handle tools/call request
    fn handle_tool_call(&self, params: Value) -> Result<Value, AppError> {
        let tool_name = params.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Missing tool name")))?;
        
        let arguments = params.get("arguments")
            .cloned()
            .unwrap_or(json!({}));
        
        let tool = self.tools.get(tool_name)
            .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Tool not found: {}", tool_name)))?;
        
        // Execute the tool
        let result = tool.execute(arguments)?;
        
        Ok(json!({
            "content": [{
                "type": "text",
                "text": serde_json::to_string(&result)?
            }]
        }))
    }
}
