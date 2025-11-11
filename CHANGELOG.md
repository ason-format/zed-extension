# Changelog - Zed Extension

All notable changes to the ASON Zed Extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-01-11

### Added
- **Initial Release** - Zed IDE integration for ASON compression
- **JavaScript API** - Complete API for Zed scripting:
  - `compressJson(jsonText)` - Compress JSON with detailed statistics
  - `decompressAson(asonText)` - Decompress ASON to formatted JSON
  - `getStats(jsonText)` - Analyze compression metrics without compressing
  - `configure(config)` - Update global compressor settings
- **Configuration Support**:
  - `indent` - Indentation level (default: 1)
  - `delimiter` - Field delimiter (default: ",")
  - `useReferences` - Enable object references (default: true)
  - `useDictionary` - Enable value dictionary (default: true)
- **Detailed Statistics** - All functions return comprehensive stats:
  - Original token count
  - Compressed token count
  - Reduction percentage (formatted to 2 decimals)
  - Original size in bytes
  - Compressed size in bytes
  - Byte savings
- **ASON Preview** - `getStats()` includes ASON preview without modifying data
- **Error Handling** - Graceful error handling with descriptive messages
- **Extension Manifest** - `extension.toml` configured for Zed

### Features
- Zero-dependency core logic (only requires `@ason-format/ason`)
- CommonJS module compatible with Zed's Node.js runtime
- Returns structured objects for easy integration
- Global configuration persists across function calls
- JSON output formatted with 2-space indentation

### Dependencies
- `@ason-format/ason@^1.1.2` - Core ASON library

### Installation
Manual installation to `~/.config/zed/extensions/ason/`

### Compatibility
- Zed IDE with Node.js extension support
- Future: Zed Extensions marketplace (pending submission)

### Note
Zed's extension API is evolving. This extension provides core functionality as a Node.js module. Full IDE integration (commands, keybindings, UI) will be available when Zed's extension capabilities mature.

[Unreleased]: https://github.com/ason-format/zed-extension/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/ason-format/zed-extension/releases/tag/v1.0.0
