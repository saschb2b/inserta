$ErrorActionPreference = "Stop"

$AppName = "inserta"
$BuildDir = "inserta_windows"

Write-Host "🦀 Building $AppName for Windows..."
cargo build --release

Write-Host "📂 Creating build directory..."
if (Test-Path $BuildDir) { Remove-Item -Recurse -Force $BuildDir }
New-Item -ItemType Directory -Force -Path $BuildDir | Out-Null

Write-Host "📋 Copying binary..."
Copy-Item "target\release\$AppName.exe" "$BuildDir\"

Write-Host "🎨 Copying assets..."
Copy-Item -Recurse "assets" "$BuildDir\"

Write-Host "📦 Compressing build..."
$ZipFile = "${AppName}_windows.zip"
Compress-Archive -Path "$BuildDir\*" -DestinationPath $ZipFile -Force

Write-Host "✅ Build complete: $ZipFile"
