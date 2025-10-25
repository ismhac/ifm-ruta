# IFM-Ruta Windows Installation Guide

## Overview

IFM-Ruta is a unified application that provides both MCP server functionality (for Cursor IDE integration) and GUI application (for interactive feedback collection).

## Download

1. Download `ifm-ruta-windows-v0.1.0.zip` from repository
2. Extract the zip file to desired directory

## Installation

### Method 1: Direct Usage (Recommended)
- **No installation required!** Just extract and run the .exe file
- Copy `ifm-ruta-64bit.exe` (or `ifm-ruta-32bit.exe`) to desired location
- Run directly from Command Prompt or double-click

### Method 2: Using Batch Files
- Use `run-mcp-server.bat` to run MCP server
- Use `run-gui.bat` to run GUI application

## Usage

### 1. MCP Server Mode (for Cursor IDE)

```cmd
ifm-ruta-64bit.exe --mcp-server
```

**Configure in Cursor:**
1. Open Cursor Settings
2. Add MCP server configuration:

```json
{
  "mcpServers": {
    "ifm-ruta": {
      "command": "C:\\path\\to\\ifm-ruta-64bit.exe",
      "args": ["--mcp-server"]
    }
  }
}
```

3. Restart Cursor

### 2. GUI Mode (for Interactive Feedback)

```cmd
ifm-ruta-64bit.exe <project_directory> [summary]
```

**Example:**
```cmd
ifm-ruta-64bit.exe C:\my-project "Fix login bug"
```

### 3. Show Help

```cmd
ifm-ruta-64bit.exe
```

## Features

- **Single executable file**: No installation required
- **MCP server**: Full integration with Cursor IDE
- **GUI application**: Graphical interface for feedback collection
- **Vietnamese font support**: Noto Sans font included
- **Conversation history storage**: Automatic conversation storage
- **Project management**: Automatic .gitignore and directory setup

## System Requirements

- Windows 7 or later
- No additional dependencies required
- Works on both Windows 32-bit and 64-bit

## Troubleshooting

### Error "Windows protected your PC"
1. Click "More info"
2. Click "Run anyway"
3. Or add file to Windows Defender exclusions

### GUI not opening
- Ensure Windows has display server running
- Try running from Command Prompt to see errors

### MCP connection failed
- Check path in Cursor configuration
- Ensure .exe file has execution permissions

### Vietnamese fonts not displaying correctly
- Noto Sans font is included
- If still having issues, check system font installation

## Distribution

File `ifm-ruta-windows-v0.1.0.zip` (5.8MB) contains:
- `ifm-ruta-64bit.exe` (6.2MB) - For Windows 64-bit
- `ifm-ruta-32bit.exe` (5.9MB) - For Windows 32-bit
- `README.txt` - Detailed instructions
- `run-mcp-server.bat` - Script to run MCP server
- `run-gui.bat` - Script to run GUI
- `VERSION.txt` - Version information

## Support

- GitHub: https://github.com/ismhac/ifm-ruta
- Issues: https://github.com/ismhac/ifm-ruta/issues

---

**Version:** 0.1.0  
**Build Date:** $(date)  
**Size:** 5.8MB (zipped)