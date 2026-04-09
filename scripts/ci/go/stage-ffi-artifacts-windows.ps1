#!/usr/bin/env pwsh
# Usage: stage-ffi-artifacts-windows.ps1 [StagingDir]
# Example: stage-ffi-artifacts-windows.ps1 "artifact-staging/kreuzcrawl-ffi"
#
# Stages FFI artifacts (Windows MinGW) for packaging into distribution tarball.
# Copies compiled DLLs, import libraries, headers, and pkg-config files.

param(
    [string]$StagingDir = "artifact-staging/kreuzcrawl-ffi"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$TargetDir = "target\x86_64-pc-windows-gnu\release"

Write-Host "=== Staging FFI artifacts to $StagingDir ==="

# Stage static library (.a) - required for Go static linking
$StaticLib = "$TargetDir\libkreuzcrawl_ffi.a"
if (Test-Path $StaticLib) {
    $StaticLibSize = (Get-Item $StaticLib).Length / 1MB
    Copy-Item $StaticLib "$StagingDir\lib\"
    Write-Host "✓ Staged static library: libkreuzcrawl_ffi.a ($([math]::Round($StaticLibSize, 1))MB)"
} else {
    Write-Error "ERROR: Static library not found: $StaticLib"
    exit 1
}

# Stage dynamic library (.dll) - optional for runtime linking
if (Test-Path "$TargetDir\kreuzcrawl_ffi.dll") {
    Copy-Item "$TargetDir\kreuzcrawl_ffi.dll" "$StagingDir\lib\"
    Write-Host "✓ Staged FFI library: kreuzcrawl_ffi.dll"
}

# Copy import libraries (for dynamic linking)
$ImportLibs = @(Get-ChildItem "$TargetDir\*.dll.a" -ErrorAction SilentlyContinue)
if ($ImportLibs.Count -gt 0) {
    Copy-Item "$TargetDir\*.dll.a" "$StagingDir\lib\"
    Write-Host "✓ Staged import libraries: $($ImportLibs.Count) files"
}

# Copy PDFium (optional, bundled during build)
if (Test-Path "$TargetDir\pdfium.dll") {
    Copy-Item "$TargetDir\pdfium.dll" "$StagingDir\lib\"
    Write-Host "✓ Staged PDFium library"
}

# Copy header
Copy-Item "crates\kreuzcrawl-ffi\kreuzcrawl.h" "$StagingDir\include\"

# Copy pkg-config file
Copy-Item "crates\kreuzcrawl-ffi\kreuzcrawl-ffi-install.pc" "$StagingDir\share\pkgconfig\kreuzcrawl-ffi.pc"

Write-Host "✓ FFI artifacts staged successfully"
