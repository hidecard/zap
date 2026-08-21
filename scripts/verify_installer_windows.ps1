param(
  [Parameter(Mandatory = $true)][string]$Archive,
  [Parameter(Mandatory = $true)][string]$ExpectedVersion
)

$ErrorActionPreference = 'Stop'

$work = Join-Path $env:RUNNER_TEMP ("zap-installer-" + [Guid]::NewGuid().ToString('N'))
$home = Join-Path $work 'home'
$destination = Join-Path $work 'archive'
New-Item -ItemType Directory -Force -Path $home, $destination | Out-Null

try {
  Expand-Archive -LiteralPath $Archive -DestinationPath $destination -Force
  $package = Join-Path $destination 'zap'
  $source = Join-Path $package 'bin\zap.exe'
  if (-not (Test-Path $source)) { throw 'missing executable in archive' }

  $env:USERPROFILE = $home
  $env:Path = "$(Join-Path $home '.zap\bin');$env:Path"
  $installer = Join-Path $package 'install_windows.bat'
  $uninstaller = Join-Path $package 'uninstall_windows.bat'

  cmd.exe /d /c $installer
  if ($LASTEXITCODE -ne 0) { throw "installer failed with exit code $LASTEXITCODE" }

  $installed = Join-Path $home '.zap\bin\zap.exe'
  if (-not (Test-Path $installed)) { throw 'installer did not create zap.exe' }
  $version = (& $installed --version | Out-String).Trim()
  if ($version -notmatch [regex]::Escape($ExpectedVersion)) { throw "installed version mismatch: $version" }

  # Reinstalling the same package is the upgrade contract.
  cmd.exe /d /c $installer
  if ($LASTEXITCODE -ne 0) { throw "upgrade failed with exit code $LASTEXITCODE" }
  if (-not (Test-Path $installed)) { throw 'upgrade removed zap.exe' }

  cmd.exe /d /c $uninstaller
  if ($LASTEXITCODE -ne 0) { throw "uninstaller failed with exit code $LASTEXITCODE" }
  if (Test-Path $installed) { throw 'uninstaller left zap.exe behind' }

  Write-Host 'Windows installer verification passed: install, version, upgrade, uninstall'
}
finally {
  if (Test-Path $work) { Remove-Item -Recurse -Force $work }
}
