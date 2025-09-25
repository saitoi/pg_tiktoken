#!/bin/bash
set -e

echo "Installing pg dependencies..."

sudo apt-get install build-essential libreadline-dev zlib1g-dev flex bison \
    libxml2-dev libxslt-dev libssl-dev libxml2-utils xsltproc ccache pkg-config

echo "Installing pg_tiktoken..."

# Install rust if not present
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Install cargo-pgrx if not present
if ! command -v cargo-pgrx &> /dev/null; then
    echo "Installing cargo-pgrx..."
    cargo install --locked cargo-pgrx
fi

# Initialize pgrx if needed
if [ ! -f "$HOME/.pgrx/config.toml" ]; then
    echo "Initializing pgrx..."
    cargo pgrx init
fi

# Build and install
echo "Building and installing extension..."
cargo pgrx install

echo "Done! Now run 'CREATE EXTENSION pg_tiktoken;' in your database."
