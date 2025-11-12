# ASON MCP Server for Zed

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Node.js](https://img.shields.io/badge/Node.js-v18+-green.svg)](https://nodejs.org/)

Model Context Protocol (MCP) extension for Zed that provides ASON compression/decompression tools in the AI assistant. Reduce token usage by 20-60% for LLM interactions while maintaining 100% lossless round-trip fidelity.

## ✨ Features

- **🤖 AI Assistant Integration**: Use ASON tools directly in Zed's AI chat
- **📦 Zero Configuration**: Works out of the box with `npx`
- **🔄 Auto-Update**: Always uses the latest version of ASON MCP server
- **⚡ Fast**: Rust-based extension with minimal overhead

### Available Tools

- `compress_json` - Compress JSON to ASON format (20-60% token reduction)
- `decompress_ason` - Decompress ASON back to JSON (lossless)
- `get_compression_stats` - Analyze compression metrics
- `configure_compressor` - Customize compression settings

## 📦 Installation

### Prerequisites

- [Zed Editor](https://zed.dev) (latest version)
- [Node.js](https://nodejs.org) v18+ (for npx)
- [Rust](https://rustup.rs) (only for development)

### Option 1: From Zed Extensions (Recommended)

*Coming soon - pending submission to Zed extensions repository*

1. Open Zed
2. Press `Cmd+Shift+X` (Extensions)
3. Search for "ASON MCP Server"
4. Click **Install**

### Option 2: Local Development Installation

```bash
# Clone the repository
git clone https://github.com/ason-format/zed-extension
cd zed-extension

# Install Rust dependencies and build
cargo build --release

# Install as dev extension in Zed
# In Zed: Cmd+Shift+P → "zed: install dev extension" → Select this folder
```

## 🚀 Usage

Once installed, the ASON MCP server is available in Zed's AI assistant:

### Compress JSON

```
Ask the assistant: "Compress this JSON using ASON"
{
  "users": [
    {"id": 1, "name": "Alice", "age": 25},
    {"id": 2, "name": "Bob", "age": 30}
  ]
}
```

The assistant will use the `compress_json` tool and return:

```
users:[2]@id,name,age
1,Alice,25
2,Bob,30
```

### Decompress ASON

```
Ask the assistant: "Decompress this ASON to JSON"
users:[2]@id,name,age
1,Alice,25
2,Bob,30
```

Returns the original JSON structure.

### Get Compression Stats

```
Ask the assistant: "Show me compression stats for this JSON"
{"data": [...]}
```

Returns detailed metrics:
- Original vs compressed tokens
- Byte size comparison
- Reduction percentage

## 🔧 Configuration

### Manual Configuration (Optional)

If you prefer manual configuration, add to your Zed `settings.json`:

```json
{
  "context_servers": {
    "ason": {
      "command": "npx",
      "args": ["-y", "@ason-format/mcp-server@latest"]
    }
  }
}
```

### Custom MCP Server Path

To use a specific version or local installation:

```json
{
  "context_servers": {
    "ason": {
      "command": "node",
      "args": ["/path/to/mcp-server/dist/index.js"]
    }
  }
}
```

## 📊 How It Works

This extension provides a thin Rust wrapper around the [@ason-format/mcp-server](https://www.npmjs.com/package/@ason-format/mcp-server) npm package:

1. **Extension loads** → Rust code returns the command to start MCP server
2. **MCP server starts** → Uses `npx` to run latest version
3. **Tools available** → AI assistant can use ASON compression tools
4. **Zero maintenance** → Always uses latest npm package

## 🏗️ Architecture

```
Zed Editor
    ↓ loads
Rust Extension (this repo)
    ↓ executes
npx @ason-format/mcp-server@latest
    ↓ provides
MCP Tools (compress/decompress/stats/configure)
    ↓ uses
@ason-format/ason (core library)
```

## 🛠️ Development

### Requirements

- Rust (via [rustup](https://rustup.rs))
- Node.js v18+

### Build

```bash
# Build the extension
cargo build --release

# Test locally in Zed
# Cmd+Shift+P → "zed: install dev extension"
```

### Project Structure

```
zed-extension/
├── Cargo.toml          # Rust dependencies
├── extension.toml      # Zed extension manifest
├── package.json        # npm package metadata
├── src/
│   └── lib.rs         # Extension implementation
├── scripts/
│   └── release.sh     # Release automation
└── README.md
```

## 🎯 Use Cases

### 1. Reduce LLM Token Costs

```
"Compress this API response before sending to GPT"
{large JSON object}
```

Save 20-60% on token costs.

### 2. Analyze Data Efficiency

```
"Show me how much this JSON could be compressed"
{your data}
```

Get metrics before committing to ASON.

### 3. Store Compressed Data

```
"Convert this to ASON format for storage"
{data to store}
```

Reduce database storage size.

## 📚 What is ASON?

ASON (Aliased Serialization Object Notation) is a token-optimized JSON compression format designed for LLMs.

**Key Features:**
- 20-60% token reduction
- 100% lossless fidelity
- Human-readable
- Multiple compression techniques

**Compression Techniques:**
1. **Uniform Arrays**: `[2]@id,name,age`
2. **Object References**: `&obj0`
3. **Value Dictionary**: `value #0`
4. **Path Flattening**: `user.profile.name:Alice`

**Learn More:**
- [ASON Core](https://github.com/ason-format/ason)
- [Interactive Demo](https://ason-format.github.io/ason/)
- [MCP Server](https://github.com/ason-format/mcp-server)

## 🚀 Publishing

To release a new version:

```bash
./scripts/release.sh

# 1. Select version bump (patch/minor/major)
# 2. Update CHANGELOG.md when prompted
# 3. Confirm push

# Then submit to Zed Extensions:
# https://github.com/zed-industries/extensions
```

## 📝 License

MIT © ASON Project Contributors

## 🤝 Contributing

Contributions welcome! Please open an issue or pull request.

## 🔗 Links

- **Extension**: https://github.com/ason-format/zed-extension
- **MCP Server**: https://github.com/ason-format/mcp-server
- **ASON Core**: https://github.com/ason-format/ason
- **Zed Extensions**: https://github.com/zed-industries/extensions
- **Issues**: https://github.com/ason-format/zed-extension/issues
