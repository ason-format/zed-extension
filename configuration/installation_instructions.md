# ASON MCP Server Installation

## Quick Start

The ASON MCP Server extension is now installed! It provides compression and decompression tools for the ASON format in Zed's AI Assistant.

## Configuration (Optional)

You can customize compression settings in your Zed settings:

```json
{
  "context_servers": {
    "ason-mcp": {
      "source": "extension",
      "enabled": true,
      "settings": {
        "indent": 1,           // Indentation level (0-4)
        "delimiter": ",",      // Field delimiter character
        "use_references": true,  // Enable object references (&obj0)
        "use_dictionary": true   // Enable value dictionary (#0)
      }
    }
  }
}
```

### Configuration Options

#### `indent` (number, default: 1)
- `0` - No indentation (maximum compression)
- `1` - Compact format (recommended)
- `2+` - Pretty print with specified indentation

#### `delimiter` (string, default: ",")
- Separator for fields in uniform arrays
- Common values: `","`, `"|"`, `";"`, `"\t"`

#### `use_references` (boolean, default: true)
- Enable object references to avoid duplication
- Objects referenced as `&obj0`, `&obj1`, etc.
- Reduces token count for repeated objects

#### `use_dictionary` (boolean, default: true)
- Enable value dictionary for repeated values
- Values referenced as `#0`, `#1`, etc.
- Reduces token count for repeated strings/numbers

## Available Tools

Once configured, these tools are available in the AI Assistant:

### compress_json
Compress JSON data to ASON format (20-60% token reduction)

### decompress_ason
Decompress ASON format back to JSON (lossless)

### get_compression_stats
Analyze compression metrics without performing compression

### configure_compressor
Update compression settings on the fly

## Usage Examples

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

Result:
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

Result:
```json
{
  "users": [
    {"id": 1, "name": "Alice", "age": 25},
    {"id": 2, "name": "Bob", "age": 30}
  ]
}
```

### Get Statistics
```
Ask the assistant: "Show me compression stats for this JSON"
{"data": [...]}
```

Returns:
- Original vs compressed tokens
- Byte size comparison
- Reduction percentage

## Troubleshooting

### Extension not loading
- Verify Node.js v18+ is installed: `node --version`
- Check Zed logs: `~/Library/Logs/Zed/Zed.log`
- Reinstall extension via Extensions panel

### Tools not available
- Ensure `"enabled": true` in settings
- Restart Zed after configuration changes
- Check context server is running in Zed logs

### Compression not working
- Verify JSON is valid
- Check configuration settings are valid
- Try default settings first

## Learn More

- **Documentation**: https://github.com/ason-format/zed-extension#readme
- **ASON Format**: https://github.com/ason-format/ason
- **Interactive Demo**: https://ason-format.github.io/ason/
- **MCP Server**: https://github.com/ason-format/mcp-server

## Support

For issues or questions:
- Extension issues: https://github.com/ason-format/zed-extension/issues
- ASON format: https://github.com/ason-format/ason/issues
- Zed Editor: https://github.com/zed-industries/zed/issues
