//! MCP server implementation

use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

use ifm_ruta_core::{
    models::AppError,
    traits::{EventBus, ProcessManager, SettingsManager, Tool},
};

/// MCP Request struct like Go
#[derive(Debug, serde::Deserialize)]
pub struct MCPRequest {
    #[allow(dead_code)]
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

/// MCP Response struct like Go
#[derive(Debug, serde::Serialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

/// MCP Error struct like Go
#[derive(Debug, serde::Serialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
}

/// MCP server
pub struct MCPServer {
    tools: HashMap<String, Box<dyn Tool>>,
    #[allow(dead_code)]
    settings_manager: Arc<dyn SettingsManager>,
    #[allow(dead_code)]
    process_manager: Arc<dyn ProcessManager>,
    #[allow(dead_code)]
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

    /// Handle a request like Go
    pub fn handle_request(&self, request: MCPRequest) -> Result<Option<MCPResponse>, AppError> {
        // Check if this is a notification (no id field)
        if request.id.is_none() {
            // Handle notifications silently (no response needed per JSON-RPC 2.0 spec)
            match request.method.as_str() {
                "notifications/initialized" => {
                    // Cursor sends this after initialize - just ignore
                    return Ok(None);
                }
                _ => {
                    // Unknown notification - ignore silently
                    return Ok(None);
                }
            }
        }

        // This is a request (has id) - send response
        let response = match request.method.as_str() {
            "initialize" => self.handle_initialize(request),
            "tools/list" => self.handle_tools_list(request),
            "tools/call" => self.handle_tool_call(request),
            _ => Ok(MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: "Method not found".to_string(),
                }),
            }),
        };

        response.map(Some)
    }

    /// Handle initialize request like Go
    fn handle_initialize(&self, request: MCPRequest) -> Result<MCPResponse, AppError> {
        Ok(MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {
                        "listChanged": true
                    }
                },
                "serverInfo": {
                    "name": "interactive-feedback-mcp",
                    "version": "1.0.0"
                }
            })),
            error: None,
        })
    }

    /// Handle tools/list request like Go
    fn handle_tools_list(&self, request: MCPRequest) -> Result<MCPResponse, AppError> {
        let tools: Vec<Value> = self
            .tools
            .values()
            .map(|tool| {
                json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "inputSchema": tool.input_schema()
                })
            })
            .collect();

        Ok(MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(json!({
                "tools": tools
            })),
            error: None,
        })
    }

    /// Handle tools/call request like Go
    fn handle_tool_call(&self, request: MCPRequest) -> Result<MCPResponse, AppError> {
        let params = request.params.unwrap_or(json!({}));
        let tool_name = params
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("Missing tool name")))?;

        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        let tool = self.tools.get(tool_name).ok_or_else(|| {
            AppError::InternalError(anyhow::anyhow!("Tool not found: {}", tool_name))
        })?;

        // Execute the tool
        let tool_result = tool.execute(arguments)?;
        let result_json = serde_json::to_string(&tool_result)?;

        Ok(MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(json!({
                "content": [{
                    "type": "text",
                    "text": result_json
                }]
            })),
            error: None,
        })
    }
}
