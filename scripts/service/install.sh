#!/bin/sh
# OriginLM background service installer for macOS and Linux.
# No admin rights: one binary in the user's path and one user-level service.
#
#   curl -fsSL https://<site>/install.sh | sh
#
# Environment overrides: LAI_RELEASE_BASE, LAI_BIN_DIR, LAI_STATE_DIR.
set -eu

BASE="${LAI_RELEASE_BASE:-https://<site>/releases/latest/download}"
BIN_DIR="${LAI_BIN_DIR:-$HOME/.local/bin}"
STATE_DIR="${LAI_STATE_DIR:-$HOME/.local-ai-kit}"
if [ -n "${XDG_RUNTIME_DIR:-}" ]; then
  default_socket="$XDG_RUNTIME_DIR/local-ai-kit/user.sock"
else
  default_socket="$STATE_DIR/user.sock"
fi
SOCKET="${LAI_BROKER_SOCKET:-$default_socket}"

say() { printf '%s\n' "$*"; }
fail() { printf 'error: %s\n' "$*" >&2; exit 1; }

# Newlines cannot be represented safely by these service-file templates.
for path in "$BIN_DIR" "$STATE_DIR" "$SOCKET"; do
  case "$path" in
    /*) ;;
    *) fail "service paths must be absolute" ;;
  esac
  case "$path" in
    *'
'*) fail "service paths must not contain newlines" ;;
  esac
done

case "$(uname -s)" in
  Darwin) os=apple-darwin ;;
  Linux) os=unknown-linux-gnu ;;
  *) fail "unsupported system: $(uname -s)" ;;
esac
case "$(uname -m)" in
  x86_64|amd64) arch=x86_64 ;;
  arm64|aarch64) arch=aarch64 ;;
  *) fail "unsupported architecture: $(uname -m)" ;;
esac
target="$arch-$os"
archive="local-ai-kit-$target.tar.gz"

command -v curl >/dev/null 2>&1 || fail "curl is required"
command -v sha256sum >/dev/null 2>&1 || command -v shasum >/dev/null 2>&1 || fail "sha256sum or shasum is required"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

say "Downloading $archive"
curl -fsSL "$BASE/$archive" -o "$tmp/$archive" || fail "download failed"
curl -fsSL "$BASE/SHA256SUMS" -o "$tmp/SHA256SUMS" || fail "checksums unavailable"

expected="$(awk -v name="$archive" '$2 == name {print $1}' "$tmp/SHA256SUMS")"
[ -n "$expected" ] || fail "no checksum for $archive"
if command -v sha256sum >/dev/null 2>&1; then
  actual="$(sha256sum "$tmp/$archive" | awk '{print $1}')"
else
  actual="$(shasum -a 256 "$tmp/$archive" | awk '{print $1}')"
fi
[ "$expected" = "$actual" ] || fail "checksum mismatch for $archive"

# A configured trust key must never silently downgrade to checksum-only.
if [ -n "${LAI_MINISIGN_PUBKEY:-}" ]; then
  command -v minisign >/dev/null 2>&1 || fail "minisign is required when a verification key is configured"
  curl -fsSL "$BASE/SHA256SUMS.minisig" -o "$tmp/SHA256SUMS.minisig" || fail "signature unavailable"
  minisign -Vm "$tmp/SHA256SUMS" -x "$tmp/SHA256SUMS.minisig" -P "$LAI_MINISIGN_PUBKEY" >/dev/null \
    || fail "checksum signature verification failed"
fi

# Extract only the expected member to stdout: archive paths/links cannot write
# into the filesystem. A linked/missing member produces no executable payload.
mkdir "$tmp/payload"
tar -xzOf "$tmp/$archive" "helper-$target/local-ai-kit" > "$tmp/payload/local-ai-kit" || fail "helper missing from archive"
[ -s "$tmp/payload/local-ai-kit" ] || fail "helper is empty"
mkdir -p "$BIN_DIR" "$STATE_DIR" "$(dirname "$SOCKET")"
install -m 0755 "$tmp/payload/local-ai-kit" "$BIN_DIR/local-ai-kit"

token="$STATE_DIR/broker-token.json"
# Service files have their own escaping rules; shell word splitting is not one.
xml_escape() { printf '%s' "$1" | sed 's/\&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g; s/"/\&quot;/g;'; }
unit_quote() {
  printf '"'
  printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g; s/%/%%/g; s/\$/$$/g'
  printf '"'
}

if [ "$(uname -s)" = Darwin ]; then
  plist="$HOME/Library/LaunchAgents/dev.originlm.helper.plist"
  mkdir -p "$(dirname "$plist")"
  plist_args=""
  for word in "$BIN_DIR/local-ai-kit" serve --ipc "$SOCKET" --independent --token-file "$token" --allow-passive-discovery; do
    plist_args="$plist_args    <string>$(xml_escape "$word")</string>
"
  done
  cat > "$plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
  <key>Label</key><string>dev.originlm.helper</string>
  <key>ProgramArguments</key><array>
$plist_args  </array>
  <key>RunAtLoad</key><true/>
  <key>KeepAlive</key><true/>
  <key>ProcessType</key><string>Background</string>
</dict></plist>
PLIST
  launchctl unload "$plist" >/dev/null 2>&1 || true
  launchctl load -w "$plist" || fail "launchd registration failed"
  say "Installed. Service: dev.originlm.helper (launchd LaunchAgent)"
else
  unit_dir="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
  mkdir -p "$unit_dir"
  unit="$unit_dir/originlm-helper.service"
  cat > "$unit" <<UNIT
[Unit]
Description=OriginLM local AI helper
After=default.target

[Service]
ExecStart=$(unit_quote "$BIN_DIR/local-ai-kit") serve --ipc $(unit_quote "$SOCKET") --independent --token-file $(unit_quote "$token") --allow-passive-discovery
Restart=on-failure
RestartSec=2
Nice=5

[Install]
WantedBy=default.target
UNIT
  systemctl --user daemon-reload
  systemctl --user enable --now originlm-helper.service || fail "systemd registration failed"
  say "Installed. Service: originlm-helper (systemd --user)"
fi

say "Socket: $SOCKET"
say "Uninstall: curl -fsSL https://<site>/uninstall.sh | sh"
say "Status: systemctl --user status originlm-helper   /   launchctl print gui/\$(id -u)/dev.originlm.helper"
