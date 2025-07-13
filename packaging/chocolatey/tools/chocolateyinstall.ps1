$ErrorActionPreference = 'Stop'

$toolsDir   = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$packageName = 'bbcpr'
$url64      = 'https://github.com/88plug/bbcpr/releases/download/v0.1.0/bbcpr-v0.1.0-x86_64-windows.zip'
$checksum64 = 'PLACEHOLDER_SHA256'

$packageArgs = @{
  packageName   = $packageName
  unzipLocation = $toolsDir
  url64bit      = $url64
  checksum64    = $checksum64
  checksumType64= 'sha256'
}

Install-ChocolateyZipPackage @packageArgs

# Add to PATH
$installPath = Join-Path $toolsDir "bbcpr.exe"
Install-ChocolateyPath $toolsDir 'User'

Write-Host "$packageName has been installed successfully!" -ForegroundColor Green
Write-Host "You can now use 'bbcpr' from any command prompt or PowerShell window." -ForegroundColor Green