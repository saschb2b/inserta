#!/bin/bash
set -e

APP_NAME="inserta"
BUILD_DIR="steam_deck_build"

# Check for Docker flag
if [ "$1" == "--docker" ]; then
    echo "🐳 Building in Steam Runtime Docker container..."
    
    # Build the builder image
    docker build -t inserta-builder -f Dockerfile .
    
    # Run the build inside the container
    # We map the current directory to /app
    # We run as the host user to avoid permission issues with generated files
    # We redirect CARGO_HOME and HOME to directories inside target/ so the user has write access
    
    mkdir -p target/cargo
    mkdir -p target/home
    
    docker run --rm \
        -v "$(pwd):/app" \
        -u "$(id -u):$(id -g)" \
        -e "HOME=/app/target/home" \
        -e "CARGO_HOME=/app/target/cargo" \
        inserta-builder \
        cargo build --release
        
    echo "🐳 Docker build complete. Proceeding to package..."
else
    echo "🦀 Building $APP_NAME for Linux (Native)..."
    cargo build --release
fi

echo "📂 Creating build directory..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

echo "📋 Copying binary..."
cp "target/release/$APP_NAME" "$BUILD_DIR/"

echo "🎨 Copying assets..."
cp -r assets "$BUILD_DIR/"

echo "📜 Creating launch script..."
cat << EOF > "$BUILD_DIR/run.sh"
#!/bin/bash
# Ensure we are in the script's directory so assets are found
cd "\$(dirname "\$0")"
./$APP_NAME "\$@"
EOF
chmod +x "$BUILD_DIR/run.sh"

echo "📦 Compressing build..."
tar -czf "${APP_NAME}_steam_deck.tar.gz" "$BUILD_DIR"

echo "✅ Build complete!"
echo "Find your build at: ${APP_NAME}_steam_deck.tar.gz"
echo ""
echo "To install on Steam Deck:"
echo "1. Transfer ${APP_NAME}_steam_deck.tar.gz to your Deck (e.g. via Warpinator or ssh)"
echo "2. Extract: tar -xzf ${APP_NAME}_steam_deck.tar.gz"
echo "3. Add '$BUILD_DIR/run.sh' as a Non-Steam Game in Steam"
echo "4. Force compatibility tool if needed (usually native Linux works fine)"
