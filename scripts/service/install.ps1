# OriginLM background service installer for Windows.
# No admin rights: one binary under LOCALAPPDATA and one logon task.
#
#   irm https://<site>/install.ps1 | iex
param(
  [string]$ReleaseBase = 'https://<site>/releases/latest/download',
  [string]$InstallDir = (Join-Path $env:LOCALAPPDATA 'OriginLM\bin'),
  [string]$StateDir = (Join-Path $env:USERPROFILE '.local-ai-kit')
)
$ErrorActionPreference = 'Stop'

$target = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
  'X64' { 'x86_64-pc-windows-msvc' }
  'Arm64' { 'aarch64-pc-windows-msvc' }
  default { throw "Unsupported architecture: $($_)" }
}
$archive = "local-ai-kit-$target.zip"
$temp = Join-Path ([System.IO.Path]::GetTempPath()) ("originlm-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $temp | Out-Null
try {
  Write-Host "Downloading $archive"
  Invoke-WebRequest -UseBasicParsing -Uri "$ReleaseBase/$archive" -OutFile (Join-Path $temp $archive)
  Invoke-WebRequest -UseBasicParsing -Uri "$ReleaseBase/SHA256SUMS" -OutFile (Join-Path $temp 'SHA256SUMS')
  $expected = (Get-Content (Join-Path $temp 'SHA256SUMS') | Where-Object { $_ -match [regex]::Escape($archive) } | Select-Object -First 1).Split(' ')[0]
  $actual = (Get-FileHash -Algorithm SHA256 (Join-Path $temp $archive)).Hash.ToLowerInvariant()
  if (-not $expected -or $expected.ToLowerInvariant() -ne $actual) { throw 'Checksum mismatch' }

  Expand-Archive -Path (Join-Path $temp $archive) -DestinationPath $temp
  New-Item -ItemType Directory -Force -Path $InstallDir, $StateDir | Out-Null
  $binary = Get-ChildItem -Path $temp -Recurse -Filter 'local-ai-kit.exe' | Select-Object -First 1
  Copy-Item $binary.FullName (Join-Path $InstallDir 'local-ai-kit.exe') -Force

  $token = Join-Path $StateDir 'broker-token.json'
  $socket = 'local-ai-kit-user'
  $arguments = "serve --ipc $socket --independent --token-file `"$token`" --allow-passive-discovery"
  $action = New-ScheduledTaskAction -Execute (Join-Path $InstallDir 'local-ai-kit.exe') -Argument $arguments
  $trigger = New-ScheduledTaskTrigger -AtLogOn
  $settings = New-ScheduledTaskSettingsSet -RestartCount 3 -RestartInterval (New-TimeSpan -Minutes 1) -AllowStartIfOnBatteries
  Register-ScheduledTask -TaskName 'OriginLM Helper' -Action $action -Trigger $trigger -Settings $settings -Force | Out-Null
  Start-ScheduledTask -TaskName 'OriginLM Helper'

  Write-Host "Installed. Task: OriginLM Helper (Task Scheduler logon task)"
  Write-Host "Uninstall: irm https://<site>/uninstall.ps1 | iex"
} finally {
  Remove-Item -Recurse -Force $temp -ErrorAction SilentlyContinue
}
