#!/bin/bash
set -e

# Ensure the target is installed
rustup target add wasm32-unknown-unknown

# Get wasm-bindgen version from Cargo.lock to ensure CLI matches
WASM_BINDGEN_VERSION=$(grep -A1 'name = "wasm-bindgen"' Cargo.lock | grep version | head -1 | sed 's/.*"\(.*\)"/\1/')
echo "Detected wasm-bindgen version in Cargo.lock: $WASM_BINDGEN_VERSION"

# Check if wasm-bindgen-cli is installed with correct version
INSTALLED_VERSION=$(wasm-bindgen --version 2>/dev/null | sed 's/wasm-bindgen //' || echo "none")
if [ "$INSTALLED_VERSION" != "$WASM_BINDGEN_VERSION" ]; then
    echo "Installing wasm-bindgen-cli version $WASM_BINDGEN_VERSION (currently: $INSTALLED_VERSION)..."
    cargo install wasm-bindgen-cli --version "$WASM_BINDGEN_VERSION"
fi

echo "Building for WASM..."
cargo build --release --target wasm32-unknown-unknown

echo "Generating bindings..."
# Create output directory if it doesn't exist
mkdir -p ./webbuild/out

# Run wasm-bindgen
# Note: The input file name depends on the crate name in Cargo.toml
wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/inserta.wasm

echo "Copying assets..."
cp -r assets ./webbuild/

echo "Creating zip artifact..."
# Remove existing zip if it exists
rm -f webbuild.zip
# Zip the contents of webbuild
# We cd into webbuild so the zip doesn't contain the 'webbuild' folder itself, but its contents
cd webbuild && zip -r ../webbuild.zip . && cd ..

echo "Done! Artifact created at webbuild.zip"
echo "To test locally, run: python3 -m http.server --directory webbuild"
