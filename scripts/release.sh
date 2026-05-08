#!/usr/bin/env bash
# Bump the version, commit, tag and push — which triggers the GitHub
# Actions release workflow that builds bundles for all four platforms.
#
# Usage:
#   scripts/release.sh              patch bump (v0.1.1 -> v0.1.2)
#   scripts/release.sh patch        explicit patch bump
#   scripts/release.sh minor        v0.1.1 -> v0.2.0
#   scripts/release.sh major        v0.1.1 -> v1.0.0
#   scripts/release.sh --no-push    bump + commit + tag locally, don't push
#   scripts/release.sh --version 1.2.3
#                                   set an exact version instead of bumping

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BLUE='\033[0;34m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RED='\033[0;31m'; NC='\033[0m'
step() { printf "\n${BLUE}==>${NC} %s\n" "$1"; }
ok()   { printf "${GREEN}✓${NC}  %s\n" "$1"; }
warn() { printf "${YELLOW}!${NC}  %s\n" "$1"; }
err()  { printf "${RED}✗${NC}  %s\n" "$1" >&2; }

BUMP="patch"
PUSH=true
EXPLICIT_VERSION=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    major|minor|patch) BUMP="$1" ;;
    --no-push)         PUSH=false ;;
    --version)
      shift
      EXPLICIT_VERSION="${1:-}"
      [[ -z "$EXPLICIT_VERSION" ]] && { err "--version needs a value"; exit 1; }
      ;;
    -h|--help)
      sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) err "Unknown argument: $1"; exit 1 ;;
  esac
  shift
done

step "Pre-flight checks"

# Working tree must be clean to avoid mixing version bumps with other work.
if ! git diff-index --quiet HEAD --; then
  err "Working tree has uncommitted changes. Commit or stash first."
  exit 1
fi
ok "Working tree clean"

CURRENT_TAG=$(git tag -l 'v*' --sort=-v:refname | head -1 || true)
if [[ -z "$CURRENT_TAG" ]]; then
  CURRENT="0.0.0"
else
  CURRENT="${CURRENT_TAG#v}"
fi

if [[ -n "$EXPLICIT_VERSION" ]]; then
  if [[ ! "$EXPLICIT_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    err "--version must look like MAJOR.MINOR.PATCH"
    exit 1
  fi
  NEW="$EXPLICIT_VERSION"
else
  IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"
  case "$BUMP" in
    major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
    minor) MINOR=$((MINOR + 1));        PATCH=0 ;;
    patch) PATCH=$((PATCH + 1)) ;;
  esac
  NEW="$MAJOR.$MINOR.$PATCH"
fi

NEW_TAG="v$NEW"

if git rev-parse "$NEW_TAG" >/dev/null 2>&1; then
  err "Tag $NEW_TAG already exists locally."
  exit 1
fi
if git ls-remote --exit-code --tags origin "$NEW_TAG" >/dev/null 2>&1; then
  err "Tag $NEW_TAG already exists on origin."
  exit 1
fi

ok "Current: ${CURRENT_TAG:-none} ($CURRENT)"
ok "New:     $NEW_TAG"

step "Updating manifests"

# Replace the first "version": "..." in each manifest. Both files only have
# one such line, so this is unambiguous.
sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW\"/" src-tauri/tauri.conf.json
rm -f src-tauri/tauri.conf.json.bak
ok "src-tauri/tauri.conf.json"

sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW\"/" package.json
rm -f package.json.bak
ok "package.json"

step "Committing"

git add src-tauri/tauri.conf.json package.json
git commit -m "chore: release $NEW_TAG"
git tag "$NEW_TAG"
ok "Tagged $NEW_TAG"

if ! $PUSH; then
  step "Done (local only)"
  echo "Push when ready:"
  echo "  git push origin HEAD && git push origin $NEW_TAG"
  exit 0
fi

step "Pushing"

BRANCH=$(git rev-parse --abbrev-ref HEAD)
git push origin "$BRANCH"
git push origin "$NEW_TAG"

step "Released $NEW_TAG"
REMOTE_URL=$(git remote get-url origin 2>/dev/null || true)
SLUG=""
case "$REMOTE_URL" in
  *github.com:*)
    SLUG="${REMOTE_URL##*github.com:}"
    SLUG="${SLUG%.git}"
    ;;
  *github.com/*)
    SLUG="${REMOTE_URL##*github.com/}"
    SLUG="${SLUG%.git}"
    ;;
esac

if [[ -n "$SLUG" ]]; then
  echo "Watch the build:  https://github.com/$SLUG/actions"
  echo "Draft release:    https://github.com/$SLUG/releases"
fi
