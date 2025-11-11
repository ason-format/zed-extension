# Release Guide

This document explains how to create and publish a new release of the ASON Zed Extension.

## Prerequisites

1. **GitHub Account**: For repository management and releases
2. **Node.js**: For building the extension
3. **Zed IDE**: For testing the extension

Note: Zed extensions are typically distributed through Zed's extension system or GitHub releases.

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0 → 2.0.0): Breaking changes
- **MINOR** (1.0.0 → 1.1.0): New features, backwards compatible
- **PATCH** (1.0.0 → 1.0.1): Bug fixes, backwards compatible

## Release Process

### Using the Release Script (Recommended)

```bash
# Run the automated release script
./scripts/release.sh

# Follow the interactive prompts:
# 1. Choose version bump type (patch/minor/major)
# 2. Review the new version
# 3. Script will update files, create tag, and push
```

The script automatically updates both `package.json` and `extension.toml`.

### Manual Release Process

### 1. Update Version

Edit both `package.json` and `extension.toml`:

**package.json:**
```json
{
  "version": "1.0.0"  // Update this
}
```

**extension.toml:**
```toml
version = "1.0.0"  # Update this
```

### 2. Update CHANGELOG.md

Add a new section at the top:

```markdown
## [1.0.0] - 2025-01-15

### Added
- New feature X
- New feature Y

### Fixed
- Bug fix Z

### Changed
- Improvement W
```

### 3. Build Extension

```bash
# Install dependencies
npm install

# Build extension
npm run build
```

### 4. Commit Changes

```bash
git add package.json extension.toml CHANGELOG.md
git commit -m "Release v1.0.0"
git push origin main
```

### 5. Create Git Tag

```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release v1.0.0

- New feature X
- New feature Y
- Bug fix Z"

# Push tag to GitHub
git push origin v1.0.0
```

### 6. Create GitHub Release

**Option A: Using GitHub UI**

1. Go to: https://github.com/ason-format/zed-extension/releases/new
2. Choose tag: `v1.0.0`
3. Release title: `v1.0.0`
4. Description: Copy from CHANGELOG.md
5. Click "Publish release"

**Option B: Using GitHub CLI**

```bash
gh release create v1.0.0 \
  --title "v1.0.0" \
  --notes-file - <<EOF
## What's New

- New feature X
- New feature Y
- Bug fix Z

## Installation

Install from Zed's extension manager or clone this repository into your Zed extensions directory.

## Full Changelog

See [CHANGELOG.md](https://github.com/ason-format/zed-extension/blob/main/CHANGELOG.md)
EOF
```

### 7. Verify Installation

Test the extension in Zed:

```bash
# Link extension for testing (macOS/Linux)
ln -s $(pwd) ~/.config/zed/extensions/ason

# Restart Zed and test the extension
```

## Quick Release Checklist

- [ ] Update version in `package.json` and `extension.toml`
- [ ] Update `CHANGELOG.md` with changes
- [ ] Build and test: `npm run build`
- [ ] Commit and push changes
- [ ] Create and push git tag `v1.x.x`
- [ ] Create GitHub release
- [ ] Test installation in Zed
- [ ] Update documentation if needed

## Troubleshooting

### Build Fails

1. Check build output: `npm run build`
2. Fix any compilation errors
3. Ensure all dependencies are installed

### Extension Not Loading in Zed

1. Check Zed's extension log
2. Verify `extension.toml` is valid
3. Ensure extension is in correct directory
4. Restart Zed

### Version Conflicts

If you need to re-release:

```bash
# Increment patch version
# Update both package.json and extension.toml to "1.0.1"

# Delete local tag
git tag -d v1.0.0

# Delete remote tag
git push --delete origin v1.0.0

# Create new tag
git tag -a v1.0.1 -m "Release v1.0.1"
git push origin v1.0.1
```

## Post-Release

1. Announce on GitHub Discussions
2. Update documentation if needed
3. Close related issues/PRs
4. Monitor for bug reports

## Beta/Pre-releases

For beta versions:

```bash
# Update versions to pre-release
# package.json: "1.1.0-beta.1"
# extension.toml: version = "1.1.0-beta.1"

# Build and tag
npm run build
git tag -a v1.0.0-beta.1 -m "Beta release"
git push origin v1.0.0-beta.1

# On GitHub, mark as "pre-release"
```

## Distribution via Zed Extension Registry

If Zed adds an official extension registry in the future:

1. Follow Zed's submission guidelines
2. Ensure `extension.toml` meets requirements
3. Submit extension for review
4. Monitor approval status

For now, users can install by:
- Cloning the repository
- Adding to Zed's extensions directory
- Following installation instructions in README

## References

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)
