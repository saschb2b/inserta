#!/bin/bash
set -e

APP_NAME="inserta"
BUILD_DIR="steam_deck_build"

echo "🦀 Building $APP_NAME for Linux (Steam Deck compatible)..."
cargo build --release

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
