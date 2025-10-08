# Brain Engine MCP Server

A simple MCP (Model Context Protocol) server that provides tools for Claude Desktop.

## Building

```bash
cargo build --release --package brain-engine-mcp
```

The binary will be located at `target/release/brain-engine-mcp`.

## Available Tools

- **sum**: Add two integers together
  - Parameters: `a` (number), `b` (number)
  - Returns: The sum of the two numbers

## Configuration

To use this MCP server with Claude Desktop, add the following to your Claude Desktop configuration file:

### macOS
Edit `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "brain-engine": {
      "command": "/absolute/path/to/brain-engine/target/release/brain-engine-mcp",
      "args": []
    }
  }
}
```

### Linux
Edit `~/.config/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "brain-engine": {
      "command": "/absolute/path/to/brain-engine/target/release/brain-engine-mcp",
      "args": []
    }
  }
}
```

### Windows
Edit `%APPDATA%\Claude\claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "brain-engine": {
      "command": "C:\\absolute\\path\\to\\brain-engine\\target\\release\\brain-engine-mcp.exe",
      "args": []
    }
  }
}
```

**Important**: Replace `/absolute/path/to/brain-engine` with the actual absolute path to your project.

## Usage

After configuring Claude Desktop and restarting it, you can use the tool by asking Claude to use it:

"Can you use the sum tool to add 5 and 3?"

Claude will call the MCP server and return the result.

## Testing Manually

You can test the server manually using stdin/stdout:

```bash
cargo run --package brain-engine-mcp
```

Then type JSON-RPC requests:

```json
{"jsonrpc":"2.0","id":1,"method":"initialize"}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"sum","arguments":{"a":5,"b":3}}}
```

Press Ctrl+D (or Ctrl+Z on Windows) when done.