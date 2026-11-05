$ErrorActionPreference = 'Stop'

$tag = '__TAG__'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

$packageArgs = @{
  packageName    = 'c2patool'
  unzipLocation  = $toolsDir
  fileType       = 'zip'
  url64bit       = "https://github.com/contentauth/c2patool/releases/download/${tag}/${tag}-x86_64-pc-windows-msvc.zip"
  checksum64     = '__CHECKSUM__'
  checksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @packageArgs
