# Define variables
$target = "wasm32-unknown-unknown"
$baseBuildDir = "builds"
$targetOutDir = Join-Path $baseBuildDir "web" "out"
$assetsDir = "assets"
$binaryName = "ef-game" # Replace with your actual binary name, without extension

# Install the wasm target
Write-Host "Installing target: $target"
rustup target add $target

# Ensure the base build directory exists
if (!(Test-Path $baseBuildDir)) {
    New-Item -ItemType Directory -Path $baseBuildDir
}

# Ensure the target-specific directory exists
if (Test-Path $targetOutDir) {
    Remove-Item -Recurse -Force $targetOutDir
}
New-Item -ItemType Directory -Path $targetOutDir

# Build the project
Write-Host "Building for target: $target"
cargo build --release --target $target

# Run wasm-bindgen to generate JS bindings
Write-Host "Running wasm-bindgen"
wasm-bindgen --out-dir $targetOutDir --target web ".\target\$target\release\$binaryName.wasm"

# Copy assets to the target-specific directory
if (Test-Path $assetsDir) {
    Copy-Item -Recurse -Force $assetsDir (Join-Path $targetOutDir "assets")
}

Write-Host "Build for target $target completed and copied to $targetOutDir"
