# Contributing to ASON Zed Extension

Thank you for your interest in contributing to the ASON Zed Extension! This document provides guidelines and instructions for contributing to this project.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps to reproduce the problem**
* **Provide specific examples** including the JSON you tried to compress
* **Describe the behavior you observed** and explain what behavior you expected
* **Include screenshots** of the Zed interface if relevant
* **Specify your environment**: Zed version, extension version, OS, etc.

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

* **Use a clear and descriptive title**
* **Provide a detailed description** of the suggested enhancement
* **Explain why this enhancement would be useful** to Zed users
* **List examples** of how the feature would be used
* **Mention if you're willing to implement** the enhancement yourself

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following our coding standards (see below)
3. **Add tests** if you're adding functionality
4. **Ensure the extension builds**: `npm run build`
5. **Update documentation** if you're changing functionality
6. **Write a clear commit message** describing your changes
7. **Submit a pull request** with a comprehensive description

## Development Setup

```bash
# Clone the repository
git clone https://github.com/ason-format/zed-extension.git
cd zed-extension

# Install dependencies
npm install

# Build the extension
npm run build

# Link for local development (in Zed extensions directory)
ln -s $(pwd) ~/.config/zed/extensions/ason
```

### Testing in Zed

1. Build the extension: `npm run build`
2. Restart Zed or reload extensions
3. Test compression/decompression commands
4. Check Zed's log for errors

## Coding Standards

### TypeScript/JavaScript Style Guide

* Use **TypeScript** for type safety
* Follow **consistent indentation** (2 spaces)
* Use **meaningful variable names**
* Add **JSDoc comments** for public APIs
* Keep functions **small and focused**
* Prefer **const** over **let**, avoid **var**

### Example:

```typescript
/**
 * Compresses selected text to ASON format
 */
export function compress(text: string): string {
  // Implementation
}
```

### Testing

* **Write tests** for new features and bug fixes
* **Test with various JSON structures**
* **Use descriptive test names** that explain what is being tested

### Commit Messages

* Use the **present tense** ("Add feature" not "Added feature")
* Use the **imperative mood** ("Move cursor to..." not "Moves cursor to...")
* **Limit the first line** to 72 characters or less
* **Reference issues and pull requests** when relevant

Examples:
```
Add language server support
Fix compression of nested objects
Update extension manifest
```

## Project Structure

```
zed-ason/
├── src/
│   └── index.ts              # Main extension entry point
├── extension.toml            # Extension manifest
├── tests/                    # Test suite
└── scripts/                  # Build and release scripts
```

## Zed Extension API

When contributing:

* Follow [Zed Extension Guidelines](https://zed.dev/docs/extensions)
* Ensure commands are properly registered in `extension.toml`
* Handle errors gracefully with user-friendly messages
* Test with different editor states

## Documentation

* Update the **README.md** if you change functionality
* Update `extension.toml` for new commands or settings
* Add examples for new features
* Update **CHANGELOG.md** with your changes

## Testing Guidelines

### Manual Testing

1. Test compression/decompression commands
2. Try with different JSON structures
3. Test with large files
4. Verify error messages are helpful
5. Check performance

## Release Process

(For maintainers)

1. Update version in `package.json` and `extension.toml`
2. Update CHANGELOG.md
3. Build and test: `npm run build`
4. Run release script: `./scripts/release.sh`
5. Follow prompts to create tag and push

## Questions?

* Check the [documentation](./README.md)
* Review [existing issues](https://github.com/ason-format/zed-extension/issues)
* Open a new issue for discussion

## Recognition

Contributors will be:
* Listed in release notes
* Mentioned in significant feature announcements
* Credited in the project documentation

Thank you for contributing to ASON Zed Extension!
