# Changelog - Zed Extension

All notable changes to the ASON Zed Extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-01-12

### Added
- **Initial Release** - MCP Server extension for Zed Editor
- **Rust-Based Extension** - Native Zed extension using official Zed Extension API
- **MCP Integration** - Full Model Context Protocol support via context server
- **AI Assistant Tools**:
  - `compress_json` - Compress JSON to ASON format (20-60% token reduction)
  - `decompress_ason` - Decompress ASON back to JSON (lossless)
  - `get_compression_stats` - Analyze compression metrics without performing compression
  - `configure_compressor` - Customize compression settings (indent, delimiter, references, dictionary)
- **Auto-Update** - Automatically installs and uses latest `@ason-format/mcp-server` from npm
- **Zero Configuration** - Works out of the box with Zed AI Assistant
- **Context Server Implementation** - Implements `context_server_command` for seamless integration

### Technical
- Built with Rust using `zed_extension_api@0.7.0`
- Compiles to WebAssembly for native Zed integration
- Uses `npm_install_package` API to manage MCP server dependency
- Executes Node.js directly with installed package path
- Minimal overhead - thin wrapper around npm package

### Architecture
```
Zed AI Assistant → Rust Extension → Node.js → @ason-format/mcp-server → @ason-format/ason
```

### Dependencies
- `zed_extension_api@0.7.0` - Official Zed extension API
- `serde@1.0` - Serialization framework
- `schemars@0.8` - JSON Schema generation
- `@ason-format/mcp-server@latest` (runtime, auto-installed)

### Requirements
- Zed Editor (latest version)
- Node.js v18+ (for npx)
- Rust (for development only)

### Compatibility
- ✅ Zed AI Assistant
- ✅ MCP Protocol (stdio transport)
- ✅ macOS, Linux, Windows

### Installation
- Via Zed Extensions (pending submission)
- Local dev installation: `zed: install dev extension`

[Unreleased]: https://github.com/ason-format/zed-extension/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/ason-format/zed-extension/releases/tag/v1.0.0
