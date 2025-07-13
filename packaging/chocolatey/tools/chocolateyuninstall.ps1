$ErrorActionPreference = 'Stop'

$packageName = 'bbcpr'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Remove from PATH
$installPath = Join-Path $toolsDir "bbcpr.exe"
Uninstall-ChocolateyPath $toolsDir 'User'

Write-Host "$packageName has been uninstalled." -ForegroundColor Green