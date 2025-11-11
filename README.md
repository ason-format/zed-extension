# ASON for Zed

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Node.js](https://img.shields.io/badge/Node.js-v18+-green.svg)](https://nodejs.org/)

Compress and decompress JSON using the ASON format directly in Zed IDE. Reduce token usage by 20-60% for LLM applications while maintaining 100% lossless round-trip fidelity.

## Features

- **Compress JSON to ASON**: Convert JSON to token-optimized ASON format
- **Decompress ASON to JSON**: Restore ASON back to original JSON (lossless)
- **Compression Statistics**: View detailed metrics on token and byte savings
- **Configurable Settings**: Customize compression behavior

## Installation

### Option 1: Manual Installation

1. Clone or copy this directory to your Zed extensions folder:

```bash
# macOS/Linux
mkdir -p ~/.config/zed/extensions
cp -r zed-ason ~/.config/zed/extensions/ason

# Install dependencies
cd ~/.config/zed/extensions/ason
npm install
```

2. Restart Zed IDE

### Option 2: Via Zed Extensions (Future)

Once published to Zed's extension registry:

1. Open Zed
2. Go to Extensions (`Cmd+Shift+X`)
3. Search for "ASON"
4. Click Install

## Usage

### Using as a Module

The extension exposes JavaScript functions that can be used in Zed's scripting environment:

```javascript
const ason = require('ason');

// Compress JSON
const result = ason.compressJson('{"users": [{"id": 1, "name": "Alice"}]}');
console.log(result.ason);
console.log(result.stats);

// Decompress ASON
const json = ason.decompressAson('users:[1]@id,name\n1,Alice');
console.log(json.json);

// Get statistics
const stats = ason.getStats('{"users": [{"id": 1, "name": "Alice"}]}');
console.log(stats.stats);
console.log(stats.asonPreview);

// Configure
ason.configure({
  indent: 2,
  delimiter: '|',
  useReferences: false
});
```

### Response Format

#### compressJson(jsonText)

```javascript
{
  success: true,
  ason: "users:[1]@id,name\n1,Alice",
  stats: {
    originalTokens: 25,
    compressedTokens: 12,
    reductionPercent: "52.00",
    originalSize: 45,
    compressedSize: 28,
    savings: 17
  }
}
```

#### decompressAson(asonText)

```javascript
{
  success: true,
  json: "{\n  \"users\": [\n    {\"id\": 1, \"name\": \"Alice\"}\n  ]\n}"
}
```

#### getStats(jsonText)

```javascript
{
  success: true,
  stats: {
    originalTokens: 25,
    compressedTokens: 12,
    reductionPercent: "52.00",
    originalSize: 45,
    compressedSize: 28,
    savings: 17
  },
  asonPreview: "users:[1]@id,name\n1,Alice",
  config: {
    indent: 1,
    delimiter: ",",
    useReferences: true,
    useDictionary: true
  }
}
```

## Configuration

Configure ASON behavior by calling the `configure()` function:

```javascript
const ason = require('ason');

ason.configure({
  indent: 1,              // Indentation level for nested structures
  delimiter: ',',         // Field delimiter for uniform arrays
  useReferences: true,    // Enable object reference aliasing (&obj0)
  useDictionary: true     // Enable inline-first value dictionary (value #0)
});
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `indent` | number | `1` | Indentation level for nested structures |
| `delimiter` | string | `","` | Field delimiter for uniform arrays |
| `useReferences` | boolean | `true` | Enable object reference aliasing (`&obj0`) |
| `useDictionary` | boolean | `true` | Enable inline-first value dictionary (`value #0`) |

## Example Usage

### Basic Compression

```javascript
const ason = require('ason');

const json = {
  "users": [
    {"id": 1, "name": "Alice", "age": 25},
    {"id": 2, "name": "Bob", "age": 30}
  ]
};

const result = ason.compressJson(JSON.stringify(json));

if (result.success) {
  console.log('ASON Output:');
  console.log(result.ason);
  // Output:
  // users:[2]@id,name,age
  // 1,Alice,25
  // 2,Bob,30

  console.log(`Token Reduction: ${result.stats.reductionPercent}%`);
  console.log(`Saved: ${result.stats.savings} bytes`);
}
```

### Custom Configuration

```javascript
const ason = require('ason');

// Configure for pipe-delimited output
ason.configure({
  delimiter: '|',
  indent: 2
});

const result = ason.compressJson('{"items": [{"a": 1, "b": 2}]}');
console.log(result.ason);
// Output: items:[1]@a|b
//         1|2
```

### Statistics Only

```javascript
const ason = require('ason');

const stats = ason.getStats('{"large": "dataset", "with": "many", "keys": true}');

console.log(`Original: ${stats.stats.originalTokens} tokens, ${stats.stats.originalSize} bytes`);
console.log(`Compressed: ${stats.stats.compressedTokens} tokens, ${stats.stats.compressedSize} bytes`);
console.log(`Reduction: ${stats.stats.reductionPercent}%`);
console.log(`\nPreview:\n${stats.asonPreview}`);
```

## Use Cases

### Reduce LLM API Costs

```javascript
const ason = require('ason');

// Before sending to LLM API
const largeData = { /* ... */ };
const result = ason.compressJson(JSON.stringify(largeData));

if (result.success) {
  // Send result.ason instead of JSON
  // Save 20-60% on token costs
  console.log(`Savings: ${result.stats.reductionPercent}%`);
}
```

### Optimize Data Storage

```javascript
const ason = require('ason');

// Compress before storing
const compressed = ason.compressJson(JSON.stringify(data));
storage.save('key', compressed.ason);

// Decompress when loading
const stored = storage.load('key');
const decompressed = ason.decompressAson(stored);
const data = JSON.parse(decompressed.json);
```

## Development

### Building from Source

```bash
cd zed-ason
npm install
```

### Testing

Create a test script:

```javascript
// test.js
const ason = require('./src/ason.js');

const testData = {
  users: [
    { id: 1, name: 'Alice', age: 25 },
    { id: 2, name: 'Bob', age: 30 }
  ]
};

console.log('Testing compression...');
const result = ason.compressJson(JSON.stringify(testData));
console.log('Result:', result);

console.log('\nTesting decompression...');
const restored = ason.decompressAson(result.ason);
console.log('Restored:', restored);

console.log('\nTesting stats...');
const stats = ason.getStats(JSON.stringify(testData));
console.log('Stats:', stats);
```

Run:

```bash
node test.js
```

## What is ASON?

ASON (Aliased Serialization Object Notation) is a token-optimized JSON compression format designed specifically for Large Language Models (LLMs).

**Key Features:**
- 20-60% token reduction on average
- 100% lossless round-trip fidelity
- Human-readable format
- Multiple compression techniques

**Compression Techniques:**
1. **Uniform Arrays**: Extract common keys to header (`[2]@id,name,age`)
2. **Object References**: Deduplicate structures (`&obj0`)
3. **Inline-First Value Dictionary**: Reduce repeated values (`value #0`)
4. **Path Flattening**: Collapse nested properties (`user.profile.name:Alice`)

**Learn More:**
- [ASON GitHub](https://github.com/ason-format/ason)
- [Interactive Demo](https://ason-format.github.io/ason/)
- [Documentation](https://ason-format.github.io/ason/docs.html)

## Zed IDE Resources

- [Zed Extensions Documentation](https://zed.dev/docs/extensions)
- [Zed Extensions Repository](https://github.com/zed-industries/extensions)
- [Zed Extension API](https://zed.dev/docs/extensions/developing-extensions)

## 🚀 Publishing

To release a new version:

```bash
# Run the release script
./scripts/release.sh

# 1. Select version bump (patch/minor/major)
# 2. Update CHANGELOG.md when prompted
# 3. Update extension.toml version (done automatically)
# 4. Confirm push

# GitHub Actions will automatically:
# - Create GitHub Release

# Then manually submit to Zed Extensions:
# https://github.com/zed-industries/extensions
```

## ⚠️ Note on Zed Extension API

Zed's extension API is still evolving. This extension provides the core ASON functionality as a Node.js module that can be integrated with Zed's scripting capabilities.

**Current capabilities:**
- ✅ Core compression/decompression functions
- ✅ Statistics and configuration
- ⏳ Native commands (pending Zed API support)
- ⏳ Keybindings (pending Zed API support)
- ⏳ UI integration (pending Zed API support)

For the latest on Zed's extension capabilities, see the [official documentation](https://zed.dev/docs/extensions).

## 📝 License

MIT © ASON Project Contributors

## 🤝 Contributing

Contributions welcome! Please open an issue or pull request.

## 🔗 Links

- **GitHub**: https://github.com/ason-format/zed-extension
- **Issues**: https://github.com/ason-format/zed-extension/issues
- **ASON Core**: https://github.com/ason-format/ason
- **Zed Extensions**: https://github.com/zed-industries/extensions
- **Zed Docs**: https://zed.dev/docs/extensions
