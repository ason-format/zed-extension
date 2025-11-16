# ASON MCP Server for Zed

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Node.js](https://img.shields.io/badge/Node.js-v18+-green.svg)](https://nodejs.org/)

Model Context Protocol (MCP) extension for Zed that provides ASON compression/decompression tools in the AI assistant. Reduce token usage by 20-60% for LLM interactions while maintaining 100% lossless round-trip fidelity.

## ✨ Features

- **🤖 AI Assistant Integration**: Use ASON tools directly in Zed's AI Assistant
- **📦 Zero Configuration**: Works out of the box, auto-installs dependencies
- **🔄 Auto-Update**: Automatically installs and uses latest MCP server from npm
- **⚡ Fast**: Native Rust extension compiled to WebAssembly

### Available Tools

- `compress_json` - Compress JSON to ASON format (20-60% token reduction)
- `decompress_ason` - Decompress ASON back to JSON (lossless)
- `get_compression_stats` - Analyze compression metrics
- `configure_compressor` - Customize compression settings

## 📦 Installation

### Prerequisites

- [Zed Editor](https://zed.dev) (latest version)
- [Node.js](https://nodejs.org) v18+ (runtime requirement)
- [Rust](https://rustup.rs) (only for local development)

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
users:[2]{id,name,age}
1|Alice|25
2|Bob|30
```

### Decompress ASON

```
Ask the assistant: "Decompress this ASON to JSON"
users:[2]{id,name,age}
1|Alice|25
2|Bob|30
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

### Enable the Context Server

After installing the extension, enable it in your Zed `settings.json`:

```json
{
  "context_servers": {
    "ason-mcp/ason": {
      "source": "extension",
      "enabled": true,
      "settings": {}
    }
  }
}
```

The extension will automatically:
1. Install `@ason-format/mcp-server@latest` from npm
2. Start the MCP server using Node.js
3. Make ASON tools available in the AI Assistant

No additional configuration needed!

## 📊 How It Works

This extension provides a thin Rust wrapper around the [@ason-format/mcp-server](https://www.npmjs.com/package/@ason-format/mcp-server) npm package:

1. **Extension loads** → Rust code initializes in Zed
2. **MCP server installs** → Uses `npm_install_package` API to get latest version
3. **Server starts** → Executes Node.js with installed package path
4. **Tools available** → AI assistant can use ASON compression tools
5. **Auto-updates** → Checks and installs latest version on each launch

## 🏗️ Architecture

```
Zed AI Assistant
    ↓ loads
Rust Extension (WebAssembly)
    ↓ installs
@ason-format/mcp-server (npm)
    ↓ executes
Node.js + MCP Server
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
├── Cargo.toml          # Rust dependencies (zed_extension_api, serde, schemars)
├── extension.toml      # Zed extension manifest
├── src/
│   └── lib.rs         # Extension implementation (context_server_command)
├── scripts/
│   └── release.sh     # Release automation
├── CHANGELOG.md        # Version history
└── README.md           # This file
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
1. **Sections**: `@section` - Organize related objects
2. **Tabular Arrays**: `key:[N]{fields}` - CSV-like format for uniform arrays
3. **Semantic References**: `$var` - Deduplicate repeated values
4. **Pipe Delimiter**: `|` - More token-efficient than commas
5. **Dot Notation**: `user.profile.name:Alice` - Flatten nested objects

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
