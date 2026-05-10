[CmdletBinding()]
param(
    [ValidateSet("auto", "windows", "msi", "all", "help")]
    [string]$Action = "auto"
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptDir "..")
Set-Location $ProjectRoot

$AppPackage = if ($env:APP_PACKAGE) { $env:APP_PACKAGE } else { "webcam-detector" }
$AppDisplayName = if ($env:APP_DISPLAY_NAME) { $env:APP_DISPLAY_NAME } else { "Webcam Detector" }
$Manufacturer = if ($env:MANUFACTURER) { $env:MANUFACTURER } else { "Webcam Detector Maintainers" }
$DistDir = Join-Path $ProjectRoot "dist"
$WorkDir = Join-Path $DistDir "release"
$UpgradeCode = if ($env:UPGRADE_CODE) { $env:UPGRADE_CODE } else { "7D4D720A-8A76-4B87-BFD0-2D2C77D3C5E1" }

function Show-Usage {
    Write-Host @"
Usage: scripts/release.ps1 [auto|windows|msi|all|help]

Targets:
  auto     Build the Windows .msi installer on Windows.
  windows  Build the Windows .msi installer.
  msi      Build the Windows .msi installer.
  all      Build every installer supported by this script.

Environment:
  APP_PACKAGE       Cargo package to bundle. Default: $AppPackage
  APP_DISPLAY_NAME  Human-readable app name. Default: $AppDisplayName
  MANUFACTURER      MSI manufacturer. Default: $Manufacturer
  UPGRADE_CODE      Stable MSI upgrade GUID. Default: $UpgradeCode

Requires WiX Toolset. Supports WiX v4 'wix' or WiX v3 'candle'/'light'.
"@
}

function Require-Command {
    param([string]$Name)

    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command not found: $Name"
    }
}

function Get-PackageVersion {
    $pkgId = cargo pkgid -p $AppPackage
    return ($pkgId -split "#")[-1]
}

function Invoke-ReleaseBuild {
    cargo build --release -p $AppPackage
}

function Get-ReleaseExecutable {
    $exeName = if ($IsWindows -or $env:OS -eq "Windows_NT") { "$AppPackage.exe" } else { $AppPackage }
    return Join-Path "target/release" $exeName
}

function Convert-ToWixId {
    param([string]$Value)

    $id = ($Value -replace "[^A-Za-z0-9_]", "_")
    if ($id -notmatch "^[A-Za-z_]") {
        $id = "_$id"
    }
    return $id
}

function New-Wix4Source {
    param(
        [string]$Path,
        [string]$Version,
        [string]$ExecutablePath
    )

    $componentId = Convert-ToWixId "$AppPackage.Executable"
    $fileId = Convert-ToWixId "$AppPackage.File"
    $installDir = Convert-ToWixId "$AppPackage.InstallDir"

    @"
<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">
  <Package Name="$AppDisplayName" Manufacturer="$Manufacturer" Version="$Version" UpgradeCode="$UpgradeCode" Scope="perMachine">
    <MajorUpgrade DowngradeErrorMessage="A newer version of $AppDisplayName is already installed." />
    <MediaTemplate EmbedCab="yes" />
    <Feature Id="MainFeature" Title="$AppDisplayName" Level="1">
      <ComponentGroupRef Id="ProductComponents" />
    </Feature>
  </Package>

  <Fragment>
    <StandardDirectory Id="ProgramFilesFolder">
      <Directory Id="$installDir" Name="$AppDisplayName" />
    </StandardDirectory>
  </Fragment>

  <Fragment>
    <ComponentGroup Id="ProductComponents" Directory="$installDir">
      <Component Id="$componentId" Guid="*">
        <File Id="$fileId" Source="$ExecutablePath" KeyPath="yes" />
      </Component>
    </ComponentGroup>
  </Fragment>
</Wix>
"@ | Set-Content -Path $Path -Encoding UTF8
}

function New-Wix3Source {
    param(
        [string]$Path,
        [string]$Version,
        [string]$ExecutablePath
    )

    $componentId = Convert-ToWixId "$AppPackage.Executable"
    $fileId = Convert-ToWixId "$AppPackage.File"
    $installDir = Convert-ToWixId "$AppPackage.InstallDir"

    @"
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
  <Product Id="*" Name="$AppDisplayName" Manufacturer="$Manufacturer" Version="$Version" UpgradeCode="$UpgradeCode" Language="1033">
    <Package InstallerVersion="500" Compressed="yes" InstallScope="perMachine" />
    <MajorUpgrade DowngradeErrorMessage="A newer version of $AppDisplayName is already installed." />
    <MediaTemplate EmbedCab="yes" />
    <Feature Id="MainFeature" Title="$AppDisplayName" Level="1">
      <ComponentGroupRef Id="ProductComponents" />
    </Feature>
  </Product>

  <Fragment>
    <Directory Id="TARGETDIR" Name="SourceDir">
      <Directory Id="ProgramFilesFolder">
        <Directory Id="$installDir" Name="$AppDisplayName" />
      </Directory>
    </Directory>
  </Fragment>

  <Fragment>
    <ComponentGroup Id="ProductComponents" Directory="$installDir">
      <Component Id="$componentId" Guid="*">
        <File Id="$fileId" Source="$ExecutablePath" KeyPath="yes" />
      </Component>
    </ComponentGroup>
  </Fragment>
</Wix>
"@ | Set-Content -Path $Path -Encoding UTF8
}

function New-Msi {
    if (-not ($IsWindows -or $env:OS -eq "Windows_NT")) {
        throw ".msi packaging must run on Windows."
    }

    Invoke-ReleaseBuild

    $version = Get-PackageVersion
    $msiWorkDir = Join-Path $WorkDir "msi"
    $wxsPath = Join-Path $msiWorkDir "$AppPackage.wxs"
    $outputPath = Join-Path $DistDir "$AppPackage-$version-windows-x64.msi"
    $exePath = Resolve-Path (Get-ReleaseExecutable)

    New-Item -ItemType Directory -Force -Path $msiWorkDir | Out-Null
    New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

    if (Get-Command wix -ErrorAction SilentlyContinue) {
        New-Wix4Source -Path $wxsPath -Version $version -ExecutablePath $exePath
        wix build $wxsPath -o $outputPath
    } elseif ((Get-Command candle -ErrorAction SilentlyContinue) -and (Get-Command light -ErrorAction SilentlyContinue)) {
        New-Wix3Source -Path $wxsPath -Version $version -ExecutablePath $exePath
        $wixObj = Join-Path $msiWorkDir "$AppPackage.wixobj"
        candle -out $wixObj $wxsPath
        light -out $outputPath $wixObj
    } else {
        throw "WiX Toolset was not found. Install WiX v4 'wix' or WiX v3 'candle'/'light'."
    }

    Write-Host "Created $outputPath"
}

switch ($Action) {
    "auto" {
        New-Msi
    }
    "windows" {
        New-Msi
    }
    "msi" {
        New-Msi
    }
    "all" {
        New-Msi
    }
    "help" {
        Show-Usage
    }
}
