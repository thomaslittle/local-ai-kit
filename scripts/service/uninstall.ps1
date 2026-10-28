# Remove the OriginLM helper task, tray autostart, Native Messaging manifests and
# the binary. User data is kept unless -Purge is passed.
param(
  [string]$InstallDir = (Join-Path $env:LOCALAPPDATA 'OriginLM\bin'),
  [string]$StateDir = (Join-Path $env:USERPROFILE '.local-ai-kit'),
  [switch]$Purge
)
$ErrorActionPreference = 'Stop'
Stop-ScheduledTask -TaskName 'OriginLM Helper' -ErrorAction SilentlyContinue
Unregister-ScheduledTask -TaskName 'OriginLM Helper' -Confirm:$false -ErrorAction SilentlyContinue
Remove-Item -Force (Join-Path ([Environment]::GetFolderPath('Startup')) 'OriginLM Tray.lnk') -ErrorAction SilentlyContinue

$manifestRoots = @(
  (Join-Path $env:LOCALAPPDATA 'Google\Chrome\User Data\NativeMessagingHosts'),
  (Join-Path $env:LOCALAPPDATA 'Chromium\User Data\NativeMessagingHosts'),
  (Join-Path $env:LOCALAPPDATA 'Microsoft\Edge\User Data\NativeMessagingHosts'),
  (Join-Path $env:LOCALAPPDATA 'BraveSoftware\Brave-Browser\User Data\NativeMessagingHosts')
)
foreach ($root in $manifestRoots) {
  Remove-Item -Force (Join-Path $root 'dev.originlm.helper.json') -ErrorAction SilentlyContinue
}

Remove-Item -Force (Join-Path $InstallDir 'local-ai-kit.exe') -ErrorAction SilentlyContinue
if ($Purge) {
  Remove-Item -Recurse -Force $StateDir -ErrorAction SilentlyContinue
  Remove-Item -Recurse -Force (Join-Path $env:LOCALAPPDATA 'OriginLM') -ErrorAction SilentlyContinue
  Write-Host 'Removed the binary, task and user data'
} else {
  Remove-Item -Force (Join-Path $StateDir 'broker-token.json') -ErrorAction SilentlyContinue
  Write-Host "Removed the binary and task; user data kept in $StateDir (use -Purge to delete)"
}
