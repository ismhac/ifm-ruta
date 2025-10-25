#!/bin/bash

# Script to restart MCP connection
echo "Restarting MCP Connection..."
echo "============================"

# Kill any existing MCP processes
echo "1. Stopping existing MCP processes..."
pkill -f "ifm-ruta.*--mcp-server" || echo "No existing MCP processes found"

# Wait a moment
sleep 2

# Verify MCP server is working
echo ""
echo "2. Testing MCP server..."
MCP_SERVER="/home/ismverseinfinity/workspaces/mcp/ifm-ruta/target/release/ifm-ruta"
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | "$MCP_SERVER" --mcp-server

echo ""
echo "3. MCP server is ready!"
echo ""
echo "Next steps:"
echo "1. In Cursor, open Command Palette (Ctrl+Shift+P)"
echo "2. Type 'MCP' and look for MCP-related commands"
echo "3. Try 'MCP: Restart Servers' or similar"
echo "4. Or restart Cursor completely"
echo ""
echo "Alternative: Check Cursor MCP status in:"
echo "- View > Output > MCP"
echo "- Developer Tools (F12) > Console"
