#!/bin/sh
# Remove the OriginLM helper service, tray autostart, browser Native Messaging
# manifests and the binary. User data is kept unless --purge is passed.
#
#   curl -fsSL https://<site>/uninstall.sh | sh
#   curl -fsSL https://<site>/uninstall.sh | sh -s -- --purge
set -eu

PURGE=0
for arg in "$@"; do
  case "$arg" in
    --purge) PURGE=1 ;;
    *) printf 'error: unknown option %s\n' "$arg" >&2; exit 2 ;;
  esac
done

BIN_DIR="${LAI_BIN_DIR:-$HOME/.local/bin}"
STATE_DIR="${LAI_STATE_DIR:-$HOME/.local-ai-kit}"
DATA_DIR="${LAI_DATA_DIR:-$HOME/.local/state/local-ai-kit}"

say() { printf '%s\n' "$*"; }

if [ "$(uname -s)" = Darwin ]; then
  plist="$HOME/Library/LaunchAgents/dev.originlm.helper.plist"
  launchctl unload "$plist" >/dev/null 2>&1 || true
  rm -f "$plist"
  rm -f "$HOME/Library/LaunchAgents/dev.originlm.tray.plist"
  say "Removed the launchd agent"
else
  systemctl --user disable --now originlm-helper.service >/dev/null 2>&1 || true
  rm -f "${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/originlm-helper.service"
  systemctl --user daemon-reload >/dev/null 2>&1 || true
  rm -f "${XDG_CONFIG_HOME:-$HOME/.config}/autostart/originlm-tray.desktop"
  say "Removed the systemd user unit"
fi

# Native Messaging manifests for every browser we may have registered.
for path in \
  "$HOME/.config/google-chrome/NativeMessagingHosts" \
  "$HOME/.config/chromium/NativeMessagingHosts" \
  "$HOME/.config/microsoft-edge/NativeMessagingHosts" \
  "$HOME/.config/BraveSoftware/Brave-Browser/NativeMessagingHosts" \
  "$HOME/Library/Application Support/Google/Chrome/NativeMessagingHosts" \
  "$HOME/Library/Application Support/Chromium/NativeMessagingHosts" \
  "$HOME/Library/Application Support/Microsoft Edge/NativeMessagingHosts"; do
  rm -f "$path/dev.originlm.helper.json" 2>/dev/null || true
done

rm -f "$BIN_DIR/local-ai-kit"
rm -rf "$STATE_DIR/workspaces"
if [ "$PURGE" -eq 1 ]; then
  rm -rf "$STATE_DIR" "$DATA_DIR"
  say "Removed the binary, service and user data"
else
  rm -f "$STATE_DIR/broker-token.json"
  say "Removed the binary and service; user data kept in $STATE_DIR and $DATA_DIR (use --purge to delete)"
fi
