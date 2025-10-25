#!/bin/bash

# Test script for MCP server
echo "Testing IFM-Ruta MCP Server..."
echo "================================"

MCP_SERVER="/home/ismverseinfinity/workspaces/mcp/ifm-ruta/target/release/ifm-ruta"

# Test 1: Initialize
echo "1. Testing initialize..."
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | "$MCP_SERVER" --mcp-server
echo ""

# Test 2: List tools
echo "2. Testing tools/list..."
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}' | "$MCP_SERVER" --mcp-server
echo ""

# Test 3: Call interactive_feedback tool
echo "3. Testing tools/call (interactive_feedback)..."
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "interactive_feedback", "arguments": {"projectDirectory": "/tmp/test", "prompt": "Test prompt", "previousUserRequest": "Test request"}}}' | "$MCP_SERVER" --mcp-server
echo ""

echo "MCP Server tests completed!"
echo ""
echo "If all tests passed, the MCP server is working correctly."
echo "If Cursor still shows connection errors, try:"
echo "1. Restart Cursor completely"
echo "2. Check Cursor MCP logs"
echo "3. Verify the path in mcp.json is correct"
