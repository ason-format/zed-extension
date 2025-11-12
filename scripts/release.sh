#!/bin/bash

# Zed Extension Release Script
# Usage: ./scripts/release.sh

set -e

# Change to project root directory
cd "$(dirname "$0")/.."

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Zed Extension Release Script         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

# Get current version from extension.toml
CURRENT_VERSION=$(grep '^version = ' extension.toml | sed 's/version = "\(.*\)"/\1/')
echo -e "Current version: ${YELLOW}v${CURRENT_VERSION}${NC}"

# Function to bump version
bump_version() {
  local version=$1
  local type=$2
  local major minor patch

  IFS='.' read -r major minor patch <<< "$version"

  case $type in
    patch) patch=$((patch + 1)) ;;
    minor) minor=$((minor + 1)); patch=0 ;;
    major) major=$((major + 1)); minor=0; patch=0 ;;
  esac

  echo "${major}.${minor}.${patch}"
}

# Select version bump type
echo ""
echo -e "${GREEN}Select version bump:${NC}"
echo "  1) patch - Bug fixes (${CURRENT_VERSION} → $(bump_version $CURRENT_VERSION patch))"
echo "  2) minor - New features (${CURRENT_VERSION} → $(bump_version $CURRENT_VERSION minor))"
echo "  3) major - Breaking changes (${CURRENT_VERSION} → $(bump_version $CURRENT_VERSION major))"
echo ""
read -p "Enter choice (1-3): " BUMP_CHOICE

case $BUMP_CHOICE in
  1) BUMP_TYPE="patch" ;;
  2) BUMP_TYPE="minor" ;;
  3) BUMP_TYPE="major" ;;
  *)
    echo -e "${RED}Invalid choice${NC}"
    exit 1
    ;;
esac

# Bump version
echo ""
echo -e "${BLUE}Bumping version...${NC}"
NEW_VERSION=$(bump_version $CURRENT_VERSION $BUMP_TYPE)

# Update extension.toml
sed -i.bak "s/^version = .*/version = \"${NEW_VERSION}\"/" extension.toml
rm -f extension.toml.bak
echo -e "${GREEN}✓ Updated extension.toml${NC}"

# Update Cargo.toml
sed -i.bak "s/^version = .*/version = \"${NEW_VERSION}\"/" Cargo.toml
rm -f Cargo.toml.bak
echo -e "${GREEN}✓ Updated Cargo.toml${NC}"

echo -e "New version: ${GREEN}v${NEW_VERSION}${NC}"
echo ""

# Update CHANGELOG
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}IMPORTANT: Update CHANGELOG.md${NC}"
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Change from:"
echo -e "  ${RED}## [Unreleased]${NC}"
echo ""
echo "To:"
echo -e "  ${GREEN}## [${NEW_VERSION}] - $(date +%Y-%m-%d)${NC}"
echo ""
read -p "Press ENTER when you've updated CHANGELOG.md..."

# Commit and tag
git add Cargo.toml Cargo.lock extension.toml CHANGELOG.md 2>/dev/null || true
git commit -m "Release v${NEW_VERSION}"

TAG_NAME="v${NEW_VERSION}"
git tag -a "$TAG_NAME" -m "Release v${NEW_VERSION}"

echo ""
echo -e "${GREEN}✓ Tag created: ${TAG_NAME}${NC}"
echo ""

# Push
read -p "Push changes and tag to remote? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  git push origin main
  git push origin "$TAG_NAME"

  echo ""
  echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}✓ Release completed!${NC}"
  echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo ""
  echo -e "Version: ${GREEN}v${NEW_VERSION}${NC}"
  echo -e "Tag: ${YELLOW}${TAG_NAME}${NC}"
  echo ""
  echo -e "${YELLOW}Next steps:${NC}"
  echo "  1. Create GitHub Release manually:"
  echo "     https://github.com/ason-format/zed-extension/releases/new"
  echo ""
  echo "  2. Submit to Zed Extensions:"
  echo "     • Fork: https://github.com/zed-industries/extensions"
  echo "     • Add submodule: git submodule add https://github.com/ason-format/zed-extension extensions/ason-mcp"
  echo "     • Create PR to zed-industries/extensions"
  echo ""
  echo "  3. Documentation:"
  echo "     • Include extension.toml and Cargo.toml"
  echo "     • Zed will auto-compile on install"
  echo ""
fi
