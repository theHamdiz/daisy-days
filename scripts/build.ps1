# Daisy Days - Build Script for Windows
# Usage: .\scripts\build.ps1 [command]
# Commands: build, server, wasm, package, clean, install, help

param(
    [Parameter(Position=0)]
    [ValidateSet("build", "server", "wasm", "package", "clean", "install", "help")]
    [string]$Command = "build"
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir
Set-Location $RootDir

function Write-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Blue }
function Write-Success { param($msg) Write-Host "[OK] $msg" -ForegroundColor Green }
function Write-Warn { param($msg) Write-Host "[WARN] $msg" -ForegroundColor Yellow }
function Write-Err { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red }

function Test-WasmTarget {
    $targets = rustup target list --installed 2>&1
    if ($targets -notmatch "wasm32-wasip1") {
        Write-Warn "wasm32-wasip1 target not installed. Installing..."
        rustup target add wasm32-wasip1
        if ($LASTEXITCODE -ne 0) {
            Write-Err "Failed to install wasm32-wasip1 target"
            exit 1
        }
    }
}

function Build-Server {
    Write-Info "Building MCP server (release)..."
    cargo build --release -p daisy_days_mcp
    if ($LASTEXITCODE -ne 0) {
        Write-Err "Failed to build MCP server"
        exit 1
    }
    Write-Success "MCP server built: target\release\daisy_days.exe"
}

function Build-Wasm {
    Write-Info "Building WASM extension..."
    Test-WasmTarget
    $env:DAISY_SKIP_SERVER = "1"
    cargo build --release --target wasm32-wasip1
    Remove-Item Env:\DAISY_SKIP_SERVER -ErrorAction SilentlyContinue
    if ($LASTEXITCODE -ne 0) {
        Write-Err "Failed to build WASM extension"
        exit 1
    }
    Write-Success "WASM extension built: target\wasm32-wasip1\release\daisy_days_extension.wasm"
}

function Build-All {
    Write-Info "Building Daisy Days extension..."
    Build-Server
    Build-Wasm
    Write-Success "Build complete!"
}

function New-Package {
    Write-Info "Packaging extension..."
    Build-All

    $DistDir = Join-Path $RootDir "target\dist"
    New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

    if (Test-Path "target\release\daisy_days.exe") {
        Copy-Item "target\release\daisy_days.exe" "$DistDir\" -Force
    }

    if (Test-Path "target\wasm32-wasip1\release\daisy_days_extension.wasm") {
        Copy-Item "target\wasm32-wasip1\release\daisy_days_extension.wasm" "$DistDir\extension.wasm" -Force
    }

    Copy-Item "extension.toml" "$DistDir\" -Force
    if (Test-Path "LICENSE") { Copy-Item "LICENSE" "$DistDir\" -Force }
    if (Test-Path "README.md") { Copy-Item "README.md" "$DistDir\" -Force }

    Write-Success "Package ready in target\dist\"
    Get-ChildItem $DistDir
}

function Clear-Build {
    Write-Info "Cleaning build artifacts..."
    cargo clean
    Write-Success "Clean complete"
}

function Install-Local {
    Write-Info "Installing to local Zed extensions..."
    Build-All

    $ZedExtDir = "$env:APPDATA\Zed\extensions\daisy-days"
    New-Item -ItemType Directory -Force -Path $ZedExtDir | Out-Null

    if (Test-Path "target\wasm32-wasip1\release\daisy_days_extension.wasm") {
        Copy-Item "target\wasm32-wasip1\release\daisy_days_extension.wasm" "$ZedExtDir\extension.wasm" -Force
    }

    if (Test-Path "target\release\daisy_days.exe") {
        Copy-Item "target\release\daisy_days.exe" "$ZedExtDir\" -Force
    }

    Copy-Item "extension.toml" "$ZedExtDir\" -Force

    Write-Success "Installed to $ZedExtDir"
}

function Show-Help {
    Write-Host ""
    Write-Host "Daisy Days - Build Script for Windows"
    Write-Host ""
    Write-Host "Usage: .\scripts\build.ps1 [command]"
    Write-Host ""
    Write-Host "Commands:"
    Write-Host "  build     Build both MCP server and WASM extension (default)"
    Write-Host "  server    Build MCP server only"
    Write-Host "  wasm      Build WASM extension only"
    Write-Host "  package   Build and package for distribution"
    Write-Host "  install   Build and install to local Zed extensions"
    Write-Host "  clean     Clean build artifacts"
    Write-Host "  help      Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\scripts\build.ps1                # Build everything"
    Write-Host "  .\scripts\build.ps1 server         # Build MCP server only"
    Write-Host "  .\scripts\build.ps1 wasm           # Build WASM extension only"
    Write-Host "  .\scripts\build.ps1 package        # Create distribution package"
    Write-Host "  .\scripts\build.ps1 install        # Install to local Zed"
    Write-Host ""
}

switch ($Command) {
    "build"   { Build-All }
    "server"  { Build-Server }
    "wasm"    { Build-Wasm }
    "package" { New-Package }
    "install" { Install-Local }
    "clean"   { Clear-Build }
    "help"    { Show-Help }
    default   {
        Write-Err "Unknown command: $Command"
        Show-Help
        exit 1
    }
}
