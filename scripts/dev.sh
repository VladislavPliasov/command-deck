#!/usr/bin/env bash
# Command Deck - start development mode (integrated Tauri desktop app).
#
# Runs the read-only environment check, verifies project dependencies are
# installed, then launches `npm run tauri dev`. It never installs packages,
# never uses sudo, and must be run as a normal (non-root) user.

set -euo pipefail

cd "$(dirname "$0")/.."

if [ "$(id -u)" -eq 0 ]; then
  echo "ERROR: run as a normal user, not root." >&2
  exit 1
fi

./scripts/check-env.sh || {
  echo "ERROR: environment check failed (see output above)." >&2
  exit 1
}

if [ ! -d node_modules ]; then
  echo "ERROR: node_modules is missing. Run: npm ci" >&2
  exit 1
fi

exec npm run tauri dev
