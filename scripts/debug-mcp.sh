#!/bin/bash

# Debug script for MCP connection issues
echo "Debugging MCP Connection Issues..."
echo "=================================="

MCP_SERVER="/home/ismverseinfinity/workspaces/mcp/ifm-ruta/target/release/ifm-ruta"
MCP_CONFIG="/home/ismverseinfinity/.cursor/mcp.json"

echo "1. Checking MCP server executable..."
if [ -f "$MCP_SERVER" ]; then
    echo "MCP server exists: $MCP_SERVER"
    ls -la "$MCP_SERVER"
else
    echo "MCP server not found: $MCP_SERVER"
    exit 1
fi

echo ""
echo "2. Checking MCP configuration..."
if [ -f "$MCP_CONFIG" ]; then
    echo "MCP config exists: $MCP_CONFIG"
    echo "Current configuration:"
    cat "$MCP_CONFIG"
else
    echo "MCP config not found: $MCP_CONFIG"
    exit 1
fi

echo ""
echo "3. Testing MCP server directly..."
echo "Running: echo '{\"jsonrpc\": \"2.0\", \"id\": 1, \"method\": \"initialize\", \"params\": {}}' | $MCP_SERVER --mcp-server"
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | "$MCP_SERVER" --mcp-server

echo ""
echo "4. Checking for running MCP processes..."
ps aux | grep -E "(ifm-ruta|mcp)" | grep -v grep

echo ""
echo "5. Checking Cursor processes..."
ps aux | grep -i cursor | grep -v grep

echo ""
echo "6. Testing MCP server with timeout..."
timeout 10s bash -c 'echo "{\"jsonrpc\": \"2.0\", \"id\": 1, \"method\": \"initialize\", \"params\": {}}" | /home/ismverseinfinity/workspaces/mcp/ifm-ruta/target/release/ifm-ruta --mcp-server'

echo ""
echo "Debug completed!"
echo ""
echo "If MCP server works directly but Cursor can't connect:"
echo "1. Restart Cursor completely (close all windows and reopen)"
echo "2. Check Cursor MCP logs in Developer Tools"
echo "3. Verify file permissions: chmod +x $MCP_SERVER"
echo "4. Try running Cursor from terminal to see error messages"
