#!/bin/bash
set -e

# Ensure the target is installed
rustup target add wasm32-unknown-unknown

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "wasm-bindgen-cli could not be found. Installing..."
    cargo install wasm-bindgen-cli
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
