# Changelog - Zed Extension

All notable changes to the ASON Zed Extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-01-11

### Added
- **Initial Release** - MCP Server extension for Zed Editor
- **Rust-Based Extension** - Native Zed extension using official API
- **MCP Integration** - Full Model Context Protocol support
- **AI Assistant Tools**:
  - `compress_json` - Compress JSON to ASON format (20-60% reduction)
  - `decompress_ason` - Decompress ASON back to JSON (lossless)
  - `get_compression_stats` - Analyze compression metrics
  - `configure_compressor` - Customize compression settings
- **Auto-Update** - Always uses latest `@ason-format/mcp-server` via npx
- **Zero Configuration** - Works out of the box with default settings
- **Context Server Command** - Implements `context_server_command` trait method

### Technical
- Built with Rust using `zed_extension_api`
- Compiles to WebAssembly for Zed integration
- Uses `npx` for automatic MCP server resolution
- Minimal overhead - thin wrapper around npm package

### Architecture
```
Zed → Rust Extension → npx → @ason-format/mcp-server → @ason-format/ason
```

### Dependencies
- `zed_extension_api@0.2.0` - Official Zed extension API
- `@ason-format/mcp-server@latest` (runtime via npx)

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
