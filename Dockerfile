# Use the official Steam Runtime "Sniper" SDK
# This ensures binaries are linked against libraries compatible with SteamOS 3+
FROM registry.gitlab.steamos.cloud/steamrt/sniper/sdk:latest

# Install dependencies required by Bevy
RUN apt-get update && apt-get install -y --no-install-recommends \
    g++ \
    pkg-config \
    libx11-dev \
    libasound2-dev \
    libudev-dev \
    libxkbcommon-x11-0 \
    libwayland-dev \
    libxkbcommon-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install Rust stable
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal

# Create a directory for the project
WORKDIR /app

# Define the build command
CMD ["cargo", "build", "--release"]
