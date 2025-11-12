# Publishing Guide - ASON Zed Extension

This guide explains how to release new versions and publish the extension to Zed's extension marketplace.

## Release Process

### 1. Create a New Release

Run the release script:

```bash
./scripts/release.sh
```

The script will:
1. Show current version
2. Ask for version bump type (patch/minor/major)
3. Update `extension.toml` and `Cargo.toml` versions
4. Prompt you to update `CHANGELOG.md`
5. Create a git commit and tag
6. Push to GitHub

### 2. Update CHANGELOG

When prompted, update `CHANGELOG.md`:

```markdown
## [Unreleased]

## [X.Y.Z] - 2025-01-XX

### Added
- New feature description

### Fixed
- Bug fix description

### Changed
- Change description
```

### 3. Create GitHub Release

After pushing, create a GitHub Release:

1. Go to: https://github.com/ason-format/zed-extension/releases/new
2. Select the tag you just created (e.g., `v1.0.0`)
3. Title: `v1.0.0`
4. Copy release notes from CHANGELOG.md
5. Publish release

## Publishing to Zed Extensions

To make your extension available in Zed's extension marketplace, you need to submit it to the official extensions repository.

### Prerequisites

- Extension must be in a public GitHub repository
- Must have `extension.toml` with correct metadata
- Must have Rust source code (`src/lib.rs`) or language configs
- Repository must be stable (not constantly rebasing)

### Submission Steps

#### 1. Fork the Extensions Repository

```bash
git clone https://github.com/zed-industries/extensions
cd extensions
```

Fork the repository on GitHub first, then clone your fork.

#### 2. Add Your Extension as a Submodule

```bash
# Add your extension as a git submodule
git submodule add https://github.com/ason-format/zed-extension extensions/ason-mcp

# Commit the submodule
git add .gitmodules extensions/ason-mcp
git commit -m "Add ASON MCP Server extension"
```

**Important naming:**
- Use the extension ID from `extension.toml` (e.g., `ason-mcp`)
- Place in `extensions/` directory

#### 3. Verify Extension Structure

Ensure your extension repository has:

```
zed-extension/
├── extension.toml       # Required - extension manifest
├── Cargo.toml           # Required for Rust extensions
├── src/
│   └── lib.rs          # Required - extension implementation
├── README.md            # Recommended
└── CHANGELOG.md         # Recommended
```

#### 4. Test Locally

Before submitting, test the extension works:

```bash
# In Zed:
# 1. Cmd+Shift+P → "zed: install dev extension"
# 2. Select your extension directory
# 3. Verify it compiles and runs correctly
```

#### 5. Create Pull Request

```bash
# Push to your fork
git push origin main

# Create PR on GitHub:
# https://github.com/zed-industries/extensions/compare
```

**PR Template:**

```markdown
# Add ASON MCP Server Extension

## Description
Model Context Protocol server for ASON compression/decompression. Reduces JSON tokens by 20-60% for LLM interactions.

## Extension Details
- **ID**: `ason-mcp`
- **Name**: ASON MCP Server
- **Repository**: https://github.com/ason-format/zed-extension
- **Version**: 1.0.0

## Features
- Compress JSON to ASON format (20-60% token reduction)
- Decompress ASON back to JSON (lossless)
- Analyze compression statistics
- Configure compression settings

## Testing
- [x] Extension compiles successfully
- [x] Context server starts correctly
- [x] Tools available in AI Assistant
- [x] README and documentation included

## Checklist
- [x] Added as git submodule in `extensions/ason-mcp`
- [x] Has valid `extension.toml`
- [x] Has Rust implementation in `src/lib.rs`
- [x] Tested locally in Zed
- [x] Documentation is complete
```

#### 6. Wait for Review

The Zed team will:
1. Review your extension code
2. Test compilation and functionality
3. Check for security issues
4. Approve or request changes

#### 7. After Approval

Once merged:
- Your extension appears in Zed's extension marketplace
- Users can install via: Extensions → Search "ASON" → Install
- Updates are pulled automatically when you push new versions

## Updating Published Extension

### For Bug Fixes (Patch Version)

```bash
# Make your changes
git commit -am "Fix: description"

# Release new version
./scripts/release.sh
# Select: 1) patch

# The extensions repo will auto-update the submodule
# No PR needed for version bumps
```

### For New Features (Minor/Major Version)

Same process as patch, but select appropriate version bump.

## Extension Metadata

Ensure `extension.toml` is complete:

```toml
id = "ason-mcp"                                    # Unique ID (lowercase, hyphens)
name = "ASON MCP Server"                           # Display name
description = "Model Context Protocol server..."   # Brief description
version = "1.0.0"                                  # Semantic version
schema_version = 1                                 # Zed schema version
authors = ["ASON Format <https://github.com/ason-format>"]
repository = "https://github.com/ason-format/zed-extension"

[context_servers.ason]
name = "ASON MCP Server"
```

## Common Issues

### Extension Won't Compile

- Ensure Rust version is compatible
- Check `Cargo.toml` dependencies match API version
- Verify `zed_extension_api` version is correct

### Submodule Not Added Correctly

```bash
# Remove and re-add
cd extensions
git rm -f extensions/ason-mcp
git submodule add https://github.com/ason-format/zed-extension extensions/ason-mcp
```

### Extension Not Showing in Marketplace

- Verify PR was merged to `main` branch
- Check extension ID is unique
- Ensure `extension.toml` is valid
- Wait a few minutes for cache refresh

## Resources

- **Zed Extensions Docs**: https://zed.dev/docs/extensions/developing-extensions
- **Extensions Repository**: https://github.com/zed-industries/extensions
- **Extension API**: https://docs.rs/zed_extension_api
- **MCP Documentation**: https://zed.dev/docs/ai/mcp

## Support

For issues with:
- **Extension functionality**: Open issue in this repo
- **Publishing process**: Ask in Zed Discord #extensions channel
- **Zed bugs**: Open issue in zed-industries/zed

---

**Next Steps:**

1. ✅ Run `./scripts/release.sh` to create v1.0.0
2. ✅ Create GitHub Release
3. ⏳ Submit to zed-industries/extensions
4. ⏳ Wait for approval
5. ✅ Extension published!
