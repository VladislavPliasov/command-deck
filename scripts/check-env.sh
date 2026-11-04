#!/usr/bin/env bash
# Command Deck - P0-01 environment check.
#
# READ-ONLY: detects required development tools and libraries.
# It installs nothing, never uses sudo, never modifies PATH or shell
# configuration, and must be run as a normal (non-root) user.

set -u

FAIL=0

pass() { printf '[PASS] %s\n' "$*"; }
fail() { printf '[FAIL] %s\n' "$*"; FAIL=1; }
note() { printf '       %s\n' "$*"; }

if [ "$(id -u)" -eq 0 ]; then
  fail "Running as root is not allowed. Run this script as a normal user."
  exit 1
fi

echo "== Command Deck environment check (read-only) =="

echo "-- Required commands --"
for cmd in rustc cargo node npm curl wget file pkg-config cc; do
  if command -v "$cmd" >/dev/null 2>&1; then
    pass "$cmd ($(command -v "$cmd"))"
  else
    fail "$cmd not found in PATH"
  fi
done

echo "-- Versions --"
if command -v rustc >/dev/null 2>&1; then
  note "rustc: $(rustc --version 2>/dev/null)"
  note "cargo: $(cargo --version 2>/dev/null)"
fi
if command -v node >/dev/null 2>&1; then
  note "node: $(node -v 2>/dev/null)"
  note "npm: $(npm -v 2>/dev/null)"
fi

echo "-- Required libraries (pkg-config) --"
if command -v pkg-config >/dev/null 2>&1; then
  for mod in webkit2gtk-4.1 javascriptcoregtk-4.1 gtk+-3.0 libsoup-3.0; do
    if pkg-config --exists "$mod" 2>/dev/null; then
      pass "$mod $(pkg-config --modversion "$mod" 2>/dev/null)"
    else
      fail "$mod not found (development package missing?)"
    fi
  done
  if pkg-config --exists libayatana-appindicator3-0.1 2>/dev/null; then
    pass "libayatana-appindicator3-0.1 $(pkg-config --modversion libayatana-appindicator3-0.1 2>/dev/null)"
  elif pkg-config --exists ayatana-appindicator3-0.1 2>/dev/null; then
    pass "ayatana-appindicator3-0.1 $(pkg-config --modversion ayatana-appindicator3-0.1 2>/dev/null)"
  else
    fail "no appindicator pkg-config module found (expected libayatana-appindicator3-0.1)"
  fi
fi

echo "-- Debian packages (dpkg -s) --"
if command -v dpkg >/dev/null 2>&1; then
  for pkg in libwebkit2gtk-4.1-dev build-essential libxdo-dev libssl-dev \
             libayatana-appindicator3-dev librsvg2-dev; do
    status="$(dpkg -s "$pkg" 2>/dev/null | awk '/^Status:/{print $2" "$3" "$4}')"
    if [ "${status:-}" = "install ok installed" ]; then
      pass "$pkg $(dpkg -s "$pkg" 2>/dev/null | awk '/^Version:/{print $2}')"
    else
      fail "$pkg not installed"
    fi
  done
else
  fail "dpkg not found (this check targets Debian/Kali)"
fi

echo
if [ "$FAIL" -ne 0 ]; then
  echo "RESULT: FAIL - missing requirements are listed above."
  echo "This script never installs anything. Install missing packages"
  echo "manually after reviewing them (see the P0-01 plan, section 5)."
  exit 1
fi

echo "RESULT: PASS - all requirements detected."
exit 0
